-- Admin password login: the admin signs in with email + password instead of OTP.
-- password_hash holds an Argon2id PHC string; it is only ever set on admin rows.
-- failed_login_attempts / locked_until back the temporary lockout after repeated
-- wrong passwords and are reset on a successful login.

ALTER TABLE users
  ADD COLUMN password_hash         VARCHAR(255),
  ADD COLUMN failed_login_attempts INTEGER     NOT NULL DEFAULT 0,
  ADD COLUMN locked_until          TIMESTAMPTZ;

ALTER TABLE users
  ADD CONSTRAINT users_password_only_for_admin
  CHECK (password_hash IS NULL OR role = 'admin');
