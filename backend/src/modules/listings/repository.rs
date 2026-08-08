use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::shared::errors::AppError;

use super::model::{ListingDetailRow, ListingMediaRow, ListingSummaryRow, ListingType};

/// Optional, combinable filters for `GET /listings` (owner_id, city, type —
/// API contract §4.3). Built with `QueryBuilder` rather than static SQL
/// per `.claude/rules/database.md` ("optional/combinable filters").
#[derive(Debug, Default)]
pub struct ListingFilters {
    pub owner_id: Option<Uuid>,
    pub city: Option<String>,
    pub listing_type: Option<ListingType>,
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
            u.id AS owner_id, \
            u.first_name AS owner_first_name, \
            u.last_name AS owner_last_name \
         FROM listings l \
         JOIN users u ON u.id = l.owner_id \
         LEFT JOIN listing_media lm ON lm.listing_id = l.id AND lm.is_cover = TRUE \
         WHERE 1 = 1",
    );
    push_filters(&mut qb, filters);
    qb.push(" ORDER BY l.created_at DESC LIMIT ");
    qb.push_bind(limit as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);

    qb.build_query_as::<ListingSummaryRow>()
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
