# MH-60 — Réordonner les photos d'un bien : plan d'implémentation (BE + FE)

- **Epic** : EP-09 — Listings (#129)
- **Ticket parent** : MH-60 — Reorder listing photos (#167)
- **Sous-tickets** : MH-60-BE (#168) → MH-60-FE (#169)
- **Branches** : MH-60-BE se code sur `mh-60-be-persist-photo-order-and-define-the-display-order`, déjà créée depuis `develop` (elle ne dépend pas de MH-59). La branche MH-60-FE n'est créée qu'après le merge du BE, et MH-59 doit être mergé dans `develop` avant (voir R1). Il n'y a pas de branche parent. Aucun commit ni push automatique.
- **Déjà livré, et utilisé ici** : MH-50/51/52 (upload, cover, suppression de photo, verrou du bien), MH-56/57/58 (motifs `AppJson`, `FieldErrors`, mutations optimistes, garde `isMutating`), MH-59 (écran photos, encore sur `origin/mh-59-build-the-listing-photo-management-screen`).
- **CDC** : aucune exigence explicite sur l'ordre. Seul point repris : « photo de couverture […] affichée en tête de la page détail ».

---

## 1. Contexte et objectif

Aujourd'hui, l'ordre des photos n'est pas fiable :

- l'upload prend `position = COUNT(*)` (`media/service.rs:23-31`) ;
- la suppression ne renumérote pas ;
- donc, après une suppression puis un upload, deux photos peuvent avoir la même position. Rien en base ne l'interdit (pas d'index unique sur `(listing_id, position)`) ;
- `PATCH /listings/:id/cover` change `is_cover` sans toucher `position`. Pourtant, la seule requête qui trie (`listings/repository.rs:198-210`, `ORDER BY position ASC`) suppose que la cover est en position 0. Son doc comment est faux.

**Objectif** : une règle d'ordre d'affichage unique, un endpoint qui enregistre l'ordre voulu par l'owner, des positions qui ne se dupliquent plus à l'upload, et des contrôles de déplacement sur l'écran photos.

**Règle d'affichage** : la cover d'abord, puis `position` croissante, puis `created_at`, puis `id`.

### Décisions prises

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Branches | BE sur la branche actuelle. FE sur une nouvelle branche après le merge du BE (et de MH-59). |
| D2 | Où vit la règle d'ordre | Une fonction pure dans `backend/src/shared/media_order.rs`, utilisée par listings (détail) et media (réponse du PUT). Le SQL ne trie plus : au plus 5 lignes. |
| D3 | Module propriétaire | `media`, comme `PATCH /listings/:id/cover` : un seul module écrit `listing_media`. |
| D4 | Verrou | `repository::lock_listing_owner` (`SELECT owner_id FROM listings WHERE id = $1 FOR UPDATE`), le même que l'upload et le changement de cover. La suppression de photo verrouille aussi la ligne `listings` (`FOR UPDATE OF l, m`). |
| D5 | Position à l'upload | `MAX(position) + 1` (0 si aucune photo) au lieu de `COUNT(*)`. Calculée sous le verrou dans `persist_media`. |
| D6 | Pas de migration | Les doublons existants sont départagés par `created_at, id`. Le premier réordonnancement réécrit les positions de 0 à n−1. Aucun index unique ajouté (la suppression laisse des trous, ce qui reste valide). |
| D7 | Corps de requête | `AppJson` (JSON mal formé → 400). `media_ids: Option<Vec<Uuid>>` : un champ absent ou `null` donne 422, pas le 400 d'une désérialisation ratée. |
| D8 | Interaction FE | Boutons « Move earlier » / « Move later » sur chaque tuile hors cover, sans glisser-déposer ni dépendance nouvelle. Ils couvrent souris, tactile et clavier. |
| D9 | Sauvegarde FE | Optimiste, une requête par déplacement. Déplacements bloqués tant qu'une mutation photo est en cours. |
| D10 | EP-09 | La scope boundary (« No photo reordering ») reste telle quelle. La contradiction est seulement notée (R2). |
| D11 | TECHNICAL_SPEC | Une ligne dans la table §4.3, plus une phrase en §4.5 sur la règle d'ordre et la position à l'upload. Le reste vient de l'OpenAPI généré. |

---

## 2. Découpage BE / FE et dépendances

```
MH-60-BE (#168)  ──►  merge dans develop  ──►  MH-60-FE (#169)   (prérequis : MH-59 mergé)
  shared/media_order (règle pure)          │     types.ts régénéré
  AppError::MediaOrderConflict             │     api.reorderListingPhotos
  media : DTO, repo, service, handler      │     photoOrder.ts (logique pure + tests)
  upload : MAX(position) + 1               │     useReorderListingPhotos (optimiste)
  listings : détail trié par la règle      │     PhotoTile : contrôles de déplacement
  spec §4.3/§4.5, .sqlx, openapi.json      │     ListingPhotosPage : verrou, live region
```

Le FE dépend du BE pour l'endpoint, le code `MEDIA_ORDER_CONFLICT` et les types générés. Il dépend aussi de MH-59 pour l'écran photos.

### 2.1 MH-60-BE : fichiers, dans l'ordre (un fichier à la fois)

1. **`backend/src/shared/media_order.rs`** (nouveau) et **`shared/mod.rs`** :
   - `pub fn display_order_key(is_cover: bool, position: i16, created_at: DateTime<Utc>, id: Uuid) -> (bool, i16, DateTime<Utc>, Uuid)` renvoie `(!is_cover, position, created_at, id)` ;
   - les appelants font `rows.sort_by_key(|row| display_order_key(…))` ;
   - le type de date est à aligner sur celui du projet : vérifier `chrono` ou `time` dans `Cargo.toml` et les autres colonnes `TIMESTAMPTZ`.
2. **`backend/src/shared/errors.rs`** :
   - variante `MediaOrderConflict`, 409 `MEDIA_ORDER_CONFLICT`, dans le bloc 409 et dans `status_and_code` ;
   - message : « The photos changed since they were loaded. ».
3. **`backend/src/modules/media/dto.rs`** :
   - `ReorderMediaRequest { media_ids: Option<Vec<Uuid>> }` (`Deserialize`, `ToSchema`) ;
   - `MediaListResponse { data: Vec<MediaDto> }`.
4. **`backend/src/modules/media/model.rs`** : `OrderedMediaRow { id, url, is_cover, position: i16, created_at }`. C'est un type distinct, pour ne pas toucher aux requêtes qui utilisent `MediaRow`.
5. **`backend/src/modules/media/repository.rs`** (génériques sur `PgExecutor`, `query!`/`query_as!`) :
   - `upload_slot(executor, listing_id) -> (i64, Option<i16>)` : `SELECT COUNT(*) AS "count!", MAX(position) AS last_position FROM listing_media WHERE listing_id = $1`. `count_media` reste pour la suppression.
   - `list_listing_media(executor, listing_id) -> Vec<OrderedMediaRow>` : `SELECT id, url, is_cover, position, created_at FROM listing_media WHERE listing_id = $1`.
   - `set_media_positions(executor, listing_id, ids: &[Uuid])` : un seul `UPDATE` :
     ```sql
     UPDATE listing_media m
     SET position = (o.ordinality - 1)::smallint
     FROM unnest($2::uuid[]) WITH ORDINALITY AS o(id, ordinality)
     WHERE m.listing_id = $1 AND m.id = o.id
     ```
6. **`backend/src/modules/media/service.rs`** :
   - `upload_decision(existing_count, last_position: Option<i16>)` : le quota est inchangé ; `position = last_position.map_or(Some(0), |p| p.checked_add(1))`, et `None` donne `AppError::Internal` (R4). `check_upload_slot` appelle `upload_slot`.
   - `validate_media_ids(Option<Vec<Uuid>>) -> Result<Vec<Uuid>, AppError>` : un champ absent, vide ou contenant un doublon donne `FieldErrors` sur `media_ids`, puis 422.
   - `apply_order(current: Vec<OrderedMediaRow>, sent: &[Uuid]) -> Result<Vec<OrderedMediaRow>, AppError>` :
     - les ensembles d'ids (`HashSet`) et les longueurs doivent être égaux, sinon `MediaOrderConflict` ;
     - chaque ligne reçoit son index dans `sent` comme position ;
     - le résultat est trié par `display_order_key`.
   - `reorder_media(pool, listing_id, caller_id, media_ids)` :
     - `validate_media_ids` → `begin` → `lock_listing_owner` (`None` → `ListingNotFound`) → `verify_ownership` → `list_listing_media` → `apply_order` → `set_media_positions` → `commit` → les lignes ;
     - une erreur avant le commit abandonne la transaction, donc rien n'est modifié.
7. **`backend/src/modules/media/handler.rs`** : `reorder_media(State, AuthUser, Path(id), AppJson(payload))` :
   - `user.require_role(&[Role::Owner])?`, puis le service, puis `Json(MediaListResponse)` (motif de `promote_cover`) ;
   - `#[utoipa::path(put, path = "/listings/{id}/media/order", tag = "listings", request_body = ReorderMediaRequest, responses(200 MediaListResponse, 400, 401, 403, 404, 409, 422))]`.
8. **`backend/src/modules/media/router.rs`** : `.routes(routes!(handler::reorder_media)) // owner`.
9. **Listings, détail** :
   - `listings/model.rs` : `ListingMediaRow` reçoit `created_at` (non exposé dans `ListingMediaDto`).
   - `listings/repository.rs::find_media_for_listing` : sélectionne `created_at`, retire `ORDER BY` et corrige le doc comment.
   - `listings/service.rs` : les trois appels (`get_listing_detail`, `create_listing`, `update_listing`) passent par un petit helper qui trie avec `display_order_key`. Ainsi, `GET /listings/:id` renvoie la cover d'abord, même avec des positions dupliquées.
10. **Docs** (D11) : `docs/TECHNICAL_SPEC_MVP_v1.2.md`.
    - §4.3, nouvelle ligne : `PUT /listings/:id/media/order`, JWT, `owner`, « Réordonner les photos ».
    - §4.5, une phrase : ordre d'affichage (cover, position, upload, id) et position d'un upload après la dernière.
11. **Artefacts** : `cargo sqlx prepare` (`.sqlx/`, trois requêtes nouvelles ou modifiées) et `cargo run --bin gen_openapi` (`frontend/docs-frontend/openapi.json`).

### 2.2 MH-60-BE : tests unitaires (logique pure, sans DB)

- `shared/media_order.rs` :
  - la cover passe en tête même avec la plus grande position ;
  - les autres photos suivent `position` croissante ;
  - à position égale, le `created_at` le plus ancien passe d'abord ;
  - à position et `created_at` égaux, l'`id` décide.
- `media/service.rs` :
  - `validate_media_ids` : champ absent, liste vide, doublon → `Validation` sur `media_ids` ; liste valide → ok ;
  - `apply_order` :
    - correspondance exacte → positions 0..n−1 dans l'ordre envoyé, cover en tête ;
    - id manquant, id en trop, id inconnu de même longueur → `MediaOrderConflict` ;
    - doublon → conflit (le service valide d'abord, mais la fonction reste sûre seule) ;
  - `upload_decision` : `position_tracks_the_existing_count` devient `position_follows_the_last_position` (aucune photo → 0 ; trou après suppression : count 2, dernière 3 → 4 ; 6e photo refusée inchangé).
- `shared/errors.rs` : `MediaOrderConflict` → 409 `MEDIA_ORDER_CONFLICT` (motif des tests d'enveloppe existants).
- **Dette** (base réelle requise, epic tests d'intégration) : 200 et positions réécrites, rollback complet sur 409, 404 identique (inconnu / autre owner), sérialisation avec upload / suppression / cover, `GET` sur des positions dupliquées, upload après suppressions. Vérifiés à la main (section 5).

### 2.3 MH-60-FE : fichiers, dans l'ordre (un fichier à la fois)

Prérequis : MH-60-BE et MH-59 mergés dans `develop`, puis la branche FE créée depuis `develop`.

1. **`frontend/src/shared/api/types.ts`** : régénéré avec `npm run generate:types:ci`, jamais à la main.
2. **`frontend/src/features/listings/api.ts`** : `reorderListingPhotos(listingId, mediaIds)` → `apiPut<{ data: ListingMedia[] }>(\`/api/v1/listings/${encodeURIComponent(listingId)}/media/order\`, { media_ids: mediaIds })`.
3. **`frontend/src/features/listings/photoOrder.ts`** (+ `.test.ts`) :
   - `movePhoto(media, mediaId, direction: -1 | 1): ListingMedia[] | null` renvoie `null` si :
     - la photo est la cover ;
     - l'id est inconnu ;
     - le déplacement sort du tableau ou prend la place de la cover.
     Sinon, il renvoie un nouveau tableau avec `position = index`.
   - `withCoverFirst(media)` met la cover en tête et garde l'ordre relatif des autres photos.
4. **`frontend/src/features/listings/listingPhotos.ts`** : `isMediaOrderConflict(error)` (`ApiError` avec le code `MEDIA_ORDER_CONFLICT`), à côté de `isCoverRequiredError`, plus les messages :
   - « Your photos changed elsewhere. They've been reloaded. »
   - « Couldn't save the new order. Please try again. »
5. **`frontend/src/features/listings/hooks/useListingPhotoMutations.ts`** :
   - `useReorderListingPhotos(listingId)`, avec `mutationKey: [...LISTING_PHOTOS_MUTATION_KEY, listingId]`, donc le verrou « Finish » et `refetchAfterLastPhotoChange` s'appliquent. `retry: false`.
     - `onMutate` : `cancelQueries(["listing", id])`, garde l'ancien `media`, écrit le nouvel ordre par `updateListingMedia`.
     - `onError` :
       - conflit → `invalidateQueries(["listing", id])` et toast `info` ;
       - autre erreur → restauration de l'ancien `media` et toast `error`.
     - `onSuccess` : `media` remplacé par la réponse (ordre serveur).
     - `onSettled` : `refetchAfterLastPhotoChange`.
     - Les toasts vivent dans le hook (callbacks de hook, exécutés même après démontage).
   - Changement de cover : `onSuccess` applique `withCoverFirst` après la remise à jour de `is_cover`. La nouvelle cover passe en tête sans attendre le refetch.
6. **`frontend/src/features/listings/components/PhotoTile.tsx`** : nouvelles props `canMoveEarlier`, `canMoveLater`, `onMoveEarlier`, `onMoveLater`, `isMoveLocked`.
   - Deux boutons icône (lucide `ArrowLeft` / `ArrowRight`) avec `aria-label` `Move photo ${n} earlier` / `later`. La grille passe à 1 colonne sur mobile, d'où « earlier/later » plutôt que « left/right ».
   - Rien sur la tuile cover.
   - Bout de liste → `disabled`. Verrou en cours → `aria-disabled` et clic ignoré, pour que le focus clavier reste sur le bouton (R6).
7. **`frontend/src/features/listings/components/ListingPhotosPage.tsx`** :
   - verrou de déplacement = `isMutatingPhotos` (uploads, suppressions, cover et ordre) OU au moins une tuile d'upload encore en cours ;
   - l'ordre envoyé est calculé sur `listing.media` complet, pas sur la liste filtrée `displayablePhotos` (R5) ;
   - région `<p role="status" className="sr-only">` : « Photo moved to position X of N. » ;
   - après un déplacement qui atteint un bout, le focus passe sur l'autre bouton de la même tuile.
8. **`ListingDetail.tsx`** : aucun changement. Il garde l'ordre du serveur et partage la clé `["listing", id]`.

### 2.4 MH-60-FE : tests unitaires (vitest, logique pure)

- `movePhoto` :
  - plus tôt, plus tard ;
  - au-delà de la fin → `null` ;
  - vers la place de la cover → `null` ;
  - la cover ne bouge pas ;
  - id inconnu → `null` ;
  - positions renumérotées.
- `withCoverFirst` : cover déjà en tête (inchangé), cover au milieu, aucune cover.
- `isMediaOrderConflict` : 409 avec le bon code → vrai ; 409 `COVER_PHOTO_REQUIRED`, 500, `TypeError` → faux.
- **Dette** (pas de React Testing Library) : contrôles par tuile, verrou, live region, focus, rollback, toasts, refetch sur conflit, détail à jour. Vérifiés avec Playwright ou à la main (section 5).

---

## 3. Ordre d'implémentation recommandé

1. **MH-60-BE**, sur la branche actuelle :
   1. `shared/media_order` et ses tests, puis `MediaOrderConflict` (1–2), puis `cargo check` ;
   2. upload `MAX + 1` (5 `upload_slot`, 6 `upload_decision`) : corrige un bug existant, isolé et testé seul ;
   3. endpoint : DTO, model, repository, service, handler, router (3–8) ;
   4. tri du détail dans listings (9) ;
   5. docs, artefacts, vérifications (section 5), contrat qualité (section 7).
2. L'utilisateur commit, merge MH-60-BE dans `develop` et s'assure que MH-59 y est aussi.
3. **MH-60-FE**, sur une nouvelle branche depuis `develop`, dans l'ordre de 2.3 :
   1. types et `api.ts` ;
   2. logique pure et tests ;
   3. hook ;
   4. tuile, puis page.
4. L'utilisateur commit, merge et ferme #167.

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement MH-60 |
| ---- | -------- | ---------------- |
| `media/router.rs` | Route cover sur le chemin `/listings` (« listing_media keeps one writer module ») | Ajout de `reorder_media`, même convention (D3). |
| `media/repository.rs::lock_listing_owner` | Verrou de l'upload (`persist_media`) et de la cover | Réutilisé (D4). |
| `media/repository.rs` delete (`FOR UPDATE OF l, m`) | Verrouille aussi la ligne `listings` | Sérialisé avec le réordonnancement : un ordre envoyé après une suppression → 409. |
| `media/service.rs::verify_ownership` | Owner différent → `ListingNotFound` | Réutilisé : même 404 pour un id inconnu ou un bien d'un autre owner. |
| `upload_decision` / `check_upload_slot` | `position = count` | `MAX(position) + 1` (D5). Quota inchangé. |
| `listings/repository.rs::find_media_for_listing` | `ORDER BY position ASC`, doc comment faux | Tri en Rust par la règle partagée (D2). |
| `shared/validation.rs::FieldErrors` | 422 `VALIDATION_FAILED` avec `fields` | Réutilisé pour `media_ids`. |
| `shared/extractors.rs::AppJson` | JSON mal formé → 400 | Réutilisé (D7). `promote_cover` garde `Json` : hors périmètre (R11). |
| `api_doc.rs` / `route.rs` | Collecte automatique des chemins via `routes!` | Rien à enregistrer à la main. |
| FE `LISTING_PHOTOS_MUTATION_KEY`, `refetchAfterLastPhotoChange`, `updateListingMedia` | MH-59 | Réutilisés par le nouveau hook. |
| FE `useUpdateListingStatus` | Motif optimiste + rollback + toasts | Modèle pour `useReorderListingPhotos`. |
| FE `["listing", id]` | Partagé par l'écran photos et le détail | Le détail suit sans rechargement manuel. |
| FE `apiPut`, `ApiError.code` | `shared/api/client.ts` | Réutilisés. |
| FE `Toast` (`aria-live="polite"`) | Seul mécanisme d'annonce existant | Toasts pour les erreurs ; région `role="status"` dédiée pour les déplacements. |

---

## 5. Vérifications

**BE** : `cargo fmt`, `cargo check`, `cargo clippy`. `cargo test` ne se lance qu'avec l'accord explicite de l'utilisateur, ciblé sur `shared::media_order`, `shared::errors` et `media`. Swagger (`/api/docs`) : `PUT /listings/{id}/media/order` sous `listings`, avec 200/400/401/403/404/409/422. À contrôler avec curl (APP_ENV=development) :

- bien à 3 photos, ordre inversé → 200, cover en tête, positions 0..2 en base dans l'ordre envoyé ; `GET /listings/:id` → même ordre ;
- cover envoyée en dernier → 200, cover toujours renvoyée en tête, position stockée = 2 ;
- `{}`, `{"media_ids": null}`, `[]`, doublon → 422 avec `fields[media_ids]` ; corps `{` → 400 ;
- id manquant, id en trop, id d'un autre bien → 409 `MEDIA_ORDER_CONFLICT`, positions inchangées (`SELECT`) ;
- bien inconnu / bien d'un autre owner → même 404 `LISTING_NOT_FOUND` ; seeker ou admin → 403 ; sans jeton → 401 ;
- positions dupliquées créées à la main (`UPDATE … SET position = 1`) → `GET` stable et cover en tête ;
- upload de 3 photos, suppression de la 2e, upload → nouvelle position 3, dernière à l'affichage ;
- course : dans `psql`, `BEGIN; SELECT … FROM listings WHERE id = … FOR UPDATE;`, puis lancer le PUT (il attend), supprimer une photo dans la même transaction, `COMMIT` → le PUT répond 409.

**FE** : `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Vérification avec le skill `run` ou Playwright headless :

- souris : « Move later » sur la 2e photo → la grille change tout de suite ; un rechargement garde l'ordre ; le détail montre le même ordre sans rechargement manuel ;
- clavier seul : Tab jusqu'au bouton, Entrée, le focus reste sur un bouton actif ; lecteur d'écran (VoiceOver) : « Photo moved to position 3 of 4. » ;
- tactile (émulation mobile, 360 px) : boutons utilisables, pas de scroll horizontal ;
- tuile cover : badge, aucun bouton de déplacement ; la 2e tuile n'a pas « Move earlier » actif ;
- « Make cover » sur la 3e photo → elle passe en tête tout de suite ;
- pendant un upload, une suppression, un changement de cover ou une sauvegarde lente (throttling) → boutons inactifs ;
- 409 forcé (photo supprimée dans un autre onglet) → photos rechargées, toast d'info ;
- serveur coupé → ordre précédent restauré, toast d'erreur.

---

## 6. Risques et inconnues

| #   | Risque / inconnue | Traitement |
| --- | ----------------- | ---------- |
| R1  | MH-59 n'est pas dans `develop`, et MH-60-FE en dépend. | Le FE démarre après les merges de MH-60-BE et de MH-59. Si MH-59 change en review, adapter 2.3 (props de `PhotoTile`, verrou de la page). |
| R2  | La scope boundary d'EP-09 dit encore « No photo reordering ». | Laissée telle quelle (décision utilisateur). Seulement signalée. |
| R3  | `promote_cover` ne touche pas `position`, alors que le détail triait par `position`. | Corrigé par la règle d'affichage (cover d'abord). Côté FE, `withCoverFirst` évite le saut au refetch. |
| R4  | `MAX(position) + 1` grandit à chaque cycle suppression/upload sans réordonnancement (`SMALLINT`). | Inatteignable en pratique (32 767 cycles). `checked_add` → `Internal` plutôt qu'un doublon silencieux. |
| R5  | L'écran filtre `displayablePhotos` (`isRemoteMediaUrl`). Envoyer seulement ces ids donnerait 409 en boucle. | L'ordre envoyé est calculé sur `listing.media` complet. |
| R6  | Un bouton qui passe `disabled` perd le focus clavier pendant la sauvegarde. | `aria-disabled` + garde dans le handler pendant le verrou ; `disabled` réel seulement aux bouts de liste, avec focus déplacé sur l'autre bouton. |
| R7  | Un uuid mal formé dans `media_ids` → erreur de désérialisation → 400, pas 422. | Accepté : corps mal formé. Le noter dans la description OpenAPI du 400. |
| R8  | `query!` avec `$2::uuid[]` et `WITH ORDINALITY` (bigint → smallint), inférence de type sqlx 0.8. | Vérifier avec Context7 ; cast explicite déjà prévu. |
| R9  | MCP PostgreSQL et Sequential Thinking en échec pendant l'analyse ; plugin GitHub en échec (remplacé par `gh`). | Réessayer le MCP PostgreSQL au début de MH-60-BE, sinon s'appuyer sur la migration de base. Les requêtes filtrent sur `listing_id` (`idx_listing_media_listing`) : aucun index nouveau, rien sur `listings`, rien sur le prix. |
| R10 | Deux onglets réordonnent le même ensemble. | Les deux répondent 200, le dernier gagne. L'ensemble n'a pas changé, donc pas de 409 : conforme au ticket. |
| R11 | `promote_cover` lit le corps avec `Json`, pas `AppJson`. | Hors périmètre, à signaler seulement. |
| R12 | La CI frontend référence `docs/openapi.json`, qui n'existe pas ; le générateur écrit dans `frontend/docs-frontend/openapi.json`. | Hors périmètre, à signaler seulement. |
| R13 | Monétisation non définie. | Aucune dépendance pour ce ticket. |

---

## 7. Contrat qualité (sur chaque branche, BE puis FE)

1. **Orientation** : `graphify query`, `path` ou `explain` avant tout grep ou lecture. Relire ce plan et le ticket (`gh issue view 168` ou `169`).
2. **Context7**, avant d'écrire le code qui utilise une librairie, pour vérifier bonnes pratiques, sécurité et idiome de la version installée :
   - BE : transaction sqlx 0.8 (`pool.begin()`, `&mut *tx`, rollback au drop), `query!` avec un paramètre `&[Uuid]` et `unnest … WITH ORDINALITY`, `Option<Vec<Uuid>>` avec serde (champ absent / `null`), `#[utoipa::path(put, …)]` avec plusieurs réponses (utoipa 5), extracteur `Json` d'axum 0.8 ;
   - FE : `useMutation` optimiste (`onMutate` / `cancelQueries` / rollback / `onSettled`), `useIsMutating` avec `mutationKey` (TanStack Query v5) ; motif ARIA pour une liste réordonnable par boutons (live region, `aria-disabled`).
3. **Code minimal, KISS** :
   - BE : une fonction de règle, une variante d'erreur, trois requêtes, une fonction de service, un handler ;
   - FE : une fonction d'API, deux fonctions pures, un prédicat, un hook, deux composants modifiés.
   - Rien d'autre que ce que le ticket demande.
4. **Fichier par fichier**, dans l'ordre de 2.1 puis 2.3, en montrant chaque fichier avant de passer au suivant.
   - Règles actives : `.claude/rules/rust.md`, `database.md`, `general-coding.md` (BE) ; `react-typecrypt.md`, `general-coding.md` (FE).
   - Commentaires d'une ou deux lignes, seulement là où le code ne s'explique pas seul, sans citer un fichier de règles ou de doc comme justification.
5. **Tests unitaires sur la logique pure uniquement** (2.2 et 2.4) : aucune dépendance à la DB, aucun mock de la DB.
6. **Vérifications** de la section 5.
7. **Première review, avant tout fix** :
   - BE avec le skill `code-review-backend`, FE avec `code-review-frontend` (review dédiée par domaine) ;
   - reproduire chaque finding avant de le corriger, et comparer avec `HEAD` ;
   - **appliquer les corrections confirmées avant de continuer**, puis refaire l'étape 6.
8. **Passe `humanizer:humanizer`** sur les commentaires, doc comments, messages d'erreur, descriptions OpenAPI, toasts, libellés ARIA, annonces de la live region et amendements de la spec : ton humain, neutre, concis et fidèle au code.
9. **`graphify update .`**
10. **Mémoire** : noter la dette de tests de 2.2 et 2.4.
11. **Arrêt** : ni commit ni push. L'utilisateur relit, commit, merge et ouvre la PR.

### Outils vérifiés pendant l'analyse

- **Skills du repo** (`.claude/skills/`) : `code-review-backend`, `code-review-frontend`, `graphify`, `github-ticket`, `docker`, `readme`.
- **Règles du repo** (`.claude/rules/`) : `general-coding.md`, `rust.md`, `database.md`, `react-typecrypt.md`.
- **Skills de plugins utilisés** : `humanizer:humanizer`, `run`.
- **MCP disponibles** : Context7 (`mcp__context7__*`, `mcp__claude_ai_Context7__*`), Git MCP (lecture seule).
- **MCP en échec pendant l'analyse** : PostgreSQL, Sequential Thinking, plugin GitHub (remplacé par la CLI `gh`).

---

## 8. Hors périmètre

Glisser-déposer (D8) ; renumérotation à la suppression ; index unique sur `(listing_id, position)` ; migration de nettoyage (D6) ; correction de `promote_cover` vers `AppJson` (R11) ; modification du texte d'EP-09 (D10) ; ordre des photos dans le feed (seule la cover y figure) ; tests d'intégration (epic dédiée).

---

## 9. Critères de fin

**MH-60-BE**

- [ ] `PUT /listings/:id/media/order` → 200 avec les photos dans l'ordre d'affichage (id, url, is_cover, position)
- [ ] Positions 0..n−1 dans l'ordre envoyé ; cover inchangée et renvoyée en tête
- [ ] 422 `VALIDATION_FAILED` (`media_ids`) pour champ absent, vide ou doublon ; 400 pour JSON mal formé
- [ ] 409 `MEDIA_ORDER_CONFLICT` si l'ensemble diffère, sans aucune modification
- [ ] 404 `LISTING_NOT_FOUND` identique (inconnu / autre owner) ; 403 seeker/admin ; 401 sans jeton
- [ ] Verrou du bien partagé avec upload, suppression et cover
- [ ] `GET /listings/:id` dans l'ordre d'affichage, même avec des positions dupliquées
- [ ] Upload après suppressions → placé après toutes les photos existantes
- [ ] OpenAPI (tag listings, 200/400/401/403/404/409/422) ; `openapi.json` et `.sqlx` à jour ; spec §4.3/§4.5
- [ ] Tests de 2.2 ; fmt, check et clippy propres

**MH-60-FE**

- [ ] Types régénérés ; `reorderListingPhotos`
- [ ] Contrôles visibles sur chaque tuile hors cover ; souris, tactile et clavier ; annonce de la nouvelle position
- [ ] Cover en tête sans contrôle ; une nouvelle cover passe en tête
- [ ] Mise à jour immédiate ; succès sans autre changement ; 409 → rechargement + message ; autre erreur → ordre restauré + toast
- [ ] Déplacements bloqués pendant upload, suppression, cover ou sauvegarde
- [ ] Détail à jour sans rechargement manuel
- [ ] Tests de 2.4 ; lint, ts:check, test et prettier propres
