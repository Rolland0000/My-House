ALTER TABLE listings ADD COLUMN published_at TIMESTAMPTZ NULL;

-- Backfill: existing listings were public before this column existed.
UPDATE listings SET published_at = created_at;
