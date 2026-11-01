use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::infra::cache::AppCacheProvider;
use crate::shared::errors::AppError;

use super::dto::ContactDto;
use super::repository;

/// Phone numbers of the owner of `listing_id`, for `user_id`.
///
/// The quota is consumed before the lookup, so a 404 counts too: this slows
/// down id scanning.
pub async fn reveal_owner_contact(
    pool: &PgPool,
    counters: &dyn AppCacheProvider<Uuid, Arc<AtomicU32>>,
    max_per_window: u32,
    user_id: Uuid,
    listing_id: Uuid,
) -> Result<ContactDto, AppError> {
    consume_reveal_quota(counters, user_id, max_per_window).await?;

    let row = repository::find_owner_contact(pool, listing_id)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    tracing::info!(user_id = %user_id, listing_id = %listing_id, "owner contact revealed");
    Ok(ContactDto {
        phone: row.phone,
        secondary_phone: row.secondary_phone,
    })
}

/// Counts one reveal for `user_id` in the current fixed window and fails with
/// `RateLimited` past `max_per_window`.
///
/// When a user's first two calls race, each may seed its own counter and one
/// count is lost, as in the per-IP limiter.
async fn consume_reveal_quota(
    counters: &dyn AppCacheProvider<Uuid, Arc<AtomicU32>>,
    user_id: Uuid,
    max_per_window: u32,
) -> Result<(), AppError> {
    let counter = match counters.get(&user_id).await {
        Some(existing) => existing,
        None => {
            let seeded = Arc::new(AtomicU32::new(0));
            counters.insert(user_id, Arc::clone(&seeded)).await;
            seeded
        }
    };
    let count = counter.fetch_add(1, Ordering::Relaxed) + 1;
    if count <= max_per_window {
        return Ok(());
    }

    tracing::warn!(user_id = %user_id, "contact reveal limit exceeded");
    Err(AppError::RateLimited)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::infra::cache::build_contact_reveal_rate_limit_cache;

    fn counters() -> Arc<dyn AppCacheProvider<Uuid, Arc<AtomicU32>>> {
        build_contact_reveal_rate_limit_cache(Duration::from_secs(60))
    }

    #[tokio::test]
    async fn allows_up_to_the_cap_then_rejects() {
        let counters = counters();
        let user_id = Uuid::new_v4();

        assert!(consume_reveal_quota(counters.as_ref(), user_id, 2)
            .await
            .is_ok());
        assert!(consume_reveal_quota(counters.as_ref(), user_id, 2)
            .await
            .is_ok());
        assert!(matches!(
            consume_reveal_quota(counters.as_ref(), user_id, 2).await,
            Err(AppError::RateLimited)
        ));
    }

    #[tokio::test]
    async fn users_have_independent_counters() {
        let counters = counters();

        assert!(consume_reveal_quota(counters.as_ref(), Uuid::new_v4(), 1)
            .await
            .is_ok());
        assert!(consume_reveal_quota(counters.as_ref(), Uuid::new_v4(), 1)
            .await
            .is_ok());
    }
}
