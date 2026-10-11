# MH-61 — Publier un bien : plan d'implémentation (BE + FE)

- **Epic** : EP-09 — Listings (#129)
- **Ticket parent** : MH-61 — Publish a listing (#170)
- **Sous-tickets** : MH-61-BE (#171) → MH-61-FE (#172)
- **Branches** : MH-61-BE sur `mh-61-be-implement-post-listingsidpublish` (existe en local et sur origin). MH-61-FE sur une nouvelle branche créée depuis `develop` après le merge du BE. Aucun commit ni push automatique.
- **Déjà livré, et utilisé ici** : MH-54 (`published_at`, règle de visibilité, badge et message de l'owner bar), MH-56/57/58 (motifs handler owner, `AppJson`, mutations, modale de suppression), MH-59/60 (écran photos, prédicats d'erreur par code).
- **CDC** (§4.2.1) : « publication et gestion de biens avec photos ». Le ticket précise la règle : au moins une photo, publication irréversible au MVP.

---

## 1. Contexte et objectif

Un bien créé est un brouillon (`published_at IS NULL`), visible seulement par son owner. La règle de visibilité publique existe déjà (`published_at IS NOT NULL` et une cover) dans `listings/repository.rs::push_public_visibility` et `listings/service.rs::is_visible_to`, mais **aucun endpoint ne renseigne `published_at`** : un bien ne peut donc jamais devenir public aujourd'hui.

**Objectif** : `POST /listings/:id/publish` (owner) qui pose `published_at = now()` sur un brouillon ayant au moins une photo, et un bouton « Publish » avec confirmation dans l'owner bar de la page détail.

### Décisions prises

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Branches | BE sur la branche actuelle. FE sur une nouvelle branche depuis `develop` après merge du BE (types régénérés depuis le vrai `openapi.json`). |
| D2 | Logique pure | `publish_decision(is_published, has_photo) -> Result<PublishDecision, AppError>` dans `listings/service.rs`. Publié (avec ou sans photo) → `AlreadyPublished` ; brouillon avec photo → `Publish` ; brouillon sans photo → `PhotoRequired`. |
| D3 | Pas de transaction ni de verrou | Le ticket l'autorise : le pire cas (course avec la suppression de la dernière photo) donne un bien publié sans photo, état permis et masqué au public. Lecture via `find_listing_by_id` (déjà : `owner_id`, `published_at`, `has_photo`), puis `UPDATE … WHERE id = $1 AND owner_id = $2 AND published_at IS NULL`. La garde `IS NULL` garde le `published_at` d'origine si deux publications se croisent. |
| D4 | « Photo » | Même définition que la visibilité : une ligne cover existe (`has_photo`). Équivalent à « au moins une photo » grâce à la cover automatique au premier upload et à `CoverPhotoRequired`. |
| D5 | Erreur | Nouvelle variante `AppError::PhotoRequired` → 409 `PHOTO_REQUIRED`. Pas de réutilisation de `COVER_PHOTO_REQUIRED` (sens différent). |
| D6 | Réponse | 200 avec `ListingDetailResponse`, via `get_listing_detail(pool, id, Some(owner_id))`, comme `update_listing`. Pas de corps de requête. |
| D7 | Ordre des contrôles | `require_role(OWNER_ROLES)` dans le handler, avant tout accès DB : seeker/admin → 403 même sur un id inconnu (comme les autres routes owner). |
| D8 | 409 côté FE | La modale se ferme, toast `info` dans le hook, refetch de `["listing", id]` : la barre repasse en « brouillon sans photo » (bouton désactivé + message). |
| D9 | Succès côté FE | `setQueryData(["listing", id], réponse)` pour un badge immédiat, puis invalidation de `["listings"]` et `["owner-listings"]`. Toast succès dans le hook : le bouton disparaît, donc le composant est démonté et les callbacks de `mutate()` ne s'exécuteraient pas. |
| D10 | TECHNICAL_SPEC | Une ligne dans la table §4.3 et un court paragraphe `POST /listings/:id/publish`. Le reste vient de l'OpenAPI généré. |

---

## 2. Découpage BE / FE et dépendances

```
MH-61-BE (#171)  ──►  merge dans develop  ──►  MH-61-FE (#172)
  AppError::PhotoRequired (409)           │     types.ts régénéré
  publish_decision (pure + tests)         │     api.publishListing
  repository::publish_listing (UPDATE)    │     isPhotoRequired (pure + tests)
  service::publish_listing                │     usePublishListing
  handler + router (POST …/publish)       │     PublishListingAction (Modal)
  spec §4.3, .sqlx, openapi.json          │     OwnerBar : bouton + messages
```

Le FE dépend du BE pour l'endpoint, le code `PHOTO_REQUIRED` et les types générés.

### 2.1 MH-61-BE : fichiers, dans l'ordre (un fichier à la fois)

1. **`backend/src/shared/errors.rs`** : variante `PhotoRequired` dans le bloc 409 (`"A listing needs at least one photo to be published."`), mapping `(StatusCode::CONFLICT, "PHOTO_REQUIRED")`, test d'enveloppe sur le motif de `MediaOrderConflict`.
2. **`backend/src/modules/listings/repository.rs`** : `publish_listing(pool, id, owner_id) -> Result<(), AppError>` :
   - `UPDATE listings SET published_at = now() WHERE id = $1 AND owner_id = $2 AND published_at IS NULL` avec `sqlx::query!` ;
   - aucune autre colonne ; `updated_at` par trigger (`fn_set_updated_at`) ;
   - le nombre de lignes est ignoré : 0 ligne = déjà publié entre-temps ou supprimé, et la relecture du service tranche.
3. **`backend/src/modules/listings/service.rs`** :
   - `enum PublishDecision { Publish, AlreadyPublished }` + `publish_decision` (D2) ;
   - `publish_listing(pool, id, owner_id)` : `find_listing_by_id` → `None` ou `row.owner_id != owner_id` → `ListingNotFound` → `publish_decision(row.published_at.is_some(), row.has_photo)?` → si `Publish`, `repository::publish_listing` → `get_listing_detail(pool, id, Some(owner_id))`.
4. **`backend/src/modules/listings/handler.rs`** : `publish(State, AuthUser, Path(id))` :
   - `user.require_role(OWNER_ROLES)?`, service, `Json(ListingDetailResponse { data })` ;
   - `#[utoipa::path(post, path = "/listings/{id}/publish", tag = "listings", params(id), responses(200 ListingDetailResponse « published, or already published », 401, 403, 404, 409 « no photo »))]`.
5. **`backend/src/modules/listings/router.rs`** : `.routes(routes!(handler::publish)) // owner`.
6. **Docs** (D10) : `docs/TECHNICAL_SPEC_MVP_v1.2.md` §4.3 :
   - ligne : `POST | /listings/:id/publish | JWT | owner | Publier un brouillon` ;
   - paragraphe : brouillon + au moins une photo → `published_at` posé, 200 forme `GET /listings/:id` ; déjà publié → 200, `published_at` inchangé ; sans photo → 409 `PHOTO_REQUIRED` ; inconnu / autre owner → 404 identique ; irréversible au MVP.
7. **Artefacts** : `cargo sqlx prepare` (`.sqlx/`, une requête nouvelle) et `cargo run --bin gen_openapi` (`frontend/docs-frontend/openapi.json`).

### 2.2 MH-61-BE : tests unitaires (logique pure, sans DB)

- `publish_decision` (critère du ticket) :
  - brouillon avec photo → `Publish` ;
  - brouillon sans photo → `Err(PhotoRequired)` ;
  - publié avec photo → `AlreadyPublished` ;
  - publié sans photo (masqué) → `AlreadyPublished`.
- `shared/errors.rs` : `PhotoRequired` → 409 `PHOTO_REQUIRED`.
- Rôles : déjà couverts par `only_owners_may_manage_their_listings` (même `OWNER_ROLES`).
- **Dette** (base réelle requise, epic tests d'intégration) : 200 et `published_at` posé, `published_at` d'origine conservé, 409 laisse un brouillon, 404 identique (inconnu / autre owner), aucun autre champ modifié, `updated_at` par trigger, apparition dans le feed. Vérifiés à la main (section 5).

### 2.3 MH-61-FE : fichiers, dans l'ordre (un fichier à la fois)

Prérequis : MH-61-BE mergé dans `develop`, branche FE créée depuis `develop`.

1. **`frontend/src/shared/api/types.ts`** : régénéré avec `npm run generate:types:ci`, jamais à la main.
2. **`frontend/src/features/listings/api.ts`** : `publishListing(id)` → `apiPost<{ data: ListingDetail }>(\`/api/v1/listings/${encodeURIComponent(id)}/publish\`)` (vérifier que `apiPost` accepte l'absence de corps).
3. **`frontend/src/features/listings/listingPublication.ts`** (+ `.test.ts`) : `isPhotoRequired(error)` (`ApiError` 409 avec le code `PHOTO_REQUIRED`), sur le motif de `listingDeletion.ts` / `isMediaOrderConflict`.
4. **`frontend/src/features/listings/hooks/usePublishListing.ts`** : `useMutation`, `retry: false`.
   - `onSuccess(response)` : `setQueryData(["listing", id], …)` (vérifier la forme stockée par `useListing`), `invalidateQueries(["listings"])` et `(["owner-listings"])`, toast `success` « Listing published. » (D9).
   - `onError` : si `isPhotoRequired` → `invalidateQueries(["listing", id])` + toast `info` « Add at least one photo before publishing. » (D8). Les autres erreurs restent dans la modale.
5. **`frontend/src/features/listings/components/PublishListingAction.tsx`** (nouveau), calqué sur `DeleteListingAction.tsx` :
   - bouton `Button` « Publish », `disabled` si `!hasPhoto`, `aria-describedby` vers le message de la barre ;
   - `Modal` « Publish this listing? » : « It will be visible to everyone. Publishing can't be undone: afterwards you can only mark it unavailable or delete it. » ;
   - pied : « Cancel » / « Publish » (`isLoading`), les deux `disabled` pendant la requête ; `handleClose` ignoré tant que `isPending` (Escape, backdrop, croix passent par `onClose`) ;
   - `mutate(undefined, { onError })` : ferme la modale si `isPhotoRequired` ; sinon `Alert` d'erreur dans la modale (« Couldn't publish the listing. Please try again. ») ;
   - `reset()` à l'ouverture.
6. **`frontend/src/features/listings/components/OwnerBar.tsx`** :
   - `PublishListingAction` rendu seulement si `visibility === "draft"` ;
   - message du brouillon selon `hasPhoto` : sans photo « Add at least one photo to publish this listing. » (le lien « Next: add photos → » existe déjà) ; avec photo « Only you can see this listing. Publish it to make it visible to everyone. » ;
   - published / hidden : inchangés, pas de bouton.
7. **`ListingDetail.tsx`** : aucun changement (passe déjà `publishedAt` et `hasPhoto`).

### 2.4 MH-61-FE : tests unitaires (vitest, logique pure)

- `isPhotoRequired` : 409 `PHOTO_REQUIRED` → vrai ; 409 `MEDIA_ORDER_CONFLICT`, 404, 500, `TypeError` → faux.
- `listingVisibility` : déjà testé, inchangé.
- **Dette** (pas de React Testing Library) : bouton présent/désactivé/absent selon l'état, modale verrouillée pendant la requête, 409 → fermeture + rechargement + toast, autre erreur → modale ouverte + alert, badge et caches à jour. Vérifiés avec Playwright ou à la main (section 5).

---

## 3. Ordre d'implémentation recommandé

1. **MH-61-BE**, sur la branche actuelle, dans l'ordre de 2.1 :
   1. `PhotoRequired` et son test, `cargo check` ;
   2. `publish_decision` et ses tests ;
   3. repository, service, handler, router ;
   4. spec, artefacts, vérifications (section 5), contrat qualité (section 7).
2. L'utilisateur commit et merge MH-61-BE dans `develop`.
3. **MH-61-FE**, nouvelle branche depuis `develop`, dans l'ordre de 2.3 : types et `api.ts` → prédicat et tests → hook → `PublishListingAction` → `OwnerBar`.
4. L'utilisateur commit, merge et ferme #170.

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement MH-61 |
| ---- | -------- | ---------------- |
| `listings/repository.rs::find_listing_by_id` | Renvoie `owner_id`, `published_at`, `has_photo` | Réutilisé pour la décision (D3), pas de requête de lecture nouvelle. |
| `listings/repository.rs::push_public_visibility` / `service::is_visible_to` | Règle publique : publié + cover | Inchangés. Le bien publié entre dans le feed sans autre code. |
| `listings/service.rs::get_listing_detail` | Lecture owner-aware triée (`display_order_key`) | Réutilisé pour la réponse 200 (D6). |
| `listings/handler.rs` `OWNER_ROLES`, `require_role` | Motif de `update` / `update_status` / `delete` | Même motif ; pas de corps, donc pas d'`AppJson`. |
| `shared/errors.rs` | Bloc 409 (`CoverPhotoRequired`, `MediaOrderConflict`) | + `PhotoRequired`. |
| `route.rs` / `api_doc.rs` | Collecte automatique via `routes!` | Rien à enregistrer à la main. |
| Migration `20260927094513_listings_published_at.sql`, trigger `fn_set_updated_at` | Colonne nullable, `updated_at` par trigger | Pas de migration. Requête sur la PK : aucun index nouveau, rien sur le prix. |
| FE `OwnerBar`, `listingVisibility`, `VISIBILITY_BADGE` | Badge + message (MH-54-FE) | Bouton et messages de brouillon. |
| FE `Modal`, `DeleteListingAction`, `useDeleteListing` | Modale verrouillée, toast dans le hook | Modèles de `PublishListingAction` / `usePublishListing`. |
| FE `["listing", id]`, `["listings"]`, `["owner-listings"]` | Clés partagées détail / feed / « My properties » | Mises à jour sans rechargement manuel. |
| FE `ApiError.code`, `apiPost`, `useToast` | `shared/` | Réutilisés. |

---

## 5. Vérifications

**BE** : `cargo fmt`, `cargo check`, `cargo clippy`. `cargo test` seulement avec l'accord explicite de l'utilisateur, ciblé sur `listings::service` et `shared::errors`. Swagger (`/api/docs`) : `POST /listings/{id}/publish` sous `listings`, avec 200/401/403/404/409. Curl (APP_ENV=development) :

- brouillon avec une photo → 200, `published_at` renseigné ; `GET /listings/:id` anonyme → 200 ; `GET /listings` → le bien apparaît ;
- second appel → 200, même `published_at` (`SELECT published_at, updated_at`) ;
- brouillon sans photo → 409 `PHOTO_REQUIRED`, `published_at` toujours `NULL` ;
- id inconnu / bien d'un autre owner → même 404 `LISTING_NOT_FOUND` ; seeker ou admin → 403 ; sans jeton → 401 ;
- avant/après : seuls `published_at` et `updated_at` changent (`SELECT *`).

**FE** : `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Skill `run` ou Playwright headless :

- brouillon sans photo : bouton désactivé, message, lien photos ; avec photo : bouton actif ;
- clic → modale avec le texte d'irréversibilité ; Escape/backdrop ignorés et boutons inactifs pendant une requête lente (throttling) ;
- succès : modale fermée, toast, badge « Published », bouton absent ; feed et « My properties » à jour sans rechargement ;
- 409 forcé (supprimer la photo dans un autre onglet) : modale fermée, photos rechargées, toast info, bouton désactivé ;
- serveur coupé : modale ouverte, alert d'erreur ;
- bien publié ou masqué : aucun bouton ; clavier seul et 360 px OK.

---

## 6. Risques et inconnues

| #  | Risque / inconnue | Traitement |
| -- | ----------------- | ---------- |
| R1 | Course avec la suppression de la dernière photo. | Acceptée par le ticket : bien publié sans photo = masqué. Pas de verrou (D3). |
| R2 | Deux publications simultanées. | Garde `published_at IS NULL` : le premier `published_at` est conservé, les deux répondent 200. |
| R3 | `hasPhoto` FE = `media.length > 0`, BE = cover existe. | Équivalents par invariant (cover auto, `CoverPhotoRequired`). Signalé seulement. |
| R4 | Bien supprimé entre la lecture et l'`UPDATE`. | 0 ligne, puis `get_listing_detail` → 404. Correct. |
| R5 | Forme de la donnée en cache `["listing", id]` (enveloppe `{ data }` ou non) et signature d'`apiPost` sans corps. | À vérifier au début de MH-61-FE avant d'écrire `setQueryData`. |
| R6 | Un bouton `disabled` n'est pas focusable : l'explication doit rester lisible. | Message visible dans la barre + `aria-describedby`. |
| R7 | La spec §4.3 ne mentionne pas l'endpoint. | Ajout (D10). |
| R8 | Rôle non revalidé à chaque requête (seul `is_active` l'est). | Hors périmètre, connu : un owner fraîchement validé doit rafraîchir son jeton. |
| R9 | MCP PostgreSQL, Sequential Thinking et plugin GitHub en échec pendant l'analyse. | `gh` utilisé ; schéma lu dans les migrations. Réessayer le MCP PostgreSQL au début de MH-61-BE. |
| R10 | Monétisation non définie. | Aucune dépendance pour ce ticket. |

---

## 7. Contrat qualité (sur chaque branche, BE puis FE)

1. **Orientation** : `graphify query` / `path` / `explain` avant tout grep ou lecture. Relire ce plan et le ticket (`gh issue view 171` ou `172`).
2. **Context7**, avant d'écrire le code qui utilise une librairie, pour vérifier bonnes pratiques, sécurité et idiome de la version installée :
   - BE : `sqlx::query!` en `execute` (sqlx 0.8), `#[utoipa::path(post, …)]` sans `request_body` avec plusieurs réponses (utoipa 5), extracteurs `Path` / `State` d'axum 0.8 ;
   - FE : `useMutation`, `setQueryData` / `invalidateQueries`, callbacks de hook vs de `mutate()` après démontage (TanStack Query v5) ; bouton désactivé accessible et dialogue de confirmation (ARIA).
3. **Code minimal, KISS** :
   - BE : une variante d'erreur, une fonction pure, une requête, une fonction de service, un handler ;
   - FE : une fonction d'API, un prédicat, un hook, un composant nouveau, un composant modifié ;
   - rien d'autre que ce que le ticket demande.
4. **Fichier par fichier**, dans l'ordre de 2.1 puis 2.3, en montrant chaque fichier avant de passer au suivant. Règles actives : `.claude/rules/rust.md`, `database.md`, `general-coding.md` (BE) ; `react-typecrypt.md`, `general-coding.md` (FE). Commentaires d'une ou deux lignes, seulement où le code ne s'explique pas seul, sans citer un fichier de règles ou de doc.
5. **Tests unitaires sur la logique pure uniquement** (2.2 et 2.4) : aucune dépendance à la DB, aucun mock de la DB.
6. **Vérifications** de la section 5.
7. **Première review, avant tout fix** :
   - review dédiée par domaine : BE avec le skill `code-review-backend`, FE avec `code-review-frontend` ;
   - reproduire chaque finding avant de le corriger, et comparer avec `HEAD` ;
   - **appliquer les corrections confirmées avant de continuer**, puis refaire l'étape 6.
8. **Passe `humanizer:humanizer`** sur commentaires, doc comments, messages d'erreur, descriptions OpenAPI, toasts, textes de la modale et de la barre, amendement de la spec : ton humain, neutre, concis et fidèle au code.
9. **`graphify update .`**
10. **Mémoire** : noter la dette de tests de 2.2 et 2.4.
11. **Arrêt** : ni commit ni push. L'utilisateur relit, commit, merge et ouvre la PR.

### Outils vérifiés pendant l'analyse

- **Skills du repo** (`.claude/skills/`) : `code-review-backend`, `code-review-frontend`, `graphify`, `github-ticket`, `docker`, `readme`.
- **Règles du repo** (`.claude/rules/`) : `general-coding.md`, `rust.md`, `database.md`, `react-typecrypt.md`.
- **Skills de plugins utilisés** : `humanizer:humanizer`, `run`.
- **MCP disponibles** : Context7 (`mcp__context7__*`, `mcp__claude_ai_Context7__*`), Git MCP (lecture seule), Filesystem.
- **MCP en échec pendant l'analyse** : PostgreSQL, Sequential Thinking, plugin GitHub (remplacé par la CLI `gh`).

---

## 8. Hors périmètre

Dépublication (irréversible au MVP) ; bouton « Publish » dans « My properties » ou sur l'écran photos ; modération admin (EP-12) ; verrou ou transaction sur la publication (D3) ; tests d'intégration (epic dédiée).

---

## 9. Critères de fin

**MH-61-BE**

- [ ] `POST /listings/:id/publish` sur un brouillon avec photo → 200, `published_at` posé, forme `GET /listings/:id`
- [ ] Déjà publié → 200, `published_at` d'origine conservé
- [ ] Brouillon sans photo → 409 `PHOTO_REQUIRED`, reste brouillon
- [ ] 404 `LISTING_NOT_FOUND` identique (inconnu / autre owner) ; 403 seeker/admin ; 401 sans jeton
- [ ] Aucun autre champ modifié ; `updated_at` par trigger
- [ ] OpenAPI (200/401/403/404/409) ; `openapi.json` et `.sqlx` à jour ; spec §4.3
- [ ] Tests de 2.2 ; fmt, check et clippy propres

**MH-61-FE**

- [ ] Types régénérés ; `publishListing`
- [ ] Brouillon : bouton « Publish » ; désactivé + message + lien photos sans photo ; actif avec photo
- [ ] Confirmation avec texte d'irréversibilité ; verrouillée pendant la requête
- [ ] Succès : modale fermée, toast, badge « Published », bouton absent, feed et « My properties » à jour
- [ ] 409 : modale fermée, photos rechargées, message ; autre erreur : modale ouverte + message
- [ ] Publié ou masqué : pas de bouton
- [ ] Tests de 2.4 ; lint, ts:check, test et prettier propres
