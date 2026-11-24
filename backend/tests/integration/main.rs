//! In-process HTTP integration suite. Each test gets its own migrated database
//! from `#[sqlx::test]`, so `DATABASE_URL` must name a role allowed to create
//! databases. Run with `cargo test --features integration`.
#![cfg(feature = "integration")]

mod admin_moderation;
mod auth_session;
mod contact_reveal;
mod discovery;
mod harness;
mod harness_checks;
mod health;
mod listing_lifecycle;
mod owner_onboarding;
