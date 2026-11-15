-- refresh_tokens retention: rows expired for more than 30 days are purged by
-- the application on each rotation (auth::repository::purge_stale_tokens).
--
-- Every DELETE on refresh_tokens checks the self-reference replaced_by_id;
-- without an index that check scans the whole table.
CREATE INDEX idx_refresh_tokens_replaced_by_id ON refresh_tokens(replaced_by_id);

-- A purged token may still be the replacement named by an older row. SET NULL
-- lets the purge delete it whatever the order of the rotation chain.
ALTER TABLE refresh_tokens
  DROP CONSTRAINT refresh_tokens_replaced_by_id_fkey,
  ADD CONSTRAINT refresh_tokens_replaced_by_id_fkey
    FOREIGN KEY (replaced_by_id) REFERENCES refresh_tokens(id) ON DELETE SET NULL;
