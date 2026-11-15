use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::shared::errors::AppError;

use super::model::{
    AdminListingRow, AdminListingVisibility, ListingDetailRow, ListingFields, ListingMediaRow,
    ListingStatus, ListingSummaryRow, ListingType,
};

/// The feed and the spec price everything in XAF; the column has no default.
const LISTING_CURRENCY: &str = "XAF";

/// A macro because `concat!` only accepts literals, and `IS_PUBLIC` is built from it.
macro_rules! has_cover_sql {
    () => {
        "EXISTS (SELECT 1 FROM listing_media c WHERE c.listing_id = l.id AND c.is_cover)"
    };
}

/// A macro so `IS_PUBLIC` can embed it with `concat!`.
macro_rules! owner_is_eligible_sql {
    () => {
        "EXISTS (SELECT 1 FROM users owner WHERE owner.id = l.owner_id \
            AND owner.is_active AND owner.role = 'owner')"
    };
}

/// True once listing `l` has a photo: the first upload is always marked cover.
const HAS_COVER: &str = has_cover_sql!();

/// True while the owner of listing `l` is active and still has the owner role.
/// Otherwise the listing leaves public view and its owner can't open it either.
const OWNER_IS_ELIGIBLE: &str = owner_is_eligible_sql!();

/// The public-visibility rule for listing `l`: published with a photo, by an eligible owner.
/// It stays self-contained on that alias because the count queries don't join `users`.
const IS_PUBLIC: &str = concat!(
    "(l.published_at IS NOT NULL AND ",
    has_cover_sql!(),
    " AND ",
    owner_is_eligible_sql!(),
    ")"
);

/// Optional, combinable filters for `GET /listings`, `GET /search` and `GET /admin/listings`,
/// built with `QueryBuilder`. `city` and `neighborhood` are expected already normalized, and
/// `keywords` trimmed and non-blank: blank keywords would match no listing.
#[derive(Debug, Default)]
pub struct ListingFilters {
    pub owner_id: Option<Uuid>,
    pub city: Option<String>,
    pub neighborhood: Option<String>,
    /// Empty means any type.
    pub listing_types: Vec<ListingType>,
    pub status: Option<ListingStatus>,
    pub keywords: Option<String>,
    pub price_min: Option<i64>,
    pub price_max: Option<i64>,
}

/// Keeps only listings that pass `IS_PUBLIC`. Shared by `count_listings` and
/// `list_listings` so `total` stays aligned with the returned pages; `pub` so
/// contact reveal can reuse it instead of re-deriving the rule.
pub fn push_public_visibility(qb: &mut QueryBuilder<Postgres>) {
    qb.push(" AND ").push(IS_PUBLIC);
}

/// Whole-value place match, ignoring case and accents. The column side must stay
/// `lower(immutable_unaccent(..))` to use `idx_listings_city_normalized`.
/// `column` is a code constant, never client input; `pub` so search can reuse it.
pub fn push_place_match(qb: &mut QueryBuilder<Postgres>, column: &'static str, value: &str) {
    qb.push(format_args!(
        " AND lower(immutable_unaccent({column})) = lower(immutable_unaccent("
    ));
    qb.push_bind(value.to_owned());
    qb.push("))");
}

/// Builds the tsquery for `keywords` with `unaccent` then the `french` configuration, as the
/// trigger that fills `search_vector` does. If the two differ, accents and stems stop matching.
fn push_keyword_query(qb: &mut QueryBuilder<Postgres>, keywords: &str) {
    qb.push("websearch_to_tsquery('french', unaccent(");
    qb.push_bind(keywords.to_owned());
    qb.push("))");
}

fn push_filters(qb: &mut QueryBuilder<Postgres>, filters: &ListingFilters) {
    if let Some(owner_id) = filters.owner_id {
        qb.push(" AND l.owner_id = ");
        qb.push_bind(owner_id);
    }
    if let Some(city) = &filters.city {
        push_place_match(qb, "l.city", city);
    }
    if let Some(neighborhood) = &filters.neighborhood {
        push_place_match(qb, "l.neighborhood", neighborhood);
    }
    if !filters.listing_types.is_empty() {
        qb.push(" AND l.type = ANY(");
        qb.push_bind(filters.listing_types.clone());
        qb.push(")");
    }
    if let Some(status) = filters.status {
        qb.push(" AND l.status = ");
        qb.push_bind(status);
    }
    if let Some(price_min) = filters.price_min {
        qb.push(" AND l.price >= ");
        qb.push_bind(price_min);
    }
    if let Some(price_max) = filters.price_max {
        qb.push(" AND l.price <= ");
        qb.push_bind(price_max);
    }
    if let Some(keywords) = &filters.keywords {
        qb.push(" AND l.search_vector @@ ");
        push_keyword_query(qb, keywords);
    }
}

