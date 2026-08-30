-- last_name and phone are mandatory for seeker and owner, nullable only for
-- admin (bootstrapped from an email alone) — this rectifies the baseline's
-- "Obligatoire pour le rôle owner" comment on phone.
-- NOT VALID grandfathers the email-only rows left by the previous flow.

ALTER TABLE users
  ADD CONSTRAINT users_profile_complete_for_non_admin
  CHECK (role = 'admin' OR (last_name IS NOT NULL AND phone IS NOT NULL))
  NOT VALID;
