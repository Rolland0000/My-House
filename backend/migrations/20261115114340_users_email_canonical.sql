-- users.email is stored in canonical form (trimmed, lowercase), the form the
-- backend normalises every address to before a lookup or an insert. The
-- existing UNIQUE(email) then refuses a second account for the same mailbox
-- whatever its casing.
--
-- Two rows sharing a canonical address make the migration fail instead of
-- merging or deleting accounts: which one to keep is a decision for a person.

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM users GROUP BY lower(btrim(email)) HAVING count(*) > 1
    ) THEN
        RAISE EXCEPTION 'users.email holds case or whitespace duplicates; resolve them before applying this migration';
    END IF;
END $$;

UPDATE users SET email = lower(btrim(email)) WHERE email <> lower(btrim(email));

ALTER TABLE users
  ADD CONSTRAINT users_email_canonical CHECK (email = lower(btrim(email)));