/// Total number of listings matching `filters` — backs the `total`/`total_pages`
/// fields of the pagination envelope.
pub async fn count_listings(pool: &PgPool, filters: &ListingFilters) -> Result<i64, AppError> {
    let mut qb: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT COUNT(*) FROM listings l WHERE 1 = 1");
    push_public_visibility(&mut qb);
    push_filters(&mut qb, filters);

    qb.build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
}

/// One page of public listings, newest first. With keywords, relevance comes first: title
/// matches, then city, neighborhood, owner or type-label matches, then the rest.
pub async fn list_listings(
    pool: &PgPool,
    filters: &ListingFilters,
    limit: u32,
    offset: u64,
) -> Result<Vec<ListingSummaryRow>, AppError> {
    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT \
            l.id, \
            l.title, \
            l.type AS listing_type, \
            l.status, \
            l.city, \
            l.neighborhood, \
            l.price::float8 AS price, \
            lm.url AS cover_photo_url, \
            to_char(l.published_at AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') AS published_at, \
            u.id AS owner_id, \
            u.first_name AS owner_first_name, \
            u.last_name AS owner_last_name \
         FROM listings l \
         JOIN users u ON u.id = l.owner_id \
         JOIN listing_media lm ON lm.listing_id = l.id AND lm.is_cover = TRUE \
         WHERE 1 = 1",
    );
    push_public_visibility(&mut qb);
    push_filters(&mut qb, filters);
    qb.push(" ORDER BY");
    if let Some(keywords) = &filters.keywords {
        // `search_vector` weights: A is the title; B the city, neighborhood, owner and type label.
        qb.push(" CASE WHEN ts_filter(l.search_vector, '{a}') @@ ");
        push_keyword_query(&mut qb, keywords);
        qb.push(" THEN 3 WHEN ts_filter(l.search_vector, '{a,b}') @@ ");
        push_keyword_query(&mut qb, keywords);
        qb.push(" THEN 2 ELSE 1 END DESC,");
    }
    // Rows inserted in one transaction share `created_at`; `id` keeps the pages stable.
    qb.push(" l.created_at DESC, l.id DESC LIMIT ");
    qb.push_bind(limit as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);

    qb.build_query_as::<ListingSummaryRow>()
        .fetch_all(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
}

/// Total number of listings owned by `owner_id`, whatever their state.
pub async fn count_owner_listings(pool: &PgPool, owner_id: Uuid) -> Result<i64, AppError> {
    sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "count!" FROM listings WHERE owner_id = $1"#,
        owner_id
    )
    .fetch_one(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// One page of `owner_id`'s listings, newest first. No public-visibility rule:
/// drafts and listings without a photo are included.
pub async fn list_owner_listings(
    pool: &PgPool,
    owner_id: Uuid,
    limit: u32,
    offset: u64,
) -> Result<Vec<ListingSummaryRow>, AppError> {
    sqlx::query_as!(
        ListingSummaryRow,
        r#"
        SELECT
            l.id,
            l.title,
            l.type AS "listing_type: ListingType",
            l.status AS "status: ListingStatus",
            l.city,
            l.neighborhood,
            l.price::float8 AS "price!",
            lm.url AS "cover_photo_url?",
            to_char(l.published_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS published_at,
            u.id AS "owner_id!",
            u.first_name AS owner_first_name,
            u.last_name AS owner_last_name
        FROM listings l
        JOIN users u ON u.id = l.owner_id
        LEFT JOIN listing_media lm ON lm.listing_id = l.id AND lm.is_cover
        WHERE l.owner_id = $1
        -- id breaks created_at ties so OFFSET pages stay stable
        ORDER BY l.created_at DESC, l.id DESC
        LIMIT $2 OFFSET $3
        "#,
        owner_id,
        i64::from(limit),
        offset as i64,
    )
    .fetch_all(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Narrows `l` to one visibility value, matching what `admin_listing_visibility` derives.
fn push_visibility_filter(qb: &mut QueryBuilder<Postgres>, visibility: AdminListingVisibility) {
    qb.push(" AND ");
    match visibility {
        AdminListingVisibility::Draft => qb.push("l.published_at IS NULL"),
        AdminListingVisibility::NoPhoto => qb.push(format_args!(
            "l.published_at IS NOT NULL AND NOT {HAS_COVER}"
        )),
        AdminListingVisibility::Public => qb.push(IS_PUBLIC),
        AdminListingVisibility::Hidden => qb.push(format_args!(
            "l.published_at IS NOT NULL AND {HAS_COVER} AND NOT {IS_PUBLIC}"
        )),
    };
}

/// Fields the back-office search looks in. `strpos` rather than `LIKE`: `unaccent` turns
/// fullwidth `％` and `＿` into wildcards.
const ADMIN_SEARCH_FIELDS: [&str; 4] = [
    "l.title",
    "l.city",
    "concat_ws(' ', u.first_name, u.last_name)",
    "u.email",
];

fn push_admin_search(qb: &mut QueryBuilder<Postgres>, search: &str) {
    qb.push(" AND (");
    for (index, field) in ADMIN_SEARCH_FIELDS.iter().enumerate() {
        if index > 0 {
            qb.push(" OR ");
        }
        qb.push(format_args!(
            "strpos(lower(immutable_unaccent({field})), lower(immutable_unaccent("
        ));
        qb.push_bind(search.to_owned());
        qb.push("))) > 0");
    }
    qb.push(")");
}

fn push_admin_filters(
    qb: &mut QueryBuilder<Postgres>,
    filters: &ListingFilters,
    visibility: Option<AdminListingVisibility>,
    search: Option<&str>,
) {
    push_filters(qb, filters);
    if let Some(visibility) = visibility {
        push_visibility_filter(qb, visibility);
    }
    if let Some(search) = search {
        push_admin_search(qb, search);
    }
}

/// The back-office `SELECT`, up to an open `WHERE`, shared by the page and the single read.
fn admin_listing_select() -> QueryBuilder<Postgres> {
    let mut qb = QueryBuilder::new(
        "SELECT \
            l.id, \
            l.title, \
            l.type AS listing_type, \
            l.status, \
            l.city, \
            l.neighborhood, \
            l.price::float8 AS price, \
            to_char(l.published_at AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') AS published_at, \
            to_char(l.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') AS created_at, ",
    );
    qb.push(format_args!(
        "{HAS_COVER} AS has_photo, \
            {IS_PUBLIC} AS is_public, \
            u.id AS owner_id, \
            u.first_name AS owner_first_name, \
            u.last_name AS owner_last_name, \
            u.email AS owner_email \
         FROM listings l \
         JOIN users u ON u.id = l.owner_id \
         WHERE 1 = 1"
    ));
    qb
}

/// Number of listings matching the filters, whatever their state.
pub async fn count_admin_listings(
    pool: &PgPool,
    filters: &ListingFilters,
    visibility: Option<AdminListingVisibility>,
    search: Option<&str>,
) -> Result<i64, AppError> {
    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT COUNT(*) FROM listings l JOIN users u ON u.id = l.owner_id WHERE 1 = 1",
    );
    push_admin_filters(&mut qb, filters, visibility, search);

    qb.build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
}

/// One page of every listing matching the filters, newest first. Drafts and
/// listings without a photo are included.
pub async fn list_admin_listings(
    pool: &PgPool,
    filters: &ListingFilters,
    visibility: Option<AdminListingVisibility>,
    search: Option<&str>,
    limit: u32,
    offset: u64,
) -> Result<Vec<AdminListingRow>, AppError> {
    let mut qb = admin_listing_select();
    push_admin_filters(&mut qb, filters, visibility, search);
    // Rows inserted in one transaction share `created_at`; `id` keeps the pages stable.
    qb.push(" ORDER BY l.created_at DESC, l.id DESC LIMIT ");
    qb.push_bind(i64::from(limit));
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);

    qb.build_query_as::<AdminListingRow>()
        .fetch_all(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
}

/// Full detail for one listing (owner joined, media excluded — see
/// [`find_media_for_listing`]). Returns `None` when the id doesn't exist.
pub async fn find_listing_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ListingDetailRow>, AppError> {
    // `query_as!` can't embed the visibility constants.
    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT \
            l.id, \
            l.title, \
            l.description, \
            l.type AS listing_type, \
            l.status, \
            l.city, \
            l.neighborhood, \
            l.price::float8 AS price, \
            l.surface_m2, \
            l.rooms, \
            to_char(l.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') AS created_at, \
            to_char(l.published_at AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') AS published_at, ",
    );
    qb.push(format_args!(
        "{HAS_COVER} AS has_photo, \
            {IS_PUBLIC} AS is_public, \
            {OWNER_IS_ELIGIBLE} AS owner_is_eligible, \
            u.id AS owner_id, \
            u.first_name AS owner_first_name, \
            u.last_name AS owner_last_name, \
            u.avatar_url AS owner_avatar_url \
         FROM listings l \
         JOIN users u ON u.id = l.owner_id \
         WHERE l.id = "
    ));
    qb.push_bind(id);

    qb.build_query_as::<ListingDetailRow>()
        .fetch_optional(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
}

/// All media attachments for a listing, unsorted; the service applies the
/// display order.
pub async fn find_media_for_listing(
    pool: &PgPool,
    listing_id: Uuid,
) -> Result<Vec<ListingMediaRow>, AppError> {
    sqlx::query_as!(
        ListingMediaRow,
        r#"
        SELECT id, url, is_cover, position,
            (EXTRACT(EPOCH FROM created_at) * 1000000)::bigint AS "created_at_micros!"
        FROM listing_media
        WHERE listing_id = $1
        "#,
        listing_id
    )
    .fetch_all(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Inserts a listing owned by `owner_id` and returns its id. `status` takes the
/// column default (`available`); `search_vector` is maintained by trigger.
pub async fn insert_listing(
    pool: &PgPool,
    owner_id: Uuid,
    listing: &ListingFields,
) -> Result<Uuid, AppError> {
    // `$5::bigint::numeric` keeps the bind an i64: a bare `numeric` bind would need a decimal crate.
    sqlx::query_scalar!(
        r#"
        INSERT INTO listings
            (owner_id, title, description, type, price, currency, city, neighborhood, surface_m2, rooms)
        VALUES ($1, $2, $3, $4, $5::bigint::numeric, $6, $7, $8, $9, $10)
        RETURNING id
        "#,
        owner_id,
        listing.title,
        listing.description,
        listing.listing_type as ListingType,
        listing.price,
        LISTING_CURRENCY,
        listing.city,
        listing.neighborhood,
        listing.surface_m2,
        listing.rooms,
    )
    .fetch_one(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Replaces the editable fields of `id` if `owner_id` owns it; `None` otherwise.
/// `updated_at` and `search_vector` are set by trigger.
pub async fn update_listing(
    pool: &PgPool,
    id: Uuid,
    owner_id: Uuid,
    listing: &ListingFields,
) -> Result<Option<Uuid>, AppError> {
    // Ownership is part of the WHERE clause, so no read can race with the write.
    sqlx::query_scalar!(
        r#"
        UPDATE listings
        SET title = $3, description = $4, type = $5, price = $6::bigint::numeric,
            city = $7, neighborhood = $8, surface_m2 = $9, rooms = $10
        WHERE id = $1 AND owner_id = $2
        RETURNING id
        "#,
        id,
        owner_id,
        listing.title,
        listing.description,
        listing.listing_type as ListingType,
        listing.price,
        listing.city,
        listing.neighborhood,
        listing.surface_m2,
        listing.rooms,
    )
    .fetch_optional(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Sets the status of `id` if `owner_id` owns it and returns the stored values;
/// `None` when the listing doesn't exist or belongs to another owner.
pub async fn update_listing_status(
    pool: &PgPool,
    id: Uuid,
    owner_id: Uuid,
    status: ListingStatus,
) -> Result<Option<(Uuid, ListingStatus)>, AppError> {
    // Setting the current value still matches the row, so a repeated call returns it too.
    sqlx::query!(
        r#"
        UPDATE listings
        SET status = $3
        WHERE id = $1 AND owner_id = $2
        RETURNING id, status AS "status: ListingStatus"
        "#,
        id,
        owner_id,
        status as ListingStatus,
    )
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|row| (row.id, row.status)))
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Sets `published_at` on `owner_id`'s draft `id`. A listing that is already published
/// keeps its original date, so matching no row is not an error.
pub async fn publish_listing(pool: &PgPool, id: Uuid, owner_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE listings
        SET published_at = now()
        WHERE id = $1 AND owner_id = $2 AND published_at IS NULL
        "#,
        id,
        owner_id,
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Locks listing `id` until the transaction ends if `owner_id` owns it, and returns whether
/// it did. Uploads, cover changes and photo deletes take the same row lock.
pub async fn lock_owned_listing<'e, E>(
    executor: E,
    id: Uuid,
    owner_id: Uuid,
) -> Result<bool, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query_scalar!(
        r#"SELECT id FROM listings WHERE id = $1 AND owner_id = $2 FOR UPDATE"#,
        id,
        owner_id
    )
    .fetch_optional(executor)
    .await
    .map(|row| row.is_some())
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Locks listing `id` until the transaction ends, whoever owns it, and returns whether
/// it exists. Same row lock as [`lock_owned_listing`].
pub async fn lock_listing<'e, E>(executor: E, id: Uuid) -> Result<bool, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query_scalar!(r#"SELECT id FROM listings WHERE id = $1 FOR UPDATE"#, id)
        .fetch_optional(executor)
        .await
        .map(|row| row.is_some())
        .map_err(|error| AppError::Database(error.to_string()))
}

/// Storage keys of every photo attached to `listing_id`.
pub async fn list_media_keys<'e, E>(executor: E, listing_id: Uuid) -> Result<Vec<String>, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query_scalar!(
        r#"SELECT storage_key FROM listing_media WHERE listing_id = $1"#,
        listing_id
    )
    .fetch_all(executor)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Deletes `id`; the `listing_media` rows go with it through the cascade.
pub async fn delete_listing<'e, E>(executor: E, id: Uuid) -> Result<(), AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query!(r#"DELETE FROM listings WHERE id = $1"#, id)
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(|error| AppError::Database(error.to_string()))
}
