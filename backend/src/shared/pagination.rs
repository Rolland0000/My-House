use serde::Serialize;
use utoipa::ToSchema;

// ─────────────────────────────────────────────────────────────────────────────
// Pagination
//
// Matches the API contract envelope for paginated responses:
//   { "data": [...], "pagination": { "page": 1, "per_page": 20, "total": 143, "total_pages": 8 } }
// ─────────────────────────────────────────────────────────────────────────────

/// Pagination metadata included alongside `data` in list responses.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// Full paginated response envelope.
///
/// Generic over the item type so every module can reuse it:
/// `PaginatedResponse<ListingSummaryDto>`, `PaginatedResponse<UserDto>`, etc.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize + ToSchema> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

/// Default number of items per page (API contract: default 20).
pub const DEFAULT_PER_PAGE: u32 = 20;

/// Hard ceiling for per_page to prevent abuse (API contract: max 50).
pub const MAX_PER_PAGE: u32 = 50;

impl PaginationMeta {
    /// Builds pagination metadata from the raw query parameters and a total row count.
    ///
    /// `raw_page` and `raw_per_page` come straight from query params — this
    /// function clamps them to sane bounds so callers don't need to validate.
    pub fn new(raw_page: Option<u32>, raw_per_page: Option<u32>, total: u64) -> Self {
        let per_page = raw_per_page
            .unwrap_or(DEFAULT_PER_PAGE)
            .clamp(1, MAX_PER_PAGE);

        let total_pages = if total == 0 {
            0
        } else {
            total.div_ceil(per_page as u64) as u32
        };

        let page = raw_page.unwrap_or(1).max(1);

        Self {
            page,
            per_page,
            total,
            total_pages,
        }
    }

    /// SQL-ready offset computed from `page` and `per_page`.
    pub fn offset(&self) -> u64 {
        (self.page.saturating_sub(1) as u64) * (self.per_page as u64)
    }
}

impl<T: Serialize + ToSchema> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, meta: PaginationMeta) -> Self {
        Self {
            data,
            pagination: meta,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults_applied_when_none() {
        let meta = PaginationMeta::new(None, None, 100);
        assert_eq!(meta.page, 1);
        assert_eq!(meta.per_page, DEFAULT_PER_PAGE);
        assert_eq!(meta.total, 100);
        assert_eq!(meta.total_pages, 5);
        assert_eq!(meta.offset(), 0);
    }

    #[test]
    fn test_per_page_clamped_to_max() {
        let meta = PaginationMeta::new(Some(1), Some(200), 100);
        assert_eq!(meta.per_page, MAX_PER_PAGE);
    }

    #[test]
    fn test_per_page_clamped_to_min() {
        let meta = PaginationMeta::new(Some(1), Some(0), 100);
        assert_eq!(meta.per_page, 1);
    }

    #[test]
    fn test_page_floors_at_one() {
        let meta = PaginationMeta::new(Some(0), None, 100);
        assert_eq!(meta.page, 1);
        assert_eq!(meta.offset(), 0);
    }

    #[test]
    fn test_offset_calculation() {
        let meta = PaginationMeta::new(Some(3), Some(20), 100);
        assert_eq!(meta.offset(), 40);
    }

    #[test]
    fn test_total_pages_rounds_up() {
        let meta = PaginationMeta::new(Some(1), Some(20), 41);
        assert_eq!(meta.total_pages, 3);
    }

    #[test]
    fn test_zero_total_gives_zero_pages() {
        let meta = PaginationMeta::new(None, None, 0);
        assert_eq!(meta.total_pages, 0);
    }

    #[test]
    fn test_paginated_response_serializes_correctly() {
        let meta = PaginationMeta::new(Some(2), Some(10), 25);
        let resp = PaginatedResponse::new(vec!["a", "b"], meta);

        let json = serde_json::to_value(&resp).expect("serialization failed");
        assert_eq!(json["data"], serde_json::json!(["a", "b"]));
        assert_eq!(json["pagination"]["page"], 2);
        assert_eq!(json["pagination"]["per_page"], 10);
        assert_eq!(json["pagination"]["total"], 25);
        assert_eq!(json["pagination"]["total_pages"], 3);
    }
}
