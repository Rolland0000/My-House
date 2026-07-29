-- MH-19: Baseline schema migration (enums, tables, indexes)
-- ============================================================
-- Extensions
-- ============================================================
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "unaccent";

-- ============================================================
-- Enums
-- ============================================================
CREATE TYPE user_role             AS ENUM ('seeker', 'owner', 'admin');
CREATE TYPE owner_request_status  AS ENUM ('pending', 'approved', 'rejected');
CREATE TYPE listing_type          AS ENUM ('apartment', 'studio', 'house', 'room', 'villa', 'other');
CREATE TYPE listing_status        AS ENUM ('available', 'unavailable');

-- ============================================================
-- users
-- ============================================================
CREATE TABLE users (
    id          UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    email       VARCHAR(255) NOT NULL UNIQUE,
    role        user_role   NOT NULL DEFAULT 'seeker',
    first_name  VARCHAR(100),
    last_name   VARCHAR(100),
    phone       VARCHAR(30),           -- Obligatoire pour le rôle owner
    avatar_url  VARCHAR(500),
    is_active   BOOLEAN     NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- refresh_tokens
-- ============================================================
CREATE TABLE refresh_tokens (
    id              UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash      VARCHAR(255) NOT NULL,
    expires_at      TIMESTAMPTZ NOT NULL,
    revoked_at      TIMESTAMPTZ,                                   -- NULL = actif
    replaced_by_id  UUID        REFERENCES refresh_tokens(id),     -- chaîne de rotation
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_hash    ON refresh_tokens(token_hash);

-- ============================================================
-- owner_requests
-- ============================================================
CREATE TABLE owner_requests (
    id                  UUID                 PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id             UUID                 NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    phone               VARCHAR(30)          NOT NULL,
    identity_data       JSONB                NOT NULL,             -- ex: { "full_name", "id_number", "id_type" }
    identity_documents  JSONB                NOT NULL DEFAULT '[]', -- [{ "storage_key", "original_filename" }]
    status              owner_request_status NOT NULL DEFAULT 'pending',
    admin_note          TEXT,
    reviewed_by         UUID                 REFERENCES users(id),
    reviewed_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ          NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ          NOT NULL DEFAULT NOW()
);

-- Un seul request actif (pending) par user à la fois
CREATE UNIQUE INDEX idx_owner_requests_one_pending
    ON owner_requests(user_id) WHERE status = 'pending';

CREATE INDEX idx_owner_requests_status ON owner_requests(status);

-- ============================================================
-- listings
-- ============================================================
CREATE TABLE listings (
    id            UUID           PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id      UUID           NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title         VARCHAR(255)   NOT NULL,
    description   TEXT           NOT NULL,
    type          listing_type   NOT NULL,
    status        listing_status NOT NULL DEFAULT 'available',
    city          VARCHAR(100)   NOT NULL,
    neighborhood  VARCHAR(100),
    price         NUMERIC(12,2)  NOT NULL,
    surface_m2    INTEGER,
    rooms         INTEGER,
    search_vector TSVECTOR,
    created_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_listings_search ON listings USING GIN(search_vector);
CREATE INDEX idx_listings_owner  ON listings(owner_id);
CREATE INDEX idx_listings_city   ON listings(city);
CREATE INDEX idx_listings_status ON listings(status);
CREATE INDEX idx_listings_type   ON listings(type);

-- ============================================================
-- listing_media
-- ============================================================
CREATE TABLE listing_media (
    id          UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    listing_id  UUID        NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    storage_key VARCHAR(500) NOT NULL,    -- Chemin interne au storage (filesystem MVP / S3 V2)
    url         VARCHAR(1000) NOT NULL,   -- URL publique permanente, jamais pré-signée/expirante
    is_cover    BOOLEAN     NOT NULL DEFAULT FALSE,
    position    SMALLINT    NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_listing_media_listing ON listing_media(listing_id);

-- Une seule cover photo par listing
CREATE UNIQUE INDEX idx_listing_media_one_cover
    ON listing_media(listing_id) WHERE is_cover = TRUE;
