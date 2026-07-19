# My House — Technical Specification MVP

> **Version :** 1.1
> **Statut :** En validation
> **Prérequis :** [`ARCHITECTURE.md`](./ARCHITECTURE.md) v2.1 — validé
> **Scope :** Spécifications d'implémentation complètes pour le périmètre MVP

---

## Table des Matières

1. [Structure du Projet](#1-structure-du-projet)
2. [Modèle de Données](#2-modèle-de-données)
3. [Abstraction Storage](#3-abstraction-storage)
4. [Contrat API](#4-contrat-api)
5. [Génération des Types TypeScript](#5-génération-des-types-typescript)
6. [Stratégie de Tests](#6-stratégie-de-tests)
7. [Infrastructure Docker](#7-infrastructure-docker)
8. [Variables d&#39;Environnement](#8-variables-denvironnement)
9. [Exclusions MVP](#9-exclusions-mvp)

---

## 1. Structure du Projet

### 1.1 Backend

```
backend/
├── Cargo.toml
├── Dockerfile
├── .env.example
├── migrations/                        # sqlx migrate — fichiers horodatés .sql
│   └── 0001_init.sql
├── docs-frontend/
└── src/
    ├── main.rs                        # Bootstrap, AppState, , DI
    │
    ├── app_server.rs                  # router global, /health, graceful shutdown
    │
    ├── config/
    │   └── mod.rs                     # AppConfig — chargement et validation des env vars
    │
    ├── middleware/
    │   ├── mod.rs
    │   ├── cors.rs          	       # CorsLayer (tower-http)
    │   ├── logging.rs		       # tracing + request_id
    │   └── rate_limit.rs    	       # Limite générique (complète le rate-limit OTP spécifique)
    │
    ├── infra/
    │   ├── mod.rs
    │   ├── db.rs                      # Pool sqlx::PgPool
    │   ├── cache.rs                   # moka::Cache — OTP + pending registrations
    │   ├── mailer.rs                  # Client SMTP (lettre)
    │   └── storage/
    │       ├── mod.rs                 # Factory : sélection impl via STORAGE_PROVIDER (local au MVP)
    │       ├── provider.rs            # trait StorageProvider (incl. presigned_url, read — non utilisés au MVP)
    │       └── local_fs.rs            # impl LocalFsStorage (MVP) — AwsS3Storage ajouté en V2
    │
    ├── modules/
    │   ├── auth/
    │   │   ├── mod.rs
    │   │   ├── router.rs
    │   │   ├── handler.rs
    │   │   ├── service.rs
    │   │   ├── repository.rs          # refresh_tokens CRUD
    │   │   ├── model.rs
    │   │   ├── dto.rs                 # ToSchema derives (utoipa)
    │   │   └── tests.rs
    │   │
    │   ├── users/
    │   │   ├── mod.rs
    │   │   ├── router.rs
    │   │   ├── handler.rs
    │   │   ├── service.rs              # Délègue upload avatar + documents identité à media::service
    │   │   ├── repository.rs          # users + owner_requests CRUD (incl. identity_data, identity_documents)
    │   │   ├── model.rs
    │   │   ├── dto.rs
    │   │   └── tests.rs
    │   │
    │   ├── listings/
    │   │   ├── mod.rs
    │   │   ├── router.rs
    │   │   ├── handler.rs
    │   │   ├── service.rs
    │   │   ├── repository.rs
    │   │   ├── model.rs
    │   │   ├── dto.rs
    │   │   └── tests.rs
    │   │
    │   ├── search/
    │   │   ├── mod.rs
    │   │   ├── router.rs
    │   │   ├── handler.rs
    │   │   ├── service.rs
    │   │   ├── repository.rs          # Requêtes full-text PostgreSQL
    │   │   ├── dto.rs
    │   │   └── tests.rs
    │   │
    │   ├── media/
    │   │   ├── mod.rs
    │   │   ├── router.rs
    │   │   ├── handler.rs
    │   │   ├── service.rs             # Validation (magic bytes) + délégation à StorageProvider
    │   │   │                         # upload_listing_photo · upload_avatar · upload_identity_documents
    │   │   ├── repository.rs          # listing_media CRUD
    │   │   ├── model.rs
    │   │   ├── dto.rs
    │   │   └── tests.rs
    │   │
    │   ├── contact/
    │   │   ├── mod.rs
    │   │   ├── router.rs
    │   │   ├── handler.rs
    │   │   ├── service.rs
    │   │   ├── dto.rs
    │   │   └── tests.rs
    │   │
    │   ├── notifications/
    │   │   ├── mod.rs
    │   │   ├── service.rs
    │   │   ├── templates/
    │   │   │   ├── otp.html
    │   │   │   ├── welcome.html
    │   │   │   ├── owner_request_received.html
    │   │   │   ├── owner_request_approved.html
    │   │   │   └── owner_request_rejected.html
    │   │   └── tests.rs
    │   │
    │   └── admin/
    │       ├── mod.rs
    │       ├── router.rs
    │       ├── handler.rs
    │       ├── service.rs
    │       ├── dto.rs
    │       └── tests.rs
    │
    └── shared/
        ├── mod.rs
        ├── errors.rs                  # AppError → réponse HTTP structurée
        ├── pagination.rs              # PaginatedResponse<T>
        ├── extractors.rs              # AuthUser — JWT claims + revérifie is_active à chaque requête
        ├── rbac.rs                    # Guards de rôle (seeker/owner/admin) utilisés par les extracteurs
        └── types.rs                   # NewType wrappers : UserId, ListingId...
```

### 1.2 Frontend

```
frontend/
├── package.json
├── tsconfig.json
├── Dockerfile
├── nginx.conf
├── docs-frontend/
├	└── openapi.json                   # Généré depuis utoipa — ne pas committer (gitignore)
└── src/
    ├── main.tsx
    ├── App.tsx
    │
    ├── app/
    │   ├── router.tsx                 # Routes globales + guards rôle
    │   ├── providers.tsx              # QueryClientProvider, AuthProvider
    │   └── layout/
    │       ├── RootLayout.tsx
    │       ├── AuthLayout.tsx
    │       └── AdminLayout.tsx
    │
    ├── features/
    │   ├── auth/
    │   │   ├── components/
    │   │   │   ├── OtpRequestForm.tsx
    │   │   │   ├── OtpVerifyForm.tsx
    │   │   │   └── ProfileSetupForm.tsx  # Affiché si is_new_user = true
    │   │   ├── hooks/useAuth.ts
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   ├── listings/
    │   │   ├── components/
    │   │   │   ├── ListingFeed.tsx       # Grille de cover photos scrollable
    │   │   │   ├── ListingCard.tsx       # Cover + résumé (titre, ville, prix)
    │   │   │   ├── ListingDetail.tsx     # Page complète + owner info
    │   │   │   └── ListingForm.tsx       # Création / édition (Owner)
    │   │   ├── hooks/useListings.ts
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   ├── search/
    │   │   ├── components/
    │   │   │   ├── SearchBar.tsx
    │   │   │   └── FilterPanel.tsx
    │   │   ├── hooks/useSearch.ts
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   ├── profile/
    │   │   ├── components/
    │   │   │   ├── ProfileForm.tsx
    │   │   │   ├── OwnerRequestForm.tsx  # Submit unique : phone + identité + documents
    │   │   │   └── AvatarUpload.tsx      # Upload / remplacement de l'avatar
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   ├── contact/
    │   │   ├── components/
    │   │   │   └── ContactReveal.tsx
    │   │   ├── api.ts
    │   │   └── index.ts
    │   │
    │   └── admin/
    │       ├── components/
    │       │   ├── UserTable.tsx
    │       │   ├── ListingTable.tsx
    │       │   └── OwnerRequestTable.tsx
    │       ├── api.ts
    │       └── index.ts
    │
    └── shared/
        ├── components/
        │   ├── Button.tsx
        │   ├── Card.tsx
        │   ├── Modal.tsx
        │   ├── Pagination.tsx
        │   └── Spinner.tsx
        ├── hooks/
        │   └── usePagination.ts
        ├── api/
        │   ├── client.ts              # Instance fetch (base URL, headers, interceptors)
        │   └── types.ts               # Auto-généré par openapi-typescript — ne pas éditer
        └── utils/
            └── format.ts
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

-- ============================================================
-- Triggers
-- ============================================================

-- search_vector : mis à jour à chaque INSERT/UPDATE sur listings
-- Inclut le nom de l'owner (pondéré B) pour permettre la recherche par propriétaire
CREATE OR REPLACE FUNCTION fn_update_listing_search_vector()
RETURNS TRIGGER AS $$
DECLARE
    v_owner_name TEXT;
BEGIN
    SELECT COALESCE(first_name, '') || ' ' || COALESCE(last_name, '')
        INTO v_owner_name
        FROM users WHERE id = NEW.owner_id;

    NEW.search_vector :=
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.title, ''))),        'A') ||
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.city, ''))),          'B') ||
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.neighborhood, ''))),  'B') ||
        setweight(to_tsvector('french', unaccent(COALESCE(v_owner_name, ''))),      'B') ||
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

| Décision                                          | Détail                                                                                                                                                                                        |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `users.first_name / last_name` nullable          | Compte créé à la vérification OTP avant complétion du profil                                                                                                                              |
| `users.phone` nullable                           | Renseigné lors de la demande owner — obligatoire pour le rôle owner                                                                                                                         |
| OTP non stocké en DB                              | Géré exclusivement dans moka (in-memory, TTL 10 min)                                                                                                                                         |
| `storage_key` vs `url`                         | `storage_key` = chemin interne pérenne. `url` = URL recalculable depuis la clé                                                                                                           |
| `is_cover` index partiel unique                  | PostgreSQL garantit l'unicité de la cover sans contrainte applicative                                                                                                                         |
| `owner_requests` index partiel                   | Bloque un second`pending` sans empêcher plusieurs demandes historiques                                                                                                                      |
| `identity_documents` JSONB                       | Tableau de références storage — accès restreint admin, jamais exposé en statique (cf. ARCHITECTURE.md §7.3)                                                                              |
| `refresh_tokens.revoked_at` / `replaced_by_id` | Supportent la rotation : un refresh consomme le token courant et chaîne vers le nouveau. Réutilisation d'un token`revoked_at IS NOT NULL` ⇒ révocation de tous les tokens du `user_id` |

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

| Méthode | Endpoint    | Auth | Description                                            |
| -------- | ----------- | ---- | ------------------------------------------------------ |
| `GET`  | `/health` | Non  | Hors`/api/v1`. Vérifie la connectivité PostgreSQL. |

```json
// Response 200
{ "status": "ok" }

// Response 503 — DB inaccessible
{ "status": "unavailable" }
```

---

### 4.1 Auth

| Méthode | Endpoint              | Auth   | Description                                                                                                                                                                                                                            |
| -------- | --------------------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `POST` | `/auth/otp/request` | Non    | Envoie un OTP. Gère login et inscription selon si l'email est connu.                                                                                                                                                                  |
| `POST` | `/auth/otp/verify`  | Non    | Vérifie le code. Crée le compte si nouvel email. Retourne`{ access_token, is_new_user }` + cookie `refresh_token`.                                                                                                               |
| `POST` | `/auth/refresh`     | Cookie | Lit le refresh token du cookie,**révoque l'ancien et en émet un nouveau** (rotation). Renvoie `{ access_token }` + cookie mis à jour. Réutilisation d'un token déjà révoqué → 401 et révocation de toute la famille. |
| `POST` | `/auth/logout`      | JWT    | Révoque le refresh token courant et efface le cookie.                                                                                                                                                                                 |

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

// Response 200
// Set-Cookie: refresh_token=...; HttpOnly; Secure; SameSite=Strict; Max-Age=2592000
{
  "data": {
    "access_token": "eyJ...",
    "is_new_user": true
  }
}

// Response 401 — code invalide ou expiré
{ "error": { "code": "OTP_INVALID", "message": "Code invalide ou expiré.", "status": 401 } }
```

---

### 4.2 Users

| Méthode   | Endpoint                    | Auth | Rôle      | Description                                                                         |
| ---------- | --------------------------- | ---- | ---------- | ----------------------------------------------------------------------------------- |
| `GET`    | `/users/me`               | JWT  | tous       | Profil de l'utilisateur connecté                                                   |
| `PUT`    | `/users/me`               | JWT  | tous       | Mise à jour du profil                                                              |
| `DELETE` | `/users/me`               | JWT  | tous       | Suppression du compte — cascade DB + nettoyage storage (listings, médias, avatar) |
| `POST`   | `/users/me/avatar`        | JWT  | tous       | Upload / remplacement de l'avatar — l'ancien fichier est supprimé du storage      |
| `GET`    | `/users/me/owner-request` | JWT  | `seeker` | Statut de la demande en cours                                                       |

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

---

### 4.2bis Owner Requests

| Méthode | Endpoint            | Auth | Rôle      | Description                                             |
| -------- | ------------------- | ---- | ---------- | ------------------------------------------------------- |
| `POST` | `/owner-requests` | JWT  | `seeker` | Soumet la demande complète en un seul appel (atomique) |

**`POST /owner-requests`**

```
Content-Type: multipart/form-data
Body: {
  phone: string,
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
| `GET`    | `/listings/:id`        | Non  | —        | Détail complet + owner info (sans phone)    |
| `POST`   | `/listings`            | JWT  | `owner` | Créer un bien                               |
| `PUT`    | `/listings/:id`        | JWT  | `owner` | Modifier son bien                            |
| `DELETE` | `/listings/:id`        | JWT  | `owner` | Supprimer son bien                           |
| `GET`    | `/users/me/listings`   | JWT  | `owner` | Ses propres biens                            |
| `PATCH`  | `/listings/:id/status` | JWT  | `owner` | Changer le statut                            |
| `PATCH`  | `/listings/:id/cover`  | JWT  | `owner` | Définir la photo de couverture              |

**`GET /listings` — Query params**

| Paramètre   | Type        | Description                                 |
| ------------ | ----------- | ------------------------------------------- |
| `owner_id` | `uuid`    | Filtrer par propriétaire                   |
| `city`     | `string`  | Filtre par ville                            |
| `type`     | `enum`    | `apartment\|studio\|house\|room\|villa\|other` |
| `page`     | `integer` | Défaut : 1                                 |
| `per_page` | `integer` | Défaut : 20, max : 50                      |

**`GET /listings` — Response 200**

```json
{
  "data": [
    {
      "id": "uuid",
      "title": "Studio meublé Plateau",
      "type": "studio",
      "status": "available",
      "city": "Dakar",
      "neighborhood": "Plateau",
      "price": 150000,
      "cover_photo_url": "https://...",
      "owner": { "id": "uuid", "first_name": "Moussa", "last_name": "Diallo" }
    }
  ],
  "pagination": { "page": 1, "per_page": 30, "total": 47, "total_pages": 3 }
}
```

**`GET /listings/:id` — Response 200**

```json
{
  "data": {
    "id": "uuid",
    "title": "Studio meublé Plateau",
    "description": "...",
    "type": "studio",
    "status": "available",
    "city": "Dakar",
    "neighborhood": "Plateau",
    "price": 150000,
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
    "created_at": "2025-06-01T10:00:00Z"
  }
}
```

**`PATCH /listings/:id/cover`**

```json
// Request
{ "media_id": "uuid" }

// Response 200
{ "data": { "message": "Cover mise à jour" } }
```

---

### 4.4 Search

| Méthode | Endpoint    | Auth | Description                   |
| -------- | ----------- | ---- | ----------------------------- |
| `GET`  | `/search` | Non  | Full-text + filtres combinés |

**Query params**

| Paramètre    | Type        | Description                                       |
| ------------- | ----------- | ------------------------------------------------- |
| `q`         | `string`  | Terme libre (titre, ville, nom owner, label bien) |
| `owner_id`  | `uuid`    | Filtrer par propriétaire                         |
| `city`      | `string`  | Filtre par ville                                  |
| `type`      | `enum`    | Type de bien                                      |
| `price_min` | `number`  | Prix minimum                                      |
| `price_max` | `number`  | Prix maximum                                      |
| `page`      | `integer` | Défaut : 1                                       |
| `per_page`  | `integer` | Défaut : 20, max : 50                            |

Retourne le même format que `GET /listings`.

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
La cover ne peut pas être supprimée sans en avoir désigné une autre au préalable.

```json
// Response 201
{ "data": { "id": "uuid", "url": "https://...", "is_cover": false, "position": 2 } }
```

---

### 4.6 Contact

| Méthode | Endpoint                  | Auth | Rôle                   | Description                        |
| -------- | ------------------------- | ---- | ----------------------- | ---------------------------------- |
| `GET`  | `/listings/:id/contact` | JWT  | `seeker` ou `owner` | Révèle le téléphone de l'owner |

```json
// Response 200
{ "data": { "phone": "+221 77 123 45 67" } }

// Response 404 — listing inexistant ou inactif
{ "error": { "code": "LISTING_NOT_FOUND", "status": 404 } }
```

---

### 4.7 Admin

| Méthode   | Endpoint                                        | Auth            | Description                                                                                                                    |
| ---------- | ----------------------------------------------- | --------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `GET`    | `/admin/listings`                             | JWT (`admin`) | Liste tous les biens. Query :`?status=&page=`                                                                                |
| `PATCH`  | `/admin/listings/:id`                         | JWT (`admin`) | Modifier statut d'un bien                                                                                                      |
| `DELETE` | `/admin/listings/:id`                         | JWT (`admin`) | Supprimer un bien                                                                                                              |
| `GET`    | `/admin/users`                                | JWT (`admin`) | Liste tous les utilisateurs. Query :`?role=&page=`                                                                           |
| `PATCH`  | `/admin/users/:id`                            | JWT (`admin`) | Suspendre / réactiver`{ is_active: bool }`                                                                                  |
| `DELETE` | `/admin/users/:id`                            | JWT (`admin`) | Supprimer un compte                                                                                                            |
| `GET`    | `/admin/owner-requests`                       | JWT (`admin`) | Liste les demandes. Query :`?status=pending&page=`                                                                           |
| `GET`    | `/admin/owner-requests/:id/documents/:doc_id` | JWT (`admin`) | Lecture contrôlée d'un document d'identité — proxie le storage (exception au principe no-proxy, cf. ARCHITECTURE.md §7.3) |
| `PATCH`  | `/admin/owner-requests/:id`                   | JWT (`admin`) | Statuer`{ status: "approved"\|"rejected", admin_note? }`                                                                      |

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

Chaque module backend contient un fichier `tests.rs` déclaré dans `mod.rs` sous `#[cfg(test)]`. Les tests d'intégration (accès DB) sont isolés via feature flag `integration` et opèrent dans une transaction rollbackée.

### 6.2 Ce qu'on Teste par Couche

| Couche            | Périmètre                                         | Approche                          |
| ----------------- | --------------------------------------------------- | --------------------------------- |
| `service.rs`    | Logique métier, règles de validation, cas limites | Repository mocké via trait       |
| `repository.rs` | Requêtes SQL, mapping des résultats               | DB de test (transaction rollback) |
| `handler.rs`    | Codes HTTP retournés, format JSON                  | `axum::test` helpers            |
| `dto.rs`        | Sérialisation / désérialisation                  | Tests unitaires directs           |

### 6.3 Cas à Couvrir par Module

| Module       | Cas critiques à tester                                                                                                                                                                                                                                                                       |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `auth`     | OTP expiré, OTP invalide, dépassement tentatives, is_new_user correct, rotation refresh token, réutilisation d'un token révoqué → révocation de la famille, compte`is_active=false` rejeté sur route protégée                                                                     |
| `users`    | Double owner-request pending rejeté, échec upload document → aucun état persisté (atomicité), cascade search_vector sur changement de nom, suppression de compte → cleanup storage (listings, avatar) effectué avant le DELETE SQL, remplacement d'avatar → ancien fichier supprimé |
| `listings` | Owner ne peut modifier que ses propres biens, cover obligatoire avant suppression                                                                                                                                                                                                             |
| `media`    | Quota 5 photos respecté, formats invalides rejetés par magic bytes (extension trompeuse), suppression cover bloquée                                                                                                                                                                        |
| `search`   | Recherche vide retourne feed complet, filtres combinés cohérents, pagination correcte                                                                                                                                                                                                       |
| `contact`  | Téléphone non exposé sans JWT, listing inactif retourne 404                                                                                                                                                                                                                                |
| `admin`    | Routes inaccessibles sans rôle admin (403), lecture document identité refusée hors rôle admin                                                                                                                                                                                             |

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
APP_ENV=development           # development | production
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
STORAGE_PROVIDER=local         # local (MVP) | s3 (V2 — non implémenté)
LOCAL_STORAGE_PATH=/app/storage
PUBLIC_MEDIA_BASE_URL=http://localhost/media   # base des URLs publiques générées (listings, avatars)

# AWS S3 — réservé V2, non utilisé tant que STORAGE_PROVIDER=local
AWS_REGION=eu-west-1
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=
STORAGE_BUCKET=myhouse

# ── Cookies ──────────────────────────────────────────────────
COOKIE_DOMAIN=localhost        # domaine du cookie refresh_token (httpOnly, Secure, SameSite=Strict)

# ── Email ────────────────────────────────────────────────────
SMTP_HOST=localhost
SMTP_PORT=1025
SMTP_FROM=noreply@myhouse.app
```

---

## 9. Exclusions MVP

| Feature                               | Module concerné         | Cible                          |
| ------------------------------------- | ------------------------ | ------------------------------ |
| Messagerie in-app                     | `contact`              | V2                             |
| Notifications push (FCM/APNs)         | `notifications`        | V2                             |
| Carte et géolocalisation             | `listings`, `search` | V2                             |
| Favoris                               | Nouveau module           | V2                             |
| OAuth (Google, Facebook)              | `auth`                 | V2                             |
| Application mobile native             | —                       | V2                             |
| Tableau de bord admin complet         | `admin`                | V2                             |
| Redis                                 | `infra`                | V2 si moka insuffisant         |
| Stockage S3-compatible (MinIO/AWS S3) | `infra/storage`        | V2 —`LocalFsStorage` au MVP |
| Conformité RGPD complète            | Transverse               | Avant go-live EU               |
| Analytics et reporting                | Nouveau module           | V2                             |

---

## Historique des Révisions

| Version | Date      | Description                                                                                                                                                                                                             |
| ------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1.0     | Juin 2025 | Document initial — issu de la refactorisation ARCHITECTURE_MVP.md v2.0                                                                                                                                                 |
| 1.1     | Juin 2026 | Renommage`seeker`, `LocalFsStorage` MVP, owner-request enrichi (identité + documents), rotation refresh token + cookie httpOnly, `is_active` à chaque requête, avatar en scope, `/health`, graceful shutdown |
