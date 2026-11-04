use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use sqlx::PgPool;
use uuid::Uuid;

use crate::infra::cache::AppCacheProvider;
use crate::shared::errors::AppError;
use crate::shared::types::RevealWindow;

use super::dto::ContactDto;
use super::repository;

/// Per-user reveal cap: at most `max_per_window` successful reveals per
/// fixed `window`.
#[derive(Debug, Clone, Copy)]
pub struct RevealCap {
    pub max_per_window: u32,
    pub window: Duration,
}

/// Phone numbers of the owner of `listing_id`, for `user_id`.
///
/// Only a successful reveal counts toward the cap: the slot reserved up front
/// is handed back on a 404, a database error or a dropped request.
pub async fn reveal_owner_contact(
    pool: &PgPool,
    windows: &dyn AppCacheProvider<Uuid, Arc<RevealWindow>>,
    cap: RevealCap,
    user_id: Uuid,
    listing_id: Uuid,
) -> Result<ContactDto, AppError> {
    let slot = reserve_reveal(windows, cap, user_id).await?;

    let row = repository::find_owner_contact(pool, listing_id)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    slot.commit();

    tracing::info!(user_id = %user_id, listing_id = %listing_id, "owner contact revealed");
    Ok(ContactDto {
        phone: row.phone,
        secondary_phone: row.secondary_phone,
    })
}

/// A reveal counted in a window, released on drop unless committed.
struct ReservedReveal {
    window: Arc<RevealWindow>,
    committed: bool,
}

impl ReservedReveal {
    fn commit(mut self) {
        self.committed = true;
    }
}

impl Drop for ReservedReveal {
    fn drop(&mut self) {
        if !self.committed {
            self.window.count.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

/// Takes one of `user_id`'s reveals in the current window, or fails with
/// `ContactRateLimited` when none is left. A refusal does not count.
///
/// When a user's first two calls race, each may seed its own window and one
/// count is lost, as in the per-IP limiter.
async fn reserve_reveal(
    windows: &dyn AppCacheProvider<Uuid, Arc<RevealWindow>>,
    cap: RevealCap,
    user_id: Uuid,
) -> Result<ReservedReveal, AppError> {
    let window = match windows.get(&user_id).await {
        Some(existing) => existing,
        None => {
            let started = Arc::new(RevealWindow::start());
            windows.insert(user_id, Arc::clone(&started)).await;
            started
        }
    };

    let reserved = window
        .count
        .try_update(Ordering::Relaxed, Ordering::Relaxed, |count| {
            (count < cap.max_per_window).then_some(count + 1)
        })
        .is_ok();
    if reserved {
        return Ok(ReservedReveal {
            window,
            committed: false,
        });
    }

    let retry_after_seconds = seconds_until_window_ends(&window, cap.window);
    tracing::warn!(user_id = %user_id, retry_after_seconds, "contact reveal limit exceeded");
    Err(AppError::ContactRateLimited {
        retry_after_seconds,
    })
}

/// Whole seconds, rounded up and at least 1, until `window` ends.
fn seconds_until_window_ends(window: &RevealWindow, length: Duration) -> u64 {
    let remaining = length.saturating_sub(window.started_at.elapsed());
    let rounded_up = remaining.as_secs() + u64::from(remaining.subsec_nanos() > 0);
    rounded_up.max(1)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

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
    async fn allows_up_to_the_cap_then_rejects_with_retry_after() {
        let windows = windows();
        let user_id = Uuid::new_v4();

        for _ in 0..2 {
            reserve_reveal(windows.as_ref(), cap(2), user_id)
                .await
                .expect("within the cap")
                .commit();
        }
        match reserve_reveal(windows.as_ref(), cap(2), user_id).await {
            Err(AppError::ContactRateLimited {
                retry_after_seconds,
            }) => assert!((1..=WINDOW.as_secs()).contains(&retry_after_seconds)),
            other => panic!("expected ContactRateLimited, got {:?}", other.map(|_| ())),
        }
    }

    #[tokio::test]
    async fn uncommitted_reveals_and_refusals_do_not_count() {
        let windows = windows();
        let user_id = Uuid::new_v4();

        drop(reserve_reveal(windows.as_ref(), cap(1), user_id).await);
        let slot = reserve_reveal(windows.as_ref(), cap(1), user_id)
            .await
            .expect("the dropped reveal was released");
        slot.commit();
        assert!(reserve_reveal(windows.as_ref(), cap(1), user_id)
            .await
            .is_err());

        let window = windows.get(&user_id).await.expect("window exists");
        assert_eq!(window.count.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn users_have_independent_counters() {
        let windows = windows();

        reserve_reveal(windows.as_ref(), cap(1), Uuid::new_v4())
            .await
            .expect("first user")
            .commit();
        reserve_reveal(windows.as_ref(), cap(1), Uuid::new_v4())
            .await
            .expect("second user")
            .commit();
    }

    #[test]
    fn retry_after_rounds_up_and_never_reaches_zero() {
        let window = RevealWindow {
            started_at: Instant::now() - Duration::from_millis(1_500),
            count: Default::default(),
        };
        assert_eq!(
            seconds_until_window_ends(&window, Duration::from_secs(10)),
            9
        );
        assert_eq!(
            seconds_until_window_ends(&window, Duration::from_secs(1)),
            1
        );
    }
}
