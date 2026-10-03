use super::StorageProvider;

/// Best-effort delete for every key in `keys`; a failure (including a file
/// already absent) is logged and never aborts the remaining keys.
pub async fn delete_storage_objects(storage: &dyn StorageProvider, keys: &[String]) {
    for key in keys {
        if let Err(error) = storage.delete(key).await {
            tracing::warn!(key, error = %error, "failed to delete storage object; continuing");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::storage::test_support::RecordingStorage;

    #[tokio::test]
    async fn deletes_every_enumerated_key() {
        let storage = RecordingStorage::new(false);
        let keys = vec![
            "avatars/u/1.png".to_string(),
            "listings/l/2.jpg".to_string(),
            "owner-requests/r/3.pdf".to_string(),
        ];

        delete_storage_objects(&storage, &keys).await;

        assert_eq!(storage.deleted_keys(), keys);
    }

    #[tokio::test]
    async fn a_missing_or_failing_key_does_not_abort_the_remaining_keys() {
        let storage = RecordingStorage::new(true);
        let keys = vec![
            "listings/l/1.jpg".to_string(),
            "listings/l/2.jpg".to_string(),
        ];

        delete_storage_objects(&storage, &keys).await;

        assert_eq!(storage.deleted_keys(), keys);
    }

    #[tokio::test]
    async fn no_keys_is_a_no_op() {
        let storage = RecordingStorage::new(false);

        delete_storage_objects(&storage, &[]).await;

        assert!(storage.deleted_keys().is_empty());
    }
}
