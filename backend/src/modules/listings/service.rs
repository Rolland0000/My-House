use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::errors::AppError;
use crate::shared::pagination::{PaginatedResponse, PaginationMeta};

use super::dto::{ListListingsQuery, ListingDetailDto, ListingSummaryDto};
use super::repository::{self, ListingFilters};

/// Fetches one page of the public listings feed matching `query`'s filters.
pub async fn list_listings(
    pool: &PgPool,
    query: ListListingsQuery,
) -> Result<PaginatedResponse<ListingSummaryDto>, AppError> {
    let filters = ListingFilters {
        owner_id: query.owner_id,
        city: query.city,
        listing_type: query.listing_type,
    };

    let total = repository::count_listings(pool, &filters).await?;
    let meta = PaginationMeta::new(query.page, query.per_page, total as u64);

    let rows = repository::list_listings(pool, &filters, meta.per_page, meta.offset()).await?;
    let data = rows.into_iter().map(ListingSummaryDto::from).collect();

    Ok(PaginatedResponse::new(data, meta))
}

/// Fetches the full detail (owner + media) for one listing.
pub async fn get_listing_detail(pool: &PgPool, id: Uuid) -> Result<ListingDetailDto, AppError> {
    let row = repository::find_listing_by_id(pool, id)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    let media = repository::find_media_for_listing(pool, id).await?;

    Ok(ListingDetailDto::from_row_and_media(row, media))
}
