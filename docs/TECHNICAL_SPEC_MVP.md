# My House — Technical Specification MVP

> **Version :** 1.3
> **Statut :** En validation
> **Prérequis :** [`ARCHITECTURE.md`](./ARCHITECTURE.md) v1.3 — validé
> **Scope :** Spécifications d'implémentation complètes pour le périmètre MVP

---

## Table des Matières

1. [Structure du Projet](#1-structure-du-projet)
2. [Modèle de Données](#2-modèle-de-données)
3. [Abstraction Storage](#3-abstraction-storage)
4. [Stack de Notification](#3bis-stack-de-notification)
5. [Contrat API](#4-contrat-api)
6. [Génération des Types TypeScript](#5-génération-des-types-typescript)
7. [Stratégie de Tests](#6-stratégie-de-tests)
8. [Infrastructure Docker](#7-infrastructure-docker)
9. [Variables d&#39;Environnement](#8-variables-denvironnement)
10. [Exclusions MVP](#9-exclusions-mvp)

---

## 1. Structure du Projet

### 1.1 Backend

```
backend/
├── Cargo.toml
├── Dockerfile
├── compose-backend.yml
├── askama.toml                        # Répertoire des templates email (modules/notifications/templates)
├── .env.example
├── migrations/                        # sqlx migrate — fichiers horodatés .sql, forward-only
│   ├── 20260709000000_baseline.sql
│   ├── 20260729041038_baseline_schema.sql
│   ├── 20260729044830_triggers_functions.sql
│   ├── 20260830120000_users_profile_completeness.sql
│   ├── 20260912081021_owner_requests_secondary_phone.sql
│   ├── 20260927094513_listings_published_at.sql
│   └── 20261024053122_search_vector_english_type_labels.sql
├── seed/
│   └── seed_listings.sql
└── src/
    ├── main.rs                        # Composition root : config, pool, cache, storage, bootstrap admin
    ├── lib.rs
    ├── app_server.rs                  # Bind, serve, graceful shutdown (SIGINT / SIGTERM)
    ├── app_state.rs                   # AppState injecté via State<AppState>
    ├── route.rs                       # Routers des modules sous /api/v1, /health, Swagger UI hors production
    ├── api_doc.rs                     # Métadonnées OpenAPI (utoipa)
    ├── bin/
    │   └── gen_openapi.rs             # Écrit frontend/docs-frontend/openapi.json sans DB ni serveur
    │
    ├── config/
    │   └── mod.rs                     # AppConfig — chargement et validation des env vars
    │
    ├── middleware/
    │   ├── mod.rs
    │   ├── cors.rs                    # CorsLayer (tower-http), origines lues dans ALLOWED_ORIGINS
    │   ├── logging.rs                 # tracing + request_id
    │   └── rate_limit.rs              # Limite générique par IP (complète le rate-limit OTP spécifique)
    │
    ├── infra/
    │   ├── mod.rs
    │   ├── db.rs                      # Pool sqlx::PgPool
    │   ├── health.rs                  # GET /health (PostgreSQL) et GET /health/storage
    │   ├── mailer.rs                  # Client SMTP (lettre)
    │   ├── cache/
    │   │   ├── mod.rs
    │   │   ├── provider.rs            # trait AppCacheProvider
    │   │   └── moka.rs                # impl moka — OTP et tickets d'inscription, is_active, rejeu refresh, rate-limits
    │   └── storage/
    │       ├── mod.rs                 # Factory : sélection impl via STORAGE_PROVIDER (local au MVP)
    │       ├── provider.rs            # trait StorageProvider (incl. presigned_url, non utilisé au MVP)
    │       ├── local_fs.rs            # impl LocalFsStorage (MVP) — AwsS3Storage ajouté en V2
    │       ├── cleanup.rs             # Suppression best-effort d'une liste de clés
    │       └── test_support.rs        # Storage factice pour les tests unitaires
    │
    ├── modules/                       # Tests unitaires dans un bloc #[cfg(test)] de chaque fichier
    │   ├── auth/
    │   │   ├── mod.rs
    │   │   ├── router.rs
    │   │   ├── handler.rs
    │   │   ├── service.rs
    │   │   ├── repository.rs          # refresh_tokens CRUD
    │   │   ├── model.rs
    │   │   └── dto.rs                 # ToSchema derives (utoipa)
    │   │
    │   ├── users/
    │   │   ├── mod.rs
    │   │   ├── router.rs              # /users/me, /users/me/avatar
    │   │   ├── handler.rs
    │   │   ├── service.rs             # Profil, suppression de compte, upload avatar (cf. §3.1)
    │   │   ├── repository.rs          # users CRUD ; lit listing_media et owner_requests.identity_documents (suppression de compte)
    │   │   ├── model.rs
    │   │   └── dto.rs
    │   │
    │   ├── owner_requests/
    │   │   ├── mod.rs
    │   │   ├── router.rs              # POST /owner-requests, GET /users/me/owner-request
    │   │   ├── handler.rs
    │   │   ├── service.rs             # Soumission atomique, upload des pièces (cf. §3.1), revue admin
    │   │   ├── repository.rs          # owner_requests CRUD (incl. identity_data, identity_documents)
    │   │   ├── model.rs
    │   │   └── dto.rs
    │   │
    │   ├── listings/
    │   │   ├── mod.rs
    │   │   ├── router.rs              # /listings, /search, /users/me/listings
    │   │   ├── handler.rs
    │   │   ├── service.rs             # Validation, visibilité publique, publication, recherche
    │   │   ├── repository.rs          # Feed et recherche : une seule paire count/list (QueryBuilder)
    │   │   ├── model.rs
    │   │   └── dto.rs
    │   │
    │   ├── media/
    │   │   ├── mod.rs
    │   │   ├── router.rs              # /media/*, /listings/:id/cover, /listings/:id/media/order
    │   │   ├── handler.rs
    │   │   ├── service.rs             # Photos de listing : quota, cover, ordre, upload vers StorageProvider
    │   │   ├── repository.rs          # listing_media CRUD (seul module qui écrit dans cette table)
    │   │   ├── model.rs
    │   │   └── dto.rs
    │   │
    │   ├── notifications/
    │   │   ├── mod.rs
    │   │   ├── service.rs
    │   │   └── templates/
    │   │       ├── otp.html
    │   │       ├── welcome.html
    │   │       ├── owner_request_received.html
    │   │       ├── owner_request_approved.html
    │   │       └── owner_request_rejected.html
    │   │
    │   ├── admin/
    │   │   ├── mod.rs
    │   │   ├── router.rs              # /admin/owner-requests/*
    │   │   └── handler.rs             # Délègue à owner_requests::service, pas de service propre
    │   │
    │   └── contact/
    │       └── mod.rs                 # Vide, non déclaré dans modules/mod.rs : GET /listings/:id/contact pas encore livré
    │
    └── shared/
        ├── mod.rs
        ├── errors.rs                  # AppError → réponse HTTP structurée
        ├── pagination.rs              # PaginatedResponse<T>
        ├── extractors.rs              # AuthUser, MaybeAuthUser, AppJson, AppQuery (revérifie is_active)
        ├── rbac.rs                    # Contrôle de rôle (seeker/owner/admin), appelé dans les handlers
        ├── types.rs                   # NewType wrappers : UserId, ListingId...
        ├── validation.rs              # Règles de champs (400 ou 422 FieldErrors), place_filter, keyword_filter
        ├── file_validation.rs         # Magic bytes (infer) et tailles max, pour tous les uploads
        ├── storage_key.rs             # Seul constructeur des clés de storage (préfixes publics/privés)
        ├── media_order.rs             # Ordre d'affichage des photos (cover d'abord)
        ├── session_cookie.rs          # Attributs du cookie refresh_token
        ├── token_decoder.rs           # Trait de décodage du JWT utilisé par AuthUser
        └── crypto/
            ├── mod.rs
            ├── jwt.rs                 # Émission et vérification de l'access token
            ├── hashing.rs             # SHA-256 (refresh tokens, codes OTP)
            └── otp.rs                 # Code à 6 chiffres (CSPRNG)
```

La recherche n'a pas de module à elle : `GET /search` est servi par `listings`, qui partage avec le feed les filtres, la règle de visibilité et les deux requêtes SQL (cf. §4.4 et ARCHITECTURE.md ADR-12).

### 1.2 Frontend

```
frontend/
├── package.json
├── package-lock.json
├── tsconfig.json                      # + tsconfig.app.json, tsconfig.node.json
├── vite.config.ts
├── eslint.config.js
├── index.html
├── .env.example
├── Dockerfile
├── compose-frontend.yml
├── nginx.conf
├── docs-frontend/
│   └── openapi.json                   # Écrit par backend/src/bin/gen_openapi.rs, versionné (détection de dérive en CI)
└── src/
    ├── main.tsx
    ├── App.tsx
    ├── index.css
    │
    ├── app/
    │   ├── router.tsx                 # Routes globales, une route lazy par page
    │   ├── providers.tsx              # QueryClientProvider, AuthProvider
    │   ├── RequireAuth.tsx            # Exige une session (refresh silencieux d'abord)
    │   ├── RequireOwner.tsx           # Rôle owner, imbriqué dans RequireAuth
    │   ├── RequireAdmin.tsx           # Rôle admin, imbriqué dans RequireAuth
    │   └── layout/
    │       ├── RootLayout.tsx
    │       ├── AuthLayout.tsx
    │       └── AdminLayout.tsx
    │
    ├── features/                      # Logique pure dans des .ts testés par un *.test.ts voisin (vitest)
    │   ├── auth/
    │   │   ├── components/
    │   │   │   ├── AuthFlow.tsx
    │   │   │   ├── OtpRequestForm.tsx
    │   │   │   ├── OtpVerifyForm.tsx
    │   │   │   ├── OtpCodeInput.tsx
    │   │   │   ├── RegistrationForm.tsx       # Affiché si is_new_user = true
    │   │   │   └── OwnerActivationModal.tsx
    │   │   ├── hooks/                 # useAuth, useOtpRequest, useOtpVerify, useRegister
    │   │   ├── AuthContext.tsx
    │   │   ├── tokenRole.ts
    │   │   ├── constants.ts
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   ├── listings/
    │   │   ├── components/
    │   │   │   ├── ListingFeed.tsx            # Feed public : grille de covers + filtres (ville, quartier, type, statut)
    │   │   │   ├── ListingCard.tsx
    │   │   │   ├── ListingPrice.tsx
    │   │   │   ├── ListingDetail.tsx          # Détail public, ou vue owner avec OwnerBar
    │   │   │   ├── ListingNotFound.tsx
    │   │   │   ├── OwnerAvatar.tsx
    │   │   │   ├── OwnerBar.tsx               # Vue owner du détail : éditer, photos, publier un brouillon
    │   │   │   ├── ListingForm.tsx            # Création / édition
    │   │   │   ├── CreateListingPage.tsx
    │   │   │   ├── EditListingPage.tsx
    │   │   │   ├── ListingPhotosPage.tsx      # Upload, cover, ordre, suppression des photos
    │   │   │   ├── PhotoTile.tsx
    │   │   │   ├── DeletePhotoModal.tsx
    │   │   │   ├── PublishListingAction.tsx
    │   │   │   ├── DeleteListingAction.tsx
    │   │   │   ├── ListingStatusControl.tsx
    │   │   │   ├── OwnerListings.tsx          # « Mes biens » (GET /users/me/listings)
    │   │   │   └── OwnerListingRow.tsx        # Statut et suppression d'un bien
    │   │   ├── hooks/                 # useListings, useOwnerListings, useCreateListing, useUpdateListing,
    │   │   │                          # useUpdateListingStatus, usePublishListing, useDeleteListing,
    │   │   │                          # useListingPhotoMutations
    │   │   ├── feedFilters.ts
    │   │   ├── labels.ts
    │   │   ├── listingFormValidation.ts
    │   │   ├── listingVisibility.ts
    │   │   ├── listingPublication.ts
    │   │   ├── listingDeletion.ts
    │   │   ├── listingPhotos.ts
    │   │   ├── photoOrder.ts
    │   │   ├── ownerListingsPage.ts
    │   │   ├── ownerListingsCache.ts
    │   │   ├── removeOwnerScopedQueries.ts
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   ├── search/                    # Page /search (MH-64-FE, MH-65, MH-67)
    │   │   ├── components/
    │   │   │   ├── SearchPage.tsx             # Mot-clé, filtres et résultats, tout l'état dans l'URL
    │   │   │   ├── SearchBar.tsx
    │   │   │   ├── FilterPanel.tsx            # Cases à cocher type et statut, ville, quartier, prix
    │   │   │   ├── PriceRangeFilter.tsx       # Curseur double et champs min/max, en XAF
    │   │   │   └── OwnerFilterChip.tsx        # Puce du filtre owner_id, retirable
    │   │   ├── hooks/
    │   │   │   ├── useSearch.ts               # Clé ["listings", "search", params]
    │   │   │   └── useUrlDraft.ts             # Brouillon éditable d'une valeur portée par l'URL
    │   │   ├── searchCriteria.ts              # URL ↔ critères ↔ paramètres d'API
    │   │   ├── priceRange.ts
    │   │   ├── ownerFilter.ts
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   ├── profile/
    │   │   ├── components/
    │   │   │   ├── ProfileForm.tsx
    │   │   │   ├── AvatarUpload.tsx           # Upload / remplacement de l'avatar
    │   │   │   ├── OwnerRequestStatusBlock.tsx
    │   │   │   └── DeleteAccountSection.tsx
    │   │   ├── hooks/                 # useProfile, useUpdateProfile, useUploadAvatar, useDeleteAccount
    │   │   ├── profileValidation.ts
    │   │   ├── avatarValidation.ts
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   ├── owner-request/
    │   │   ├── components/
    │   │   │   ├── OwnerRequestForm.tsx       # Submit unique : phone + identité + documents
    │   │   │   └── OwnerRequestStatus.tsx
    │   │   ├── hooks/                 # useSubmitOwnerRequest, useOwnerRequestStatus
    │   │   ├── ownerRequestValidation.ts
    │   │   ├── ownerRequestStatusView.ts
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   ├── admin/
    │   │   ├── components/
    │   │   │   ├── OwnerRequestQueueList.tsx
    │   │   │   ├── OwnerRequestDetail.tsx
    │   │   │   └── DocumentViewer.tsx
    │   │   ├── hooks/                 # useOwnerRequestQueue, useOwnerRequestDetail,
    │   │   │                          # useOwnerRequestDocument, useReviewOwnerRequest
    │   │   ├── ownerRequestStatusBadge.ts
    │   │   ├── reviewErrorMessage.ts
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   └── contact/                   # api.ts et index.ts vides : révélation du contact pas encore livrée
    │
    └── shared/
        ├── components/
        │   ├── index.ts
        │   ├── Alert.tsx
        │   ├── Badge.tsx
        │   ├── Button.tsx
        │   ├── Card.tsx
        │   ├── Checkbox.tsx           # Case native dans son <label> (MH-64-FE), réutilisée par les filtres
        │   ├── DimensionRule.tsx
        │   ├── EmptyState.tsx
        │   ├── FileDropzone.tsx
        │   ├── FormField.tsx
        │   ├── Input.tsx
        │   ├── Modal.tsx
        │   ├── Pagination.tsx
        │   ├── Select.tsx
        │   ├── SiteFooter.tsx
        │   ├── SiteHeader.tsx
        │   ├── Skeleton.tsx
        │   ├── Spinner.tsx
        │   ├── TextArea.tsx
        │   └── Toast.tsx
        ├── hooks/
        │   ├── useCountdown.ts
        │   ├── useDebouncedValue.ts   # + TEXT_FILTER_DEBOUNCE_MS, partagé par le feed et la recherche
        │   └── usePagination.ts
        ├── api/
        │   ├── client.ts              # Instance fetch (base URL, headers, refresh, paramètres répétés)
        │   ├── constants.ts           # Bornes de champs alignées sur backend/src/shared/validation.rs
        │   └── types.ts               # Auto-généré par openapi-typescript — ne pas éditer (gitignore)
        └── utils/
            ├── cn.ts
            ├── format.ts
            ├── imageFileCheck.ts
            ├── mediaUrl.ts
            └── pagination.ts
```

---

## 2. Modèle de Données

### 2.1 Schéma Relationnel

```
users ──< refresh_tokens
users ──< owner_requests >── users (reviewed_by)
users ──< listings
listings ──< listing_media
```

### 2.2 DDL Complet

```sql
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
    phone       VARCHAR(30),           -- Obligatoire hors admin (contrainte ci-dessous)
    avatar_url  VARCHAR(500),
    is_active   BOOLEAN     NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Migration 20260830120000. Les colonnes restent nullable pour le rôle admin,
-- bootstrappé depuis un email seul. NOT VALID : les comptes email-seul créés
-- par l'ancien flux sont grandfathered.
ALTER TABLE users
  ADD CONSTRAINT users_profile_complete_for_non_admin
  CHECK (role = 'admin' OR (last_name IS NOT NULL AND phone IS NOT NULL))
  NOT VALID;

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
    secondary_phone     VARCHAR(30),                                -- ajouté par MH-45-BE, nullable, propre à la demande (jamais recopié sur users.phone)
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
    price         NUMERIC(12,2)  NOT NULL,   -- Loyer mensuel ; entier en entrée d'API
    currency      CHAR(3)        NOT NULL,   -- ISO 4217 ; toujours 'XAF' au MVP, écrit par le repository
    surface_m2    INTEGER,
    rooms         INTEGER,
    search_vector TSVECTOR,
    published_at  TIMESTAMPTZ,   -- Migration 20260927094513. NULL = brouillon ; visible publiquement seulement si non NULL et au moins une photo
    created_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_listings_search ON listings USING GIN(search_vector);
CREATE INDEX idx_listings_owner  ON listings(owner_id);
CREATE INDEX idx_listings_city   ON listings(city);
CREATE INDEX idx_listings_status ON listings(status);
CREATE INDEX idx_listings_type   ON listings(type);

-- unaccent() est STABLE : il ne peut pas servir dans une expression d'index.
-- Ce wrapper fige le dictionnaire "unaccent" et se déclare IMMUTABLE.
CREATE OR REPLACE FUNCTION immutable_unaccent(text)
    RETURNS text
    LANGUAGE sql
    IMMUTABLE
    PARALLEL SAFE
    STRICT
AS $$
    SELECT unaccent('unaccent', $1)
$$;

-- Filtre city : égalité sans casse ni accents (cf. §4.3). Le filtre neighborhood
-- utilise la même expression, sans index dédié.
CREATE INDEX idx_listings_city_normalized ON listings(lower(immutable_unaccent(city)));

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

-- ============================================================
-- Triggers
-- ============================================================

-- search_vector : mis à jour à chaque INSERT/UPDATE sur listings
-- Inclut le nom de l'owner (pondéré B) pour permettre la recherche par propriétaire,
-- et le libellé du type en français puis en anglais (pondéré B, migration
-- 20261024053122, MH-66) : le mot affiché dans l'interface retrouve le bien.
-- studio et villa s'écrivent pareil dans les deux langues.
CREATE OR REPLACE FUNCTION fn_update_listing_search_vector()
RETURNS TRIGGER AS $$
DECLARE
    v_owner_name TEXT;
    v_type_label TEXT;
BEGIN
    SELECT COALESCE(first_name, '') || ' ' || COALESCE(last_name, '')
        INTO v_owner_name
        FROM users WHERE id = NEW.owner_id;

    v_type_label := CASE NEW.type
        WHEN 'apartment' THEN 'appartement apartment'
        WHEN 'studio'    THEN 'studio'
        WHEN 'house'     THEN 'maison house'
        WHEN 'room'      THEN 'chambre room'
        WHEN 'villa'     THEN 'villa'
        WHEN 'other'     THEN 'autre other'
    END;

    NEW.search_vector :=
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.title, ''))),        'A') ||
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.city, ''))),          'B') ||
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.neighborhood, ''))),  'B') ||
        setweight(to_tsvector('french', unaccent(COALESCE(v_owner_name, ''))),      'B') ||
        setweight(to_tsvector('french', unaccent(COALESCE(v_type_label, ''))),      'B') ||
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.description, ''))),   'C');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tg_listings_search_vector
    BEFORE INSERT OR UPDATE ON listings
    FOR EACH ROW EXECUTE FUNCTION fn_update_listing_search_vector();

-- Cascade : si le nom de l'owner est modifié, invalide les search_vector
-- de ses listings (le trigger tg_listings_search_vector les recalcule au prochain UPDATE)
CREATE OR REPLACE FUNCTION fn_cascade_owner_name_to_listings()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.first_name IS DISTINCT FROM NEW.first_name
    OR OLD.last_name  IS DISTINCT FROM NEW.last_name THEN
        UPDATE listings SET updated_at = NOW()
        WHERE owner_id = NEW.id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tg_users_cascade_search
    AFTER UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION fn_cascade_owner_name_to_listings();

-- updated_at automatique
CREATE OR REPLACE FUNCTION fn_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN NEW.updated_at = NOW(); RETURN NEW; END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tg_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();

CREATE TRIGGER tg_listings_updated_at
    BEFORE UPDATE ON listings
    FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();

CREATE TRIGGER tg_owner_requests_updated_at
    BEFORE UPDATE ON owner_requests
    FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();
```

### 2.3 Notes de Conception

| Décision                                 | Détail                                                                              |
| ----------------------------------------- | ------------------------------------------------------------------------------------ |
| `users.first_name` nullable             | Facultatif — certaines personnes n'ont qu'un nom                                  |
| `users.last_name / phone` nullable en DDL | Rendus obligatoires hors admin par `users_profile_complete_for_non_admin` ; la colonne reste nullable pour le bootstrap admin |
| OTP non stocké en DB                     | Géré exclusivement dans moka (in-memory, TTL 10 min)                               |
| `storage_key` vs `url`                | `storage_key` = chemin interne pérenne. `url` = URL recalculable depuis la clé |
| `is_cover` index partiel unique         | PostgreSQL garantit l'unicité de la cover sans contrainte applicative               |
| `owner_requests` index partiel          | Bloque un second `pending` sans empêcher plusieurs demandes historiques           |
| `identity_documents` JSONB              | Tableau de références storage — accès restreint admin, jamais exposé en statique (cf. ARCHITECTURE.md §7.3) |
| `listings.published_at` nullable        | `NULL` = brouillon. La publication pose la date une fois, sans retour arrière au MVP. Les biens antérieurs à la colonne ont été rétro-remplis avec `created_at` |
| `listings.currency` sans défaut         | Le repository écrit toujours `XAF` ; l'API ne l'expose pas, tous les prix sont en XAF au MVP |
| Libellés de type dans `search_vector`  | Français et anglais, pour que « apartment » comme « appartement » trouvent le bien. Changer la liste demande une migration qui réindexe |
| `refresh_tokens.revoked_at` / `replaced_by_id` | Supportent la rotation : un refresh consomme le token courant et chaîne vers le nouveau. Réutilisation d'un token `revoked_at IS NOT NULL` ⇒ révocation de tous les tokens du `user_id` |

---

## 3. Abstraction Storage

### 3.1 Trait `StorageProvider`

```
infra/storage/
├── provider.rs     # trait StorageProvider
├── local_fs.rs     # impl LocalFsStorage  (STORAGE_PROVIDER=local — MVP)
└── mod.rs          # fn build_storage_provider() → Arc<dyn StorageProvider>
                     # AwsS3Storage (STORAGE_PROVIDER=s3) ajouté en V2
```

**Interface :**

```rust
// provider.rs
#[async_trait]
pub trait StorageProvider: Send + Sync {
    async fn upload(&self, key: &str, data: Bytes, content_type: &str) -> Result<String, AppError>;
    async fn read(&self, key: &str) -> Result<Bytes, AppError>;        // requis pour les documents privés (owner_requests)
    async fn delete(&self, key: &str) -> Result<(), AppError>;
    async fn presigned_url(&self, key: &str, expires_in: Duration) -> Result<String, AppError>; // non implémenté pour LocalFsStorage (MVP)
}
```

**Convention de nommage des clés :**

```
listings/{listing_id}/{uuid}.{ext}        — public, servi en statique par nginx
avatars/{user_id}/{uuid}.{ext}            — public, servi en statique par nginx
owner-requests/{request_id}/{uuid}.{ext}  — privé, jamais exposé en statique (lecture via read(), admin uniquement)
```

**`LocalFsStorage` (MVP) :** écrit sous `LOCAL_STORAGE_PATH` (volume Docker `storage_data`). Le nom de fichier est toujours généré côté serveur (UUID) — jamais dérivé du nom fourni par le client, pour éliminer tout risque de path traversal. `upload()` retourne une URL calculée à partir de `PUBLIC_MEDIA_BASE_URL` + clé pour les préfixes publics ; pour `owner-requests/*`, aucune URL publique n'est générée — seule la clé est stockée, lue via `read()`.

**Injection :**
`Arc<dyn StorageProvider>` est stocké dans `AppState` et injecté dans les handlers via l'extracteur Axum `State<AppState>`.

**Appelants :** trois services écrivent dans le storage, chacun pour son propre domaine. Aucun ne passe par un autre module ; ils partagent deux helpers de `shared/`.

| Upload | Service | Validation | Clé |
| ------ | ------- | ---------- | --- |
| Photo de listing (`POST /media/upload`) | `media::service::upload` | `validate_image`, 5 MB | `listing_media_key` |
| Avatar (`POST /users/me/avatar`) | `users::service::replace_avatar` | `validate_image`, 5 MB | `avatar_key` |
| Pièces d'identité (`POST /owner-requests`) | `owner_requests::service::submit` | Un PDF (`validate_pdf`, 3 MB) ou deux images recto/verso (`validate_image`, 5 MB chacune) | `owner_request_document_key` |

`shared::file_validation` contrôle les magic bytes (crate `infer`) et la taille. `shared::storage_key` est le seul endroit qui construit une clé : il impose le préfixe public ou privé et génère le nom de fichier (UUID). `media::service` ne gère que les photos de listing, et reste le seul module qui écrit dans `listing_media`.

L'ordre des écritures protège la base contre une URL morte. L'avatar est écrit, puis `users.avatar_url` pointe dessus, puis l'ancien fichier est supprimé en best-effort. Pour une demande owner, toutes les pièces sont validées en mémoire, puis envoyées au storage, puis la ligne `owner_requests` est insérée ; un échec à l'une des deux dernières étapes supprime les pièces déjà envoyées.

---

## 3bis. Stack de Notification

Vue bout-en-bout du chemin d'une notification, de l'événement déclencheur jusqu'à la boîte mail de l'utilisateur. Cette section consolide ce qui était implicite dans `modules/notifications/` et clarifie ce qui est **in** vs **hors** périmètre MVP.

### 3bis.1 Chaîne technique (MVP)

```
Événement métier (ex: owner_request créée)
   → Handler du module concerné (ex: users::handler)
   → Service du module (ex: users::service) déclenche l'envoi
   → notifications::service::send_xxx_email(...)
   → Rendu du template HTML (modules/notifications/templates/*.html)
   → infra/mailer.rs (client SMTP — crate lettre)
   → Relais SMTP configuré (SMTP_HOST / SMTP_PORT / SMTP_SECURITY / SMTP_USERNAME / SMTP_PASSWORD / SMTP_FROM)
   → Boîte mail du destinataire
```

**Caractéristiques MVP :**

| Aspect | Détail |
| --- | --- |
| Transport | SMTP uniquement (`lettre`), vers n'importe quel relais conforme aux standards. Authentification (`SMTP_USERNAME`/`SMTP_PASSWORD`) et chiffrement (`SMTP_SECURITY` : `none` \| `starttls` \| `tls`) inclus au MVP (MH-113). Changer de relais ne demande qu'un changement de variables d'environnement. Identifiants obligatoires en staging/production, et alors `SMTP_SECURITY` ≠ `none`. Pas de SDK fournisseur ni d'API transactionnelle HTTP (cf. §3bis.3, R-03 ARCHITECTURE.md) |
| Synchronicité | Envoi **synchrone** dans le flux de la requête HTTP qui le déclenche — pas de file d'attente, pas de retry automatique |
| Échec d'envoi | Un échec SMTP **ne fait pas échouer** la requête métier (ex: `POST /owner-requests` reste `201` même si l'email de notification admin échoue) — l'envoi est best-effort, journalisé via `tracing`, jamais bloquant pour l'écriture DB |
| Notification in-app | **Aucune** au MVP — pas de table `notifications`, pas d'endpoint de liste, pas de badge. Le seul canal est l'email. L'admin consulte l'état des demandes directement via `GET /admin/owner-requests?status=pending` (pas de flux de notifications séparé) |
| Rappel planifié (LIST-03) | **Hors scope MVP** — cf. ARCHITECTURE.md §6.2 et R-10. Aucune tâche planifiée n'existe dans le binaire backend au MVP |

### 3bis.2 Événements déclencheurs (MVP)

| Événement | Déclenché par | Destinataire | Template |
| --- | --- | --- | --- |
| Envoi OTP | `POST /auth/otp/request` | Utilisateur | `otp.html` |
| Bienvenue (nouveau compte) | `POST /auth/register` (après COMMIT)      | Utilisateur | `welcome.html` |
| Nouvelle demande owner | `POST /owner-requests` | Admin (email fixe, configuré) | `owner_request_received.html` |
| Demande owner approuvée | `PATCH /admin/owner-requests/:id` (approved) | Utilisateur | `owner_request_approved.html` |
| Demande owner rejetée | `PATCH /admin/owner-requests/:id` (rejected) | Utilisateur | `owner_request_rejected.html` |

**Note — email admin fixe :** l'admin étant unique au MVP (cf. ARCHITECTURE.md §8.1), l'adresse destinataire des notifications admin (`owner_request_received.html`) est une variable de configuration (`ADMIN_NOTIFICATION_EMAIL`) plutôt qu'une requête dynamique sur la table `users` — il n'existe qu'un seul destinataire possible. À faire évoluer en V2 avec le rôle superviseur (notification à tous les superviseurs actifs).

### 3bis.3 Hors scope MVP (déféré V2)

- Notifications in-app (table, endpoint, badge de lecture)
- Notifications push (FCM/APNs)
- File d'attente / retry automatique sur échec d'envoi
- Rappel mensuel de disponibilité (LIST-03) — tâche planifiée
- SDK fournisseur ou API transactionnelle HTTP (SES/Postmark) — SMTP direct au MVP, relais authentifié/TLS compris (cf. §3bis.1)

---

## 4. Contrat API

### Préfixe global : `/api/v1`

**Format de réponse — Succès :**

```json
{ "data": { ... } }
{ "data": [...], "pagination": { "page": 1, "per_page": 20, "total": 143, "total_pages": 8 } }
```

**Format de réponse — Erreur :**

```json
{ "error": { "code": "LISTING_NOT_FOUND", "message": "...", "status": 404 } }
```

---

### 4.0 Health Check

| Méthode | Endpoint  | Auth | Description                                          |
| -------- | --------- | ---- | ----------------------------------------------------- |
| `GET`  | `/health` | Non  | Hors `/api/v1`. Vérifie la connectivité PostgreSQL. |

```json
// Response 200
{ "status": "ok" }

// Response 503 — DB inaccessible
{ "status": "unavailable" }
```

---

### 4.1 Auth

| Méthode | Endpoint              | Auth | Description                                                                                                   |
| -------- | --------------------- | ---- | ------------------------------------------------------------------------------------------------------------- |
| `POST` | `/auth/otp/request` | Non  | Envoie un OTP. Gère login et inscription selon si l'email est connu.                                         |
| `POST` | `/auth/otp/verify`  | Non  | Vérifie le code. **Ne crée aucun compte.** Email connu → `{ is_new_user: false, access_token }` + cookie `refresh_token`. Email inconnu → `{ is_new_user: true, registration_ticket }`, sans cookie. |
| `POST` | `/auth/register`    | Non  | Consomme le `registration_ticket` (usage unique, TTL 10 min) et crée `users` + `refresh_tokens` en **une transaction**. Retourne `{ access_token }` + cookie `refresh_token`. |
| `POST` | `/auth/refresh`     | Cookie | Lit le refresh token du cookie, **révoque l'ancien et en émet un nouveau** (rotation). Renvoie `{ access_token }` + cookie mis à jour. Réutilisation d'un token déjà révoqué → 401 et révocation de toute la famille. |
| `POST` | `/auth/logout`      | JWT  | Révoque le refresh token courant et efface le cookie.                                                          |

**`POST /auth/otp/request`**

```json
// Request
{ "email": "user@example.com" }

// Response 200
{ "data": { "message": "Code envoyé" } }

// Response 429 — rate limit
{ "error": { "code": "OTP_RATE_LIMITED", "message": "Veuillez attendre avant de demander un nouveau code.", "status": 429 } }
```

**`POST /auth/otp/verify`**

```json
// Request
{ "email": "user@example.com", "code": "482910" }

// Response 200 — email connu
// Set-Cookie: refresh_token=...; HttpOnly; Secure; SameSite=Strict; Max-Age=2592000
{
  "data": {
    "is_new_user": false,
    "access_token": "eyJ...",
    "registration_ticket": null
  }
}

// Response 200 — email inconnu : aucune ligne `users`, aucun cookie
{
  "data": {
    "is_new_user": true,
    "access_token": null,
    "registration_ticket": "a3f1c8e2-..."
  }
}

// Response 401 — code invalide ou expiré
{ "error": { "code": "OTP_INVALID", "message": "Code invalide ou expiré.", "status": 401 } }
```

**`POST /auth/register`**

L'email n'est jamais lu dans le body : il provient du ticket, donc de l'OTP
vérifié. Obligations par rôle sur les champs de profil :

| Rôle | `last_name` | `phone` | `first_name` |
| ---- | ----------- | ------- | ------------ |
| `seeker`, `owner` | obligatoire | obligatoire | facultatif |
| `admin` | nullable | nullable | facultatif |

Invariant garanti en base par la contrainte `users_profile_complete_for_non_admin`
(migration `20260830120000`), et bornes `MAX_NAME_LENGTH = 100` /
`MAX_PHONE_LENGTH = 30`.

```json
// Request
{
  "registration_ticket": "a3f1c8e2-...",
  "first_name": "Moussa",
  "last_name": "Diallo",
  "phone": "+225 07 00 00 00 00"
}

// Response 200
// Set-Cookie: refresh_token=...; HttpOnly; Secure; SameSite=Strict; Max-Age=2592000
{ "data": { "access_token": "eyJ..." } }

// Response 401 — ticket inconnu, expiré ou déjà consommé
{ "error": { "code": "OTP_INVALID", "message": "Code invalide ou expiré.", "status": 401 } }

// Response 409 — l'email du ticket a été pris entre-temps
{ "error": { "code": "EMAIL_ALREADY_EXISTS", "message": "Ce compte existe déjà.", "status": 409 } }
```

---

### 4.2 Users

| Méthode   | Endpoint                    | Auth | Rôle      | Description                       |
| ---------- | --------------------------- | ---- | ---------- | ---------------------------------- |
| `GET`    | `/users/me`               | JWT  | tous       | Profil de l'utilisateur connecté |
| `PUT`    | `/users/me`               | JWT  | tous       | Mise à jour du profil            |
| `DELETE` | `/users/me`               | JWT  | tous       | Suppression du compte — cascade DB + nettoyage storage (listings, médias, avatar) |
| `POST`   | `/users/me/avatar`        | JWT  | tous       | Upload / remplacement de l'avatar — l'ancien fichier est supprimé du storage |
| `GET`    | `/users/me/owner-request` | JWT  | tous¹      | Statut de la demande en cours     |

¹ **Élargissement de rôle (MH-46-BE), définitif :** l'endpoint n'impose aucun filtre de rôle —
tout utilisateur authentifié peut lire sa propre demande, y compris `owner`/`admin` (utile après
approbation/rejet, ou pour un admin qui aurait aussi déposé une demande). Le contrat d'origine
ne visait que `seeker` ; cet élargissement le remplace.

**`GET /users/me` — Response 200**

```json
{
  "data": {
    "id": "uuid",
    "email": "user@example.com",
    "role": "seeker",
    "first_name": "Moussa",
    "last_name": "Diallo",
    "phone": null,
    "avatar_url": null,
    "is_active": true,
    "created_at": "2025-06-01T10:00:00Z"
  }
}
```

**`PUT /users/me`** — édition d'un profil déjà complet, jamais une complétion :
`last_name` et `phone` sont requis (mêmes bornes que `/auth/register`),
`first_name` facultatif. Un `phone` omis est un 400, pas un effacement.

```json
// Request
{ "first_name": "Moussa", "last_name": "Diallo", "phone": "+225 07 00 00 00 00" }

// Response 404 — le compte n'existe plus
{ "error": { "code": "USER_NOT_FOUND", "message": "User not found.", "status": 404 } }
```

**`POST /users/me/avatar`**

```
Content-Type: multipart/form-data
Body: { file: <binary> }
```

Mêmes contraintes de validation que les photos de listing (magic bytes, formats JPEG/PNG/WebP, taille max 5 MB). Stocké sous `avatars/{user_id}/{uuid}.{ext}`. L'ancien avatar est supprimé du storage après écriture réussie du nouveau.

```json
// Response 200
{ "data": { "avatar_url": "https://..." } }
```

**`GET /users/me/owner-request` — Response 200**

`admin_note` n'est renseigné que si `status` vaut `rejected`. `reviewed_at` et `admin_note`
sont `null` tant que la demande n'a pas été traitée par un admin.

```json
// Une demande existe
{
  "data": {
    "id": "uuid",
    "status": "pending",
    "created_at": "2026-09-01T10:00:00Z",
    "reviewed_at": null,
    "admin_note": null
  }
}

// Aucune demande déposée — pas de 404
{ "data": null }
```

---

### 4.2bis Owner Requests

| Méthode | Endpoint           | Auth | Rôle      | Description                                          |
| -------- | ------------------ | ---- | ---------- | ----------------------------------------------------- |
| `POST` | `/owner-requests` | JWT  | `seeker` | Soumet la demande complète en un seul appel (atomique) |

**`POST /owner-requests`**

```
Content-Type: multipart/form-data
Body: {
  phone: string,
  secondary_phone?: string,        // ajouté par MH-45-BE — écrit sur owner_requests uniquement, jamais sur users.phone
  identity_data: <JSON string — { full_name, id_type, id_number }>,
  documents: <file[]>
}
```

Soumission atomique : si l'upload d'un document échoue, toute la requête échoue — aucun état intermédiaire n'est persisté (pas de `draft`). Les documents sont stockés sous `owner-requests/{request_id}/{uuid}.{ext}`, jamais exposés en statique — accès admin uniquement via `GET /admin/owner-requests/:id/documents/:doc_id` (cf. ARCHITECTURE.md §7.3).

```json
// Response 201
{ "data": { "id": "uuid", "status": "pending", "created_at": "..." } }

// Response 409 — demande déjà en cours
{ "error": { "code": "OWNER_REQUEST_ALREADY_PENDING", "status": 409 } }

// Response 422 — document invalide (format/taille)
{ "error": { "code": "INVALID_DOCUMENT", "status": 422 } }
```

---

### 4.3 Listings

| Méthode   | Endpoint                 | Auth | Rôle     | Description                                  |
| ---------- | ------------------------ | ---- | --------- | -------------------------------------------- |
| `GET`    | `/listings`            | Non  | —        | Feed public paginé (cover photo + résumé) |
| `GET`    | `/listings/:id`        | Optionnelle | —        | Détail complet + owner info (sans phone)    |
| `POST`   | `/listings`            | JWT  | `owner` | Créer un bien (brouillon)                   |
| `PUT`    | `/listings/:id`        | JWT  | `owner` | Modifier son bien                            |
| `DELETE` | `/listings/:id`        | JWT  | `owner` | Supprimer son bien et ses photos             |
| `GET`    | `/users/me/listings`   | JWT  | `owner` | Ses propres biens, brouillons compris        |
| `PATCH`  | `/listings/:id/status` | JWT  | `owner` | Changer le statut                            |
| `POST`   | `/listings/:id/publish` | JWT | `owner` | Publier un brouillon                         |
| `PATCH`  | `/listings/:id/cover`  | JWT  | `owner` | Définir la photo de couverture              |
| `PUT`    | `/listings/:id/media/order` | JWT | `owner` | Réordonner les photos                  |

`PATCH /listings/:id/cover` et `PUT /listings/:id/media/order` sont servis par le module `media`, seul à écrire dans `listing_media`. `GET /users/me/listings` est servi par `listings`.

**Règle de visibilité publique** : un bien est visible publiquement si et seulement si `published_at IS NOT NULL` et il a au moins une photo (une ligne de couverture). Elle s'applique au feed, à son total, au filtre `owner_id`, à la recherche (§4.4) et au détail. Sur `GET /listings/:id`, l'authentification est optionnelle : le propriétaire voit toujours son bien, quel que soit son état ; tout autre appelant (anonyme, seeker, admin, autre owner) reçoit un `404 LISTING_NOT_FOUND` identique à celui d'un id inconnu. Un en-tête `Authorization` absent est traité comme anonyme ; un en-tête présent mais mal formé, invalide, expiré, ou un compte suspendu, renvoie `401` (jamais un repli sur l'anonyme).

**Prix** : `price` est le loyer mensuel, en XAF, sans centimes. En entrée (`POST`, `PUT`, `price_min`, `price_max`), c'est un entier. En sortie, c'est un nombre JSON à virgule (`150000.0`), lu depuis `price::float8`. La devise n'apparaît pas dans l'API : la colonne `currency` vaut toujours `XAF` au MVP (§2.2).

**Statut et publication** : `status` (`available` / `unavailable`) et `published_at` sont indépendants. Le feed montre les biens publics des deux statuts ; le filtre `status` restreint à l'un des deux. Changer le statut ne publie ni ne dépublie rien.

**`GET /listings` — Query params**

| Paramètre   | Type        | Description                                 |
| ------------ | ----------- | ------------------------------------------- |
| `owner_id` | `uuid`    | Filtrer par propriétaire                   |
| `city`     | `string`  | Filtre par ville                            |
| `neighborhood` | `string` | Filtre par quartier                    |
| `type`     | `enum`, répétable | `apartment\|studio\|house\|room\|villa\|other`. `type=apartment&type=studio` garde les biens de l'un ou l'autre type ; absent : tous |
| `status`   | `enum`    | `available\|unavailable` ; absent : les deux |
| `page`     | `integer` | Défaut : 1                                 |
| `per_page` | `integer` | Défaut : 20, max : 50                      |

`city` et `neighborhood` sont normalisés comme à l'enregistrement (trim, espaces internes réduits à un seul), puis comparés à la valeur entière, sans tenir compte de la casse ni des accents : « yaounde » trouve « Yaoundé », « Bonap » ne trouve pas « Bonapriso ». Une valeur vide après normalisation est ignorée. Un caractère NUL, une valeur `type` (y compris l'une des valeurs répétées) ou `status` inconnue, ou un paramètre mal formé renvoient `400 INVALID_QUERY_PARAM`.

Le feed est trié du plus récent au plus ancien (`created_at` décroissant, puis `id` décroissant).

**`GET /listings` — Response 200**

```json
{
  "data": [
    {
      "id": "uuid",
      "title": "Studio meublé Bonapriso",
      "type": "studio",
      "status": "available",
      "city": "Douala",
      "neighborhood": "Bonapriso",
      "price": 150000.0,
      "cover_photo_url": "https://...",
      "published_at": "2025-06-02T09:00:00Z",
      "owner": { "id": "uuid", "first_name": "Moussa", "last_name": "Diallo" }
    }
  ],
  "pagination": { "page": 1, "per_page": 20, "total": 47, "total_pages": 3 }
}
```

`published_at` n'est jamais `null` dans le feed (règle de visibilité publique).

**`GET /users/me/listings` — Query params** : `page` et `per_page`, avec les mêmes défauts et bornes que le feed. Le propriétaire est toujours l'appelant ; aucun paramètre ne permet de le changer.

**`GET /users/me/listings` — Response 200**

Tous les biens de l'appelant, sans règle de visibilité publique : brouillons et biens sans photo compris, quel que soit le statut, triés par `created_at` décroissant. Même forme qu'un élément du feed ; `published_at` vaut `null` pour un brouillon et `cover_photo_url` vaut `null` pour un bien sans photo. `403` pour un seeker ou un admin, `401` sans jeton.

```json
{
  "data": [
    {
      "id": "uuid",
      "title": "Studio meublé Bonapriso",
      "type": "studio",
      "status": "available",
      "city": "Douala",
      "neighborhood": "Bonapriso",
      "price": 150000.0,
      "cover_photo_url": null,
      "published_at": null,
      "owner": { "id": "uuid", "first_name": "Moussa", "last_name": "Diallo" }
    }
  ],
  "pagination": { "page": 1, "per_page": 20, "total": 1, "total_pages": 1 }
}
```

**`GET /listings/:id` — Response 200**

```json
{
  "data": {
    "id": "uuid",
    "title": "Studio meublé Bonapriso",
    "description": "...",
    "type": "studio",
    "status": "available",
    "city": "Douala",
    "neighborhood": "Bonapriso",
    "price": 150000.0,
    "surface_m2": 35,
    "rooms": 1,
    "media": [
      { "id": "uuid", "url": "https://...", "is_cover": true, "position": 0 },
      { "id": "uuid", "url": "https://...", "is_cover": false, "position": 1 }
    ],
    "owner": {
      "id": "uuid",
      "first_name": "Moussa",
      "last_name": "Diallo",
      "avatar_url": null
    },
    "created_at": "2025-06-01T10:00:00Z",
    "published_at": null
  }
}
```

**`POST /listings`**

Crée un brouillon de l'appelant : `status` vaut `available`, `published_at` vaut `null`. Le bien reste invisible du public jusqu'à `POST /listings/:id/publish`. Response `201` : même forme que `GET /listings/:id`.

```json
// Request
{
  "title": "Studio meublé Bonapriso",
  "description": "...",
  "type": "studio",
  "price": 150000,
  "city": "Douala",
  "neighborhood": "Bonapriso",
  "surface_m2": 35,
  "rooms": 1
}
```

**Règles de validation** (communes à `POST` et `PUT`) :

| Champ | Requis | Règle |
| ----- | ------ | ----- |
| `title` | oui | 5 à 120 caractères après trim |
| `description` | oui | 20 à 2 000 caractères après trim |
| `type` | oui | `apartment`, `studio`, `house`, `room`, `villa` ou `other` |
| `price` | oui | Entier, 1 à 9 999 999 999 (loyer mensuel en XAF) |
| `city` | oui | 100 caractères max ; trim, espaces internes réduits à un seul ; vide refusé |
| `neighborhood` | oui | Mêmes règles que `city` |
| `surface_m2` | non | Entier, 1 à 100 000 |
| `rooms` | non | Entier, 0 à 100 |

Un caractère NUL dans un champ texte est refusé. Toutes les violations sont collectées avant de répondre : un seul `422 VALIDATION_FAILED` liste chaque champ fautif dans `error.fields`. Un champ requis absent est un `422`, pas un `400`. Un JSON mal formé, ou une valeur du mauvais type JSON (un `price` décimal ou en chaîne, par exemple), donne `400`. `owner_id`, `status`, `published_at` et `created_at` présents dans le corps sont ignorés.

```json
// Response 422
{
  "error": {
    "code": "VALIDATION_FAILED",
    "message": "...",
    "status": 422,
    "fields": [
      { "field": "title", "message": "title must be between 5 and 120 characters." },
      { "field": "price", "message": "price must be between 1 and 9999999999." }
    ]
  }
}
```

| Code    | Cas                                                                                  |
| ------- | ------------------------------------------------------------------------------------ |
| `201` | Brouillon créé                                                                        |
| `400` | Corps JSON mal formé                                                                 |
| `401` | Jeton absent ou invalide                                                              |
| `403` | Appelant seeker ou admin                                                              |
| `422` | `VALIDATION_FAILED`, une entrée par champ invalide dans `error.fields`               |

**`PUT /listings/:id`**

Remplacement complet des champs éditables : même corps, même validation et même normalisation que `POST /listings`. Un `surface_m2` ou `rooms` absent est remis à `null`. `status`, `published_at`, `owner_id` et `created_at` ne sont jamais modifiés ; présents dans le corps, ils sont ignorés. Le contrôle de propriété et l'écriture se font dans un seul `UPDATE … WHERE id = $1 AND owner_id = $2`.

```json
// Request
{
  "title": "Studio meublé Bonapriso",
  "description": "...",
  "type": "studio",
  "price": 150000,
  "city": "Douala",
  "neighborhood": "Bonapriso",
  "surface_m2": 35,
  "rooms": 1
}
```

Response 200 : même forme que `GET /listings/:id`, brouillon compris (lecture côté propriétaire).

| Code    | Cas                                                                                  |
| ------- | ------------------------------------------------------------------------------------ |
| `200` | Bien mis à jour                                                                       |
| `400` | Corps JSON mal formé                                                                 |
| `401` | Jeton absent ou invalide                                                              |
| `403` | Appelant seeker ou admin                                                              |
| `404` | `LISTING_NOT_FOUND` : id inconnu ou bien d'un autre owner (même réponse, jamais `403`) |
| `422` | `VALIDATION_FAILED`, une entrée par champ invalide dans `error.fields`               |

**`POST /listings/:id/publish`**

Pas de corps. Sur un brouillon qui a au moins une photo, pose `published_at` et répond `200` avec la même forme que `GET /listings/:id`. Un bien déjà publié répond aussi `200` et garde son `published_at` d'origine. Aucun autre champ n'est modifié. La publication est irréversible au MVP.

| Code    | Cas                                                                                  |
| ------- | ------------------------------------------------------------------------------------ |
| `200` | Bien publié, ou déjà publié                                                           |
| `401` | Jeton absent ou invalide                                                              |
| `403` | Appelant seeker ou admin                                                              |
| `404` | `LISTING_NOT_FOUND` : id inconnu ou bien d'un autre owner (même réponse, jamais `403`) |
| `409` | `PHOTO_REQUIRED` : brouillon sans photo, qui reste un brouillon                       |

**`PATCH /listings/:id/status`**

Fonctionne aussi sur un brouillon. `published_at` n'est jamais modifié. Redemander le statut courant répond aussi `200`.

```json
// Request
{ "status": "unavailable" }

// Response 200
{ "data": { "id": "uuid", "status": "unavailable" } }
```

| Code    | Cas                                                                                  |
| ------- | ------------------------------------------------------------------------------------ |
| `200` | Statut posé                                                                           |
| `400` | Corps JSON mal formé                                                                 |
| `401` | Jeton absent ou invalide                                                              |
| `403` | Appelant seeker ou admin                                                              |
| `404` | `LISTING_NOT_FOUND` : id inconnu ou bien d'un autre owner (même réponse, jamais `403`) |
| `422` | `VALIDATION_FAILED` sur `status` : absent, ou autre que `available` / `unavailable`  |

**`DELETE /listings/:id`**

Supprime définitivement le bien et ses photos. Response `204`, sans corps. La ligne est verrouillée comme pour un upload, puis supprimée (cascade sur `listing_media`) et commitée ; les fichiers sont supprimés ensuite en best-effort. Une panne du storage ne peut laisser qu'un fichier orphelin (cf. ARCHITECTURE.md §8.1).

| Code    | Cas                                                                                  |
| ------- | ------------------------------------------------------------------------------------ |
| `204` | Bien et photos supprimés                                                              |
| `401` | Jeton absent ou invalide                                                              |
| `403` | Appelant seeker ou admin                                                              |
| `404` | `LISTING_NOT_FOUND` : id inconnu, déjà supprimé, ou bien d'un autre owner             |

**`PATCH /listings/:id/cover`**

```json
// Request
{ "media_id": "uuid" }

// Response 200 — la nouvelle cover
{ "data": { "id": "uuid", "url": "https://...", "is_cover": true, "position": 1 } }
```

L'ancienne cover est rétrogradée et la nouvelle promue dans une seule transaction, sous le verrou du listing. Désigner la cover actuelle répond aussi `200`, sans écriture.

Écart connu : le corps de cette route est lu par l'extracteur `Json` d'axum, pas par `AppJson`. Un corps invalide est donc rejeté avant le handler, sans l'enveloppe `error` : `400` (JSON mal formé), `415` (`Content-Type` absent) ou `422` (`media_id` absent ou UUID invalide), en texte brut. Les autres routes à corps JSON des biens et des photos passent par `AppJson`.

| Code    | Cas                                                                                  |
| ------- | ------------------------------------------------------------------------------------ |
| `200` | Cover changée                                                                         |
| `401` | Jeton absent ou invalide                                                              |
| `403` | Appelant seeker ou admin                                                              |
| `404` | `LISTING_NOT_FOUND` (bien inconnu ou d'un autre owner) ou `MEDIA_NOT_FOUND` (photo absente de ce bien) |

**`PUT /listings/:id/media/order`**

`media_ids` donne l'ordre voulu et doit contenir exactement les photos actuelles du bien. Les positions sont réécrites de 0 à n−1 dans l'ordre envoyé, dans une seule transaction : une erreur ne change aucune position. La réponse liste les photos dans l'ordre d'affichage (§4.5), la cover d'abord.

```json
// Request
{ "media_ids": ["uuid-2", "uuid-1", "uuid-3"] }

// Response 200
{
  "data": [
    { "id": "uuid-1", "url": "https://...", "is_cover": true,  "position": 1 },
    { "id": "uuid-2", "url": "https://...", "is_cover": false, "position": 0 },
    { "id": "uuid-3", "url": "https://...", "is_cover": false, "position": 2 }
  ]
}
```

| Code    | Cas                                                                                  |
| ------- | ------------------------------------------------------------------------------------ |
| `200` | Ordre enregistré                                                                      |
| `400` | Corps JSON mal formé, y compris un UUID invalide                                      |
| `401` | Jeton absent ou invalide                                                              |
| `403` | Appelant seeker ou admin                                                              |
| `404` | `LISTING_NOT_FOUND` : id inconnu ou bien d'un autre owner                              |
| `409` | `MEDIA_ORDER_CONFLICT` : les ids ne correspondent pas aux photos actuelles du bien     |
| `422` | `VALIDATION_FAILED` sur `media_ids` : absent, vide, ou avec un doublon               |

---

### 4.4 Search

| Méthode | Endpoint    | Auth | Description                   |
| -------- | ----------- | ---- | ----------------------------- |
| `GET`  | `/search` | Non  | Mots-clés + filtres combinés |

Servi par le module `listings` (tag OpenAPI `listings`), pas par un module dédié (cf. ARCHITECTURE.md ADR-12). Le feed et la recherche passent par les deux mêmes requêtes SQL (comptage et page) : ils partagent la règle de visibilité publique (§4.3), les filtres et la pagination, et `total` reste aligné sur les pages. Réponse : même enveloppe et mêmes éléments que `GET /listings`.

**Query params**

| Paramètre    | Type        | Description                                       |
| ------------- | ----------- | ------------------------------------------------- |
| `q`         | `string`  | Mots-clés, 100 caractères max après trim. Vide ou absent : pas de recherche par mots-clés |
| `owner_id`  | `uuid`    | Filtrer par propriétaire                         |
| `city`      | `string`  | Filtre par ville, mêmes règles que le feed (§4.3) |
| `neighborhood` | `string` | Filtre par quartier, mêmes règles que le feed |
| `type`      | `enum`, répétable | Comme le feed : un bien de l'un des types donnés |
| `status`    | `enum`    | `available\|unavailable` ; absent : les deux     |
| `price_min` | `integer` | Loyer mensuel minimum en XAF, inclus, 0 ou plus  |
| `price_max` | `integer` | Loyer mensuel maximum en XAF, inclus, 0 ou plus, pas sous `price_min` |
| `page`      | `integer` | Défaut : 1                                       |
| `per_page`  | `integer` | Défaut : 20, max : 50                            |

Chaque borne de prix peut venir seule. Tous les filtres se combinent (ET).

**Correspondance des mots-clés** : `q` passe par `websearch_to_tsquery('french', unaccent(q))` et se compare à `search_vector` (§2.2) : titre, ville, quartier, nom de l'owner, libellé du type en français et en anglais, description. La casse et les accents sont ignorés, et la racinisation française s'applique (« appartements » trouve « appartement »). Seuls les mots entiers correspondent : pas de préfixe ni de recherche floue. La syntaxe web est acceptée : `"phrase exacte"`, `OR`, `-exclu`. Une syntaxe mal formée (guillemet non fermé, `OR` isolé) ne lève jamais d'erreur. Une requête faite uniquement de mots vides (« le de ») ne trouve rien : `200` avec une liste vide et `total: 0`.

**Tri** : avec `q`, par palier de pertinence, puis du plus récent au plus ancien (`created_at` décroissant, puis `id` décroissant).

| Palier | Le mot-clé correspond dans |
| ------ | -------------------------- |
| 1 | Le titre (poids A) |
| 2 | La ville, le quartier, le nom de l'owner ou le libellé du type (poids B) |
| 3 | La description seulement (poids C) |

Sans `q`, l'ordre est celui du feed.

**Erreurs** : `400 INVALID_QUERY_PARAM` pour un `q` de plus de 100 caractères, un `type` ou un `status` inconnu, un `owner_id` mal formé, une borne de prix non entière, négative ou inversée (`price_min > price_max`), ou un caractère NUL dans `q` ou un filtre de lieu.

**Performance** : `search_vector` porte l'index GIN `idx_listings_search`. Le filtre de prix n'a pas d'index (ARCHITECTURE.md R-07).

---

### 4.5 Media

| Méthode   | Endpoint          | Auth | Rôle     | Description        |
| ---------- | ----------------- | ---- | --------- | ------------------ |
| `POST`   | `/media/upload` | JWT  | `owner` | Upload une photo   |
| `DELETE` | `/media/:id`    | JWT  | `owner` | Supprime un média |

**`POST /media/upload`**

```
Content-Type: multipart/form-data
Body: { listing_id: uuid, file: <binary> }
```

Contraintes : formats JPEG / PNG / WebP validés par **magic bytes** (crate `infer`, pas par extension ni `Content-Type` déclaré), taille max 5 MB, max 5 photos par listing. Le nom de fichier stocké est toujours généré côté serveur (UUID) — jamais dérivé du nom fourni par le client.

**Sélection automatique de la cover :** la première photo uploadée sur un listing (`listing_media` vide pour ce `listing_id`) est automatiquement marquée `is_cover = true`. Les uploads suivants sont `is_cover = false` par défaut. L'owner peut ensuite désigner une autre photo existante comme cover via `PATCH /listings/:id/cover` (§4.3) — aucune action manuelle n'est requise pour qu'un listing ait une cover dès sa première photo.
La cover ne peut pas être supprimée sans en avoir désigné une autre au préalable.

**Ordre d'affichage :** la cover d'abord, puis `position` croissante, puis la date d'upload, puis `id`. `PUT /listings/:id/media/order` réécrit les positions de 0 à n−1 dans l'ordre envoyé ; un upload prend la position qui suit la plus haute existante.

```json
// Response 201 — première photo du listing (cover automatique)
{ "data": { "id": "uuid", "url": "https://...", "is_cover": true, "position": 0 } }

// Response 201 — photo suivante
{ "data": { "id": "uuid", "url": "https://...", "is_cover": false, "position": 2 } }
```

---

### 4.6 Contact

| Méthode | Endpoint                  | Auth | Rôle                 | Description                        |
| -------- | ------------------------- | ---- | --------------------- | ---------------------------------- |
| `GET`  | `/listings/:id/contact` | JWT  | `seeker` ou `owner` | Révèle le téléphone de l'owner |

```json
// Response 200
{ "data": { "phone": "+237 6 77 12 34 56" } }

// Response 404 — listing inexistant ou inactif
{ "error": { "code": "LISTING_NOT_FOUND", "status": 404 } }
```

---

### 4.7 Admin

| Méthode   | Endpoint                      | Auth            | Description                                                |
| ---------- | ----------------------------- | --------------- | ---------------------------------------------------------- |
| `GET`    | `/admin/listings`           | JWT (`admin`) | Liste tous les biens. Query :`?status=&page=`            |
| `PATCH`  | `/admin/listings/:id`       | JWT (`admin`) | Modifier statut d'un bien                                  |
| `DELETE` | `/admin/listings/:id`       | JWT (`admin`) | Supprimer un bien                                          |
| `GET`    | `/admin/users`              | JWT (`admin`) | Liste tous les utilisateurs. Query :`?role=&page=`       |
| `PATCH`  | `/admin/users/:id`          | JWT (`admin`) | Suspendre / réactiver `{ is_active: bool }`             |
| `DELETE` | `/admin/users/:id`          | JWT (`admin`) | Supprimer un compte                                        |
| `GET`    | `/admin/owner-requests`     | JWT (`admin`) | Liste les demandes. Query :`?status=pending&page=`       |
| `GET`    | `/admin/owner-requests/:id/documents/:doc_id` | JWT (`admin`) | Lecture contrôlée d'un document d'identité — proxie le storage (exception au principe no-proxy, cf. ARCHITECTURE.md §7.3) |
| `PATCH`  | `/admin/owner-requests/:id` | JWT (`admin`) | Statuer `{ status: "approved"\|"rejected", admin_note? }` |

---

## 5. Génération des Types TypeScript

### Pipeline

```
1. Backend démarre → utoipa monte /api/docs/openapi.json
2. npm run generate:types → openapi-typescript consomme le JSON
3. src/shared/api/types.ts est régénéré
4. Les features importent les types depuis types.ts
```

### Configuration `package.json`

```json
{
  "scripts": {
    "generate:types": "openapi-typescript http://localhost:3000/api/docs/openapi.json -o src/shared/api/types.ts",
    "dev": "npm run generate:types && vite"
  }
}
```

**Règle :** `src/shared/api/types.ts` est ajouté au `.gitignore`. Il est régénéré à chaque démarrage en développement et dans la pipeline CI avant le build frontend.

---

## 6. Stratégie de Tests

### 6.1 Règle Générale

Les tests unitaires de chaque fichier backend sont dans un bloc `#[cfg(test)] mod tests` en fin de fichier, sans fichier `tests.rs` séparé. Les tests d'intégration (accès DB) sont isolés via feature flag `integration` et opèrent dans une transaction rollbackée.

### 6.2 Ce qu'on Teste par Couche

| Couche            | Périmètre                                         | Approche                          |
| ----------------- | --------------------------------------------------- | --------------------------------- |
| `service.rs`    | Logique métier, règles de validation, cas limites | Repository mocké via trait       |
| `repository.rs` | Requêtes SQL, mapping des résultats               | DB de test (transaction rollback) |
| `handler.rs`    | Codes HTTP retournés, format JSON                  | `axum::test` helpers            |
| `dto.rs`        | Sérialisation / désérialisation                  | Tests unitaires directs           |

### 6.3 Cas à Couvrir par Module

| Module       | Cas critiques à tester                                                                          |
| ------------ | ------------------------------------------------------------------------------------------------ |
| `auth`     | OTP expiré, OTP invalide, dépassement tentatives, is_new_user correct, rotation refresh token, réutilisation d'un token révoqué → révocation de la famille, compte `is_active=false` rejeté sur route protégée, `/auth/otp/verify` sur email inconnu → aucune ligne `users`, ticket d'inscription à usage unique, transaction `create_account` (rollback si l'insert refresh_tokens échoue), 409 `EMAIL_ALREADY_EXISTS` |
| `users`    | Cascade search_vector sur changement de nom, suppression de compte → cleanup storage (listings, avatar, pièces d'identité) effectué avant le DELETE SQL, remplacement d'avatar → ancien fichier supprimé |
| `owner_requests` | Double owner-request pending rejeté, échec upload document → aucun état persisté (atomicité), forme des pièces (1 PDF ou 2 images) |
| `listings` | Owner ne peut modifier que ses propres biens, publication refusée sans photo. Recherche (`GET /search`) : `q` vide retourne le feed complet, filtres combinés cohérents, pagination correcte, bornes de prix rejetées si négatives ou inversées |
| `media`    | Quota 5 photos respecté, formats invalides rejetés par magic bytes (extension trompeuse), suppression cover bloquée, 1ère photo d'un listing marquée `is_cover=true` automatiquement, uploads suivants `is_cover=false` |
| `contact`  | Téléphone non exposé sans JWT, listing inactif retourne 404                                   |
| `admin`    | Routes inaccessibles sans rôle admin (403), lecture document identité refusée hors rôle admin |

---

## 7. Infrastructure Docker

### 7.1 `backend/Dockerfile`

```dockerfile
# ── Stage 1 : Build ──────────────────────────────────────────
FROM rust:1.78-slim AS builder
WORKDIR /app

# Cache des dépendances Cargo
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Build final
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ── Stage 2 : Runtime ────────────────────────────────────────
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl3 ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/myhouse-backend /usr/local/bin/myhouse-backend
EXPOSE 3000
CMD ["myhouse-backend"]
```

### 7.2 `frontend/Dockerfile`

```dockerfile
# ── Stage 1 : Build ──────────────────────────────────────────
FROM node:20-alpine AS builder
WORKDIR /app

COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

COPY . .
RUN yarn build

# ── Stage 2 : Serve ──────────────────────────────────────────
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
```

### 7.3 `frontend/nginx.conf`

```nginx
server {
    listen 80;

    # SPA fallback
    location / {
        root  /usr/share/nginx/html;
        index index.html;
        try_files $uri $uri/ /index.html;
    }

    # Fichiers publics (listings, avatars) — lecture directe, jamais via le backend
    # owner-requests/* n'est jamais monté ici : pas de location = pas d'exposition
    location /media/ {
        alias /usr/share/nginx/storage/;
        add_header Cache-Control "public, max-age=86400";
    }

    # Proxy vers le backend
    location /api {
        proxy_pass         http://backend:3000;
        proxy_set_header   Host             $host;
        proxy_set_header   X-Real-IP        $remote_addr;
        proxy_set_header   X-Forwarded-For  $proxy_add_x_forwarded_for;
        client_max_body_size 10M;           # Upload photos
    }
}
```

### 7.4 `docker-compose.yml`

```yaml
version: '3.9'

services:

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB:       myhouse
      POSTGRES_USER:     myhouse
      POSTGRES_PASSWORD: myhouse
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U myhouse"]
      interval: 5s
      timeout: 5s
      retries: 5

  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    ports:
      - "3000:3000"
    env_file: ./backend/.env
    volumes:
      - storage_data:/app/storage          # RW — LocalFsStorage écrit ici
    depends_on:
      postgres:
        condition: service_healthy
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 10s
      timeout: 5s
      retries: 5
      start_period: 10s

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
    ports:
      - "80:80"
    volumes:
      - storage_data:/usr/share/nginx/storage:ro  # RO — nginx sert /media/* directement
    depends_on:
      backend:
        condition: service_healthy

volumes:
  postgres_data:
  storage_data:
```

---

## 8. Variables d'Environnement

Fichier de référence : `backend/.env.example`

```bash
# ── Application ──────────────────────────────────────────────
APP_PORT=3000
APP_ENV=development           # development | staging | production
RUST_LOG=info

# ── Base de données ──────────────────────────────────────────
DATABASE_URL=postgresql://myhouse:myhouse@postgres:5432/myhouse

# ── JWT ──────────────────────────────────────────────────────
JWT_SECRET=<minimum_256_bits_random_string>
JWT_ACCESS_TTL_SECONDS=900    # 15 minutes
JWT_REFRESH_TTL_DAYS=30

# ── OTP ──────────────────────────────────────────────────────
OTP_TTL_SECONDS=600           # 10 minutes
OTP_MAX_ATTEMPTS=3
OTP_RATE_LIMIT_SECONDS=60     # 1 demande OTP / 60s par email

# ── Storage ──────────────────────────────────────────────────
STORAGE_PROVIDER=local         # LocalFsStorage au MVP ; trait StorageProvider permet de brancher
                                # un autre provider object storage (ex. MinIO, AWS S3) en V2 sans
                                # changer le code métier — s3 (V2, non implémenté au MVP)
LOCAL_STORAGE_PATH=/app/storage
PUBLIC_MEDIA_BASE_URL=/media   # chemin relatif à l'origine — préfixe des URLs publiques générées (listings, avatars)

# Object Storage V2 — réservé, non utilisé tant que STORAGE_PROVIDER=local
AWS_REGION=eu-west-1
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=
STORAGE_BUCKET=myhouse

# ── Cookies ──────────────────────────────────────────────────
COOKIE_DOMAIN=localhost        # domaine du cookie refresh_token (httpOnly, Secure, SameSite=Strict)

# ── CORS ─────────────────────────────────────────────────────
ALLOWED_ORIGINS=http://localhost,http://localhost:5173   # obligatoire ; liste séparée par des virgules
                                # chaque origine commence par http:// ou https://, sans slash final ;
                                # une entrée vide (virgule en trop) bloque le démarrage

# ── Email ────────────────────────────────────────────────────
SMTP_HOST=localhost
SMTP_PORT=1025                 # toujours utilisé tel quel, quel que soit SMTP_SECURITY
SMTP_SECURITY=none             # none | starttls | tls (TLS implicite) — défaut none
SMTP_USERNAME=                 # les deux ou aucun — obligatoires en staging/production
SMTP_PASSWORD=
SMTP_FROM=noreply@myhouse.app
ADMIN_NOTIFICATION_EMAIL=admin@myhouse.app     # destinataire fixe — admin unique au MVP (cf. §3bis.2)

# ── Admin Bootstrap ──────────────────────────────────────────
ADMIN_BOOTSTRAP_EMAIL=admin@myhouse.app # email du compte admin unique, créé au démarrage si absent en DB ; requis si ON_STARTUP=true
ADMIN_BOOTSTRAP_ON_STARTUP=true # si true, vérifie/crée le compte admin au boot (idempotent)

# ── Rate limiting générique ──────────────────────────────────
RATE_LIMIT_MAX_REQUESTS=100     # obligatoire, > 0 ; requêtes par IP et par fenêtre (distinct de OTP_RATE_LIMIT_SECONDS)
RATE_LIMIT_WINDOW_SECONDS=60    # obligatoire, > 0
TRUSTED_PROXIES=                # obligatoire, même vide ; IP/CIDR dont X-Forwarded-For est cru ; vide = IP du pair (cargo run en local)
                                # en staging/prod : la plage la plus étroite couvrant nginx seul
```

La CORS autorise les méthodes `GET`, `POST`, `PUT`, `PATCH`, `DELETE`, les en-têtes `Authorization` et `Content-Type`, et les credentials (cookie `refresh_token`). Seules les origines d'`ALLOWED_ORIGINS` sont acceptées.

---

## 9. Exclusions MVP

| Feature                       | Module concerné         | Cible                  |
| ----------------------------- | ------------------------ | ---------------------- |
| Messagerie in-app             | `contact`              | V2                     |
| Notifications push (FCM/APNs) | `notifications`        | V2                     |
| Carte et géolocalisation     | `listings`             | V2                     |
| Favoris                       | Nouveau module           | V2                     |
| OAuth (Google, Facebook)      | `auth`                 | V2                     |
| Application mobile native     | —                       | V2                     |
| Tableau de bord admin complet | `admin`                | V2                     |
| Redis                         | `infra`                | V2 si moka insuffisant |
| Stockage S3-compatible (MinIO/AWS S3) | `infra/storage` | V2 — `LocalFsStorage` au MVP |
| Conformité RGPD complète    | Transverse               | Avant go-live EU       |
| Analytics et reporting        | Nouveau module           | V2                     |
| Rappel mensuel disponibilité (LIST-03) | `listings`, `notifications` | V2 — nécessite tâche planifiée, hors scope MVP (cf. ARCHITECTURE.md R-10) |
| Notifications in-app           | `notifications`         | V2 — email uniquement au MVP (cf. §3bis) |

---

## Historique des Révisions

| Version | Date      | Description                                                             |
| ------- | --------- | ----------------------------------------------------------------------- |
| 1.0     | Juin 2025 | Document initial — issu de la refactorisation ARCHITECTURE_MVP.md v2.0 |
| 1.1     | Juin 2026 | Renommage `seeker`, `LocalFsStorage` MVP, owner-request enrichi (identité + documents), rotation refresh token + cookie httpOnly, `is_active` à chaque requête, avatar en scope, `/health`, graceful shutdown |
| 1.2     | Août 2026 | Correction pagination feed (20/page uniforme), sélection automatique de la cover photo au premier upload, nouvelle section Stack de Notification (§3bis) bout-en-bout, admin unique + bootstrap documentés, rappel mensuel disponibilité (LIST-03) formellement différé en V2 |
| 1.3     | Novembre 2026 | MH-68, alignement sur le code après EP-09 et EP-10. §1.1/§1.2 : arbres tels que construits (pas de module `search`, `GET /search` servi par `listings` ; module `owner_requests` ; `features/search/` et `Checkbox` partagée). §2.2 : `published_at`, `currency`, `immutable_unaccent` et `idx_listings_city_normalized`, libellés de type français et anglais dans `search_vector` (MH-66). §3.1 : appelants du `StorageProvider` (avatar et pièces d'identité hors `media`). §4.3 : filtres `neighborhood` et `status`, `type` répétable, visibilité publique, prix en XAF, endpoints `POST`, `PUT`, `PATCH status`, `DELETE`, `publish`, `cover`, `media/order` et `/users/me/listings`, règles de validation. §4.4 : contrat de MH-64-BE. §8 : `ALLOWED_ORIGINS`, rate-limit générique, `TRUSTED_PROXIES`. Fichier renommé sans numéro de version |
