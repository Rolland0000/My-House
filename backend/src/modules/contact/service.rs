use std::sync::Arc;
use std::time::Duration;

use sqlx::PgPool;
use uuid::Uuid;

use crate::infra::cache::AppCacheProvider;
use crate::shared::errors::AppError;
use crate::shared::types::RevealWindow;

use super::dto::ContactDto;
use super::repository;

/// Per-user reveal cap: at most `max_per_window` distinct listings per fixed
/// `window`.
#[derive(Debug, Clone, Copy)]
pub struct RevealCap {
    pub max_per_window: u32,
    pub window: Duration,
}

/// Phone numbers of the owner of `listing_id`, for `user_id`.
///
/// The cap comes after the lookup, so a 404 neither counts nor opens a window.
pub async fn reveal_owner_contact(
    pool: &PgPool,
    windows: &dyn AppCacheProvider<Uuid, Arc<RevealWindow>>,
    cap: RevealCap,
    user_id: Uuid,
    listing_id: Uuid,
) -> Result<ContactDto, AppError> {
    let row = repository::find_owner_contact(pool, listing_id)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    count_reveal(windows, cap, user_id, listing_id).await?;

    tracing::info!(user_id = %user_id, listing_id = %listing_id, "owner contact revealed");
    Ok(ContactDto {
        phone: row.phone,
        secondary_phone: row.secondary_phone,
    })
}

/// Counts `listing_id` in `user_id`'s window, opening one if none is running,
/// or fails with `ContactRateLimited` when it is a listing too many.
async fn count_reveal(
    windows: &dyn AppCacheProvider<Uuid, Arc<RevealWindow>>,
    cap: RevealCap,
    user_id: Uuid,
    listing_id: Uuid,
) -> Result<(), AppError> {
    let window = windows
        .get_or_insert(user_id, Arc::new(RevealWindow::start()))
        .await;
    if window.admit(listing_id, cap.max_per_window as usize) {
        return Ok(());
    }

    let retry_after_seconds = window.seconds_left(cap.window);
    tracing::warn!(user_id = %user_id, retry_after_seconds, "contact reveal limit exceeded");
    Err(AppError::ContactRateLimited {
        retry_after_seconds,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::cache::build_contact_reveal_rate_limit_cache;

    const WINDOW: Duration = Duration::from_secs(60);

    fn cap(max_per_window: u32) -> RevealCap {
        RevealCap {
            max_per_window,
            window: WINDOW,
        }
    }

    fn windows() -> Arc<dyn AppCacheProvider<Uuid, Arc<RevealWindow>>> {
        build_contact_reveal_rate_limit_cache(WINDOW)
    }

    #[tokio::test]
    async fn refuses_a_listing_past_the_cap_with_the_seconds_left() {
        let windows = windows();
        let user_id = Uuid::new_v4();

        for _ in 0..2 {
            count_reveal(windows.as_ref(), cap(2), user_id, Uuid::new_v4())
                .await
                .expect("within the cap");
        }
        match count_reveal(windows.as_ref(), cap(2), user_id, Uuid::new_v4()).await {
            Err(AppError::ContactRateLimited {
                retry_after_seconds,
            }) => assert!((1..=WINDOW.as_secs()).contains(&retry_after_seconds)),
            other => panic!("expected ContactRateLimited, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn a_listing_already_revealed_passes_once_the_cap_is_reached() {
        let windows = windows();
        let user_id = Uuid::new_v4();
        let listing_id = Uuid::new_v4();

        count_reveal(windows.as_ref(), cap(1), user_id, listing_id)
            .await
            .expect("first reveal");
        assert!(
            count_reveal(windows.as_ref(), cap(1), user_id, Uuid::new_v4())
                .await
                .is_err()
        );
        count_reveal(windows.as_ref(), cap(1), user_id, listing_id)
            .await
            .expect("same listing again");
    }

    #[tokio::test]
    async fn a_new_window_allows_new_listings() {
        let length = Duration::from_millis(50);
        let windows = build_contact_reveal_rate_limit_cache(length);
        let cap = RevealCap {
            max_per_window: 1,
            window: length,
        };
        let user_id = Uuid::new_v4();

        count_reveal(windows.as_ref(), cap, user_id, Uuid::new_v4())
            .await
            .expect("first window");
        tokio::time::sleep(length * 2).await;
        count_reveal(windows.as_ref(), cap, user_id, Uuid::new_v4())
            .await
            .expect("next window");
    }

    #[tokio::test]
    async fn concurrent_first_reveals_share_one_window() {
        let windows = windows();
        let user_id = Uuid::new_v4();

        let (first, second) = tokio::join!(
            count_reveal(windows.as_ref(), cap(1), user_id, Uuid::new_v4()),
            count_reveal(windows.as_ref(), cap(1), user_id, Uuid::new_v4()),
        );

        assert!(first.is_ok() != second.is_ok());
    }

    #[tokio::test]
    async fn users_have_independent_counts() {
        let windows = windows();
        let listing_id = Uuid::new_v4();

        count_reveal(windows.as_ref(), cap(1), Uuid::new_v4(), listing_id)
            .await
            .expect("first user");
        count_reveal(windows.as_ref(), cap(1), Uuid::new_v4(), Uuid::new_v4())
            .await
            .expect("second user");
    }
}
