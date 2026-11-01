use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::modules::listings::repository::push_public_visibility;
use crate::shared::errors::AppError;

#[derive(Debug, FromRow)]
pub struct OwnerContactRow {
    pub phone: String,
    pub secondary_phone: Option<String>,
}

/// Owner numbers of a publicly visible listing whose owner is active and has a
/// phone. `None` covers every other case, an unknown id included.
pub async fn find_owner_contact(
    pool: &PgPool,
    listing_id: Uuid,
) -> Result<Option<OwnerContactRow>, AppError> {
    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT u.phone, r.secondary_phone \
        FROM listings l \
        JOIN users u ON u.id = l.owner_id \
        LEFT JOIN LATERAL ( \
            SELECT o.secondary_phone FROM owner_requests o \
            WHERE o.user_id = u.id AND o.status = 'approved' \
            ORDER BY o.reviewed_at DESC NULLS LAST, o.created_at DESC \
            LIMIT 1 \
        ) r ON TRUE \
        WHERE l.id = ",
    );
    qb.push_bind(listing_id);
    qb.push(" AND u.is_active AND u.phone IS NOT NULL");
    push_public_visibility(&mut qb);

    qb.build_query_as::<OwnerContactRow>()
        .fetch_optional(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
}
