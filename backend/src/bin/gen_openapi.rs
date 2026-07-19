//! Writes the OpenAPI schema to a file without booting a database or HTTP
//! server — the schema is collected purely from `#[utoipa::path]` route
//! annotations (see `route::openapi_spec`).
//!
//! Usage: `cargo run --bin gen_openapi -- <output-path>` (defaults to
//! `../frontend/docs-frontend/openapi.json`, relative to the `backend/` crate root).

use std::{fs, path::PathBuf};

fn main() {
    let output_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("../frontend/docs-frontend/openapi.json"));

    let json = backend_my_house::route::openapi_spec()
        .to_pretty_json()
        .expect("serialize OpenAPI schema to JSON");

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("create output directory");
    }
    fs::write(&output_path, json).expect("write OpenAPI schema to disk");

    println!("OpenAPI schema written to {}", output_path.display());
}
