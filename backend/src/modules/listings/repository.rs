use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::shared::errors::AppError;

use super::model::{
    ListingDetailRow, ListingFields, ListingMediaRow, ListingStatus, ListingSummaryRow, ListingType,
};

/// The feed and the spec price everything in XAF; the column has no default.
const LISTING_CURRENCY: &str = "XAF";

/// Optional, combinable filters for `GET /listings` (owner_id, city, type —
/// API contract §4.3). Built with `QueryBuilder` rather than static SQL
/// per `.claude/rules/database.md` ("optional/combinable filters").
#[derive(Debug, Default)]
pub struct ListingFilters {
    pub owner_id: Option<Uuid>,
    pub city: Option<String>,
    pub listing_type: Option<ListingType>,
}

/// Public-visibility predicate: a listing is public once published and it has
/// at least one photo (a cover row — the first upload is always marked cover).
/// Shared by `count_listings` and `list_listings` so `total` stays aligned
/// with the returned pages; `pub` so search (EP-10) and contact reveal (EP-11)
/// can reuse it instead of re-deriving the rule.
pub fn push_public_visibility(qb: &mut QueryBuilder<'_, Postgres>) {
    qb.push(
        " AND l.published_at IS NOT NULL AND EXISTS (\
            SELECT 1 FROM listing_media c WHERE c.listing_id = l.id AND c.is_cover\
        )",
    );
}

fn push_filters(qb: &mut QueryBuilder<'_, Postgres>, filters: &ListingFilters) {
    if let Some(owner_id) = filters.owner_id {
        qb.push(" AND l.owner_id = ");
        qb.push_bind(owner_id);
    }
    if let Some(city) = &filters.city {
        qb.push(" AND lower(l.city) = lower(");
        qb.push_bind(city.clone());
        qb.push(")");
    }
    if let Some(listing_type) = filters.listing_type {
        qb.push(" AND l.type = ");
        qb.push_bind(listing_type);
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

/// One page of the public listings feed, newest first.
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
    // Rows inserted in one transaction share `created_at`; `id` keeps the pages stable.
    qb.push(" ORDER BY l.created_at DESC, l.id DESC LIMIT ");
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

/// Full detail for one listing (owner joined, media excluded — see
/// [`find_media_for_listing`]). Returns `None` when the id doesn't exist.
pub async fn find_listing_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ListingDetailRow>, AppError> {
    sqlx::query_as!(
        ListingDetailRow,
        r#"
        SELECT
            l.id,
            l.title,
            l.description,
            l.type AS "listing_type: ListingType",
            l.status AS "status: crate::modules::listings::model::ListingStatus",
            l.city,
            l.neighborhood,
            l.price::float8 AS "price!",
            l.surface_m2,
            l.rooms,
            to_char(l.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS "created_at!",
            to_char(l.published_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS published_at,
            EXISTS (SELECT 1 FROM listing_media c WHERE c.listing_id = l.id AND c.is_cover) AS "has_photo!",
            u.id AS "owner_id!",
            u.first_name AS owner_first_name,
            u.last_name AS owner_last_name,
            u.avatar_url AS owner_avatar_url
        FROM listings l
        JOIN users u ON u.id = l.owner_id
        WHERE l.id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// All media attachments for a listing, ordered for display (cover first via
/// `position`, which the owner-write path — out of scope here — keeps in sync).
pub async fn find_media_for_listing(
    pool: &PgPool,
    listing_id: Uuid,
) -> Result<Vec<ListingMediaRow>, AppError> {
    sqlx::query_as!(
        ListingMediaRow,
        r#"SELECT id, url, is_cover, position FROM listing_media WHERE listing_id = $1 ORDER BY position ASC"#,
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
