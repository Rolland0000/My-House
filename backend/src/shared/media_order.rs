//! Display order of a listing's photos, shared by every read that returns them.

use uuid::Uuid;

/// Sort key for one photo: the cover first, then `position`, then the oldest
/// upload, then `id`.
pub fn display_order_key(
    is_cover: bool,
    position: i16,
    created_at_micros: i64,
    id: Uuid,
) -> (bool, i16, i64, Uuid) {
    (!is_cover, position, created_at_micros, id)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Photo {
        id: Uuid,
        is_cover: bool,
        position: i16,
        created_at_micros: i64,
    }

    fn photo(is_cover: bool, position: i16, created_at_micros: i64) -> Photo {
        Photo {
            id: Uuid::new_v4(),
            is_cover,
            position,
            created_at_micros,
        }
    }

    fn sorted_ids(mut photos: Vec<Photo>) -> Vec<Uuid> {
        photos.sort_by_key(|photo| {
            display_order_key(
                photo.is_cover,
                photo.position,
                photo.created_at_micros,
                photo.id,
            )
        });
        photos.into_iter().map(|photo| photo.id).collect()
    }

    #[test]
    fn cover_comes_first_even_with_the_highest_position() {
        let cover = photo(true, 4, 0);
        let other = photo(false, 0, 0);
        let expected = vec![cover.id, other.id];
        assert_eq!(sorted_ids(vec![other, cover]), expected);
    }

    #[test]
    fn other_photos_follow_ascending_position() {
        let first = photo(false, 1, 300);
        let second = photo(false, 2, 200);
        let third = photo(false, 3, 100);
        let expected = vec![first.id, second.id, third.id];
        assert_eq!(sorted_ids(vec![third, first, second]), expected);
    }

    #[test]
    fn equal_positions_put_the_oldest_upload_first() {
        let older = photo(false, 1, 100);
        let newer = photo(false, 1, 200);
        let expected = vec![older.id, newer.id];
        assert_eq!(sorted_ids(vec![newer, older]), expected);
    }

    #[test]
    fn equal_position_and_upload_time_fall_back_to_id() {
        let mut low = photo(false, 1, 100);
        let mut high = photo(false, 1, 100);
        low.id = Uuid::from_u128(1);
        high.id = Uuid::from_u128(2);
        assert_eq!(
            sorted_ids(vec![high, low]),
            vec![Uuid::from_u128(1), Uuid::from_u128(2)]
        );
    }
}
