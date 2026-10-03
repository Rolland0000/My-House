# MH-56 — Modifier un bien : plan d'implémentation (BE + FE)  

- **Epic** : EP-09 — Listings (#129)
- **Ticket parent** : MH-56 — Edit a listing (#141)
- **Sous-tickets** : MH-56-BE (#142) → MH-56-FE (#143)
- **Branches** : `141-mh-56-edit-a-listing` est la branche parent (alignée sur `develop`, `c0d2857`). Les sous-branches BE puis FE n'existent pas encore : l'utilisateur les crée depuis le parent, puis les y merge. Une seule PR part ensuite du parent vers `develop` (modèle MH-53 / MH-55).
- **Livré, sur quoi ce ticket s'appuie** : MH-53 (création, validateur, normalisation), MH-54 (`published_at`, détail owner-aware, `OwnerBar`), MH-55 (« My properties », zone d'actions de `OwnerListingRow`).
- **Exigence CDC** : LIST-02 — « Un owner peut modifier et supprimer uniquement ses propres biens » (Must).

Ce plan couvre les deux sous-tickets. Chacun est implémenté dans sa propre session, sur sa propre sous-branche, en suivant le contrat qualité de la section 7.

---

## 1. Contexte et objectif

Un owner peut aujourd'hui créer un bien et le retrouver dans « My properties », mais pas le corriger. MH-56 ajoute l'édition, avec exactement les champs et les règles de la création. L'édition ne touche ni au statut de disponibilité (MH-57) ni à l'état de publication : un bien publié reste publié.

- **BE** : `PUT /listings/:id`. Remplacement complet des champs éditables, même validateur et même normalisation que `POST /listings`. Contrôle de propriété et écriture dans un seul `UPDATE … WHERE id = $1 AND owner_id = $2`. Un bien d'un autre owner répond le même 404 qu'un id inconnu. Réponse 200 dans la forme de `GET /listings/:id`.
- **FE** : le formulaire de création devient un formulaire partagé à deux modes. Route `/owner/listings/:id/edit`, préremplie depuis le détail, sauvegarde en PUT. Deux points d'entrée « Edit » : la ligne de « My properties » et l'`OwnerBar` du détail.

### Décisions tranchées

| #  | Sujet                        | Décision                                                                                                                                                                              |
| -- | ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| D1 | Branches                     | Sous-branches BE puis FE, mergées dans`141-mh-56-edit-a-listing`. Le FE régénère ses types depuis l'`openapi.json` du BE mergé dans le parent. Une seule PR vers `develop`. |
| D2 | Renommages BE                | Noms neutres partout :`CreateListingRequest` → `ListingRequest`, `validate_new_listing` → `validate_listing`, `NewListing` → `ListingFields`.                           |
| D3 | Navigation après sauvegarde | `navigate(\`/listings/${id}\`, { replace: true })`en mode édition : le retour arrière depuis le détail ne rouvre pas le formulaire soumis. La création garde son`push` actuel. |
| D4 | Specs                        | Dans MH-56-BE, TECHNICAL_SPEC §4.3 reçoit le corps et la réponse de`PUT /listings/:id` (codes 200/400/401/403/404/422).                                                           |

### Décisions de conception prises pendant l'analyse

- **Pas de lecture préalable** côté BE : l'`UPDATE` porte le prédicat de propriété et renvoie `id` (`fetch_optional`). Aucune ligne → `AppError::ListingNotFound`. Rien ne peut s'intercaler entre contrôle et écriture.
- **Colonnes écrites** : `title, description, type, price, city, neighborhood, surface_m2, rooms`, rien d'autre. `status`, `published_at`, `owner_id`, `created_at`, `currency` ne figurent pas dans le `SET`. Le prix reprend le cast `$n::bigint::numeric` de `insert_listing`.
- **Remplacement complet** : un `surface_m2` ou `rooms` absent du corps devient `NULL`. C'est ce qui permet au FE de vider ces champs.
- **Triggers existants** : `tg_listings_updated_at` (`fn_set_updated_at`) et `tg_listings_search_vector` (`BEFORE INSERT OR UPDATE`) couvrent `updated_at` et l'index de recherche. Aucune migration.
- **Réponse** : `service::get_listing_detail(pool, id, Some(owner_id))`, la lecture owner-aware de MH-54. Un brouillon édité répond donc 200, pas 404.
- **Champs étrangers ignorés** : `ListingRequest` n'a pas `deny_unknown_fields` ; `status`, `owner_id`, `published_at` dans le corps sont ignorés par serde. Le test existant `an_owner_id_in_the_body_is_ignored` est étendu (voir 2.2).
- **Rôle** : `user.require_role(OWNER_ROLES)` avant toute requête, comme `create`. Seeker et admin → 403 ; sans jeton → 401 (`AuthUser`).
- **FE, propriété** : `GET /listings/:id` renvoie 200 à n'importe qui pour un bien publié avec photo. La page d'édition doit donc comparer `profile.id` (`useProfile`) à `listing.owner.id`, exactement comme `ListingDetail` calcule `isOwner`, et afficher l'état « not found » sinon. Le BE reste seul juge à l'écriture (404 sur le PUT).
- **FE, préremplissage** : `reset(values)` une seule fois quand la requête du détail résout (garde par id de bien), pas via `defaultValues`. Un refetch en arrière-plan (focus de l'onglet) ne doit pas écraser une saisie en cours. Alternative à valider via Context7 : option `values` + `resetOptions: { keepDirtyValues: true }` de react-hook-form.
- **FE, valeurs numériques vides** : `surfaceM2`/`rooms` à `null` deviennent `NaN`, la même valeur qu'un champ vide lu par `valueAsNumber`. `toListingPayload` les omet déjà, ce qui donne `NULL` côté BE (remplacement complet).
- **FE, prix** : l'API renvoie un flottant ; le mapping applique `Math.round` pour que le formulaire garde un entier.
- **FE, quartier legacy** : `neighborhood: null` → `""`. `placeNameFieldRule` le rend déjà obligatoire, donc l'owner doit le remplir avant de sauvegarder.
- **Point d'entrée dans la ligne** : un `<Link>` texte « Edit » (wireframe MH-14 : bouton « Modifier » dans la ligne), dans la zone `relative z-10` prévue par MH-55, avec un `aria-label` qui inclut le titre. Jamais imbriqué dans le lien étiré.

---

## 2. Découpage BE / FE et dépendances

```
MH-56-BE (#142)  ──►  merge dans la branche parent  ──►  MH-56-FE (#143)
  renommages (ListingRequest, validate_listing,  │          types.ts régénéré (ListingRequest, PUT)
    ListingFields)                               │          api.updateListing + useUpdateListing
  repository::update_listing (UPDATE … owner)    │          mapping détail → valeurs (+ tests)
  service::update_listing + handler PUT          │          ListingForm partagé (create / edit)
  OpenAPI 200/400/401/403/404/422, TECH_SPEC,    │          page /owner/listings/:id/edit
    .sqlx, openapi.json                          │          Edit dans la ligne et dans l'OwnerBar
```

Le FE dépend du BE pour l'endpoint et pour le type renommé (`ListingRequest` dans `types.ts`). MH-54-FE (`OwnerBar`) et MH-55-FE (zone d'actions) sont déjà livrés.

### 2.1 MH-56-BE — Fichiers, dans l'ordre (un fichier à la fois)

1. **`backend/src/modules/listings/dto.rs`**
   - `CreateListingRequest` → `ListingRequest`. Doc comment : corps de `POST /listings` et de `PUT /listings/:id` ; `owner_id`, `status`, `published_at` éventuels sont ignorés.
2. **`backend/src/modules/listings/model.rs`**
   - `NewListing` → `ListingFields` (doc comment d'une ligne : champs éditables validés, communs à la création et à l'édition).
3. **`backend/src/modules/listings/repository.rs`**
   - `insert_listing` : prend `&ListingFields`.
   - `update_listing(pool, id, owner_id, fields: &ListingFields) -> Result<Option<Uuid>, AppError>` : `query_scalar!` avec `UPDATE listings SET title = $3, description = $4, type = $5, price = $6::bigint::numeric, city = $7, neighborhood = $8, surface_m2 = $9, rooms = $10 WHERE id = $1 AND owner_id = $2 RETURNING id`, `fetch_optional`. Doc comment : `None` quand le bien n'existe pas ou appartient à un autre owner.
4. **`backend/src/modules/listings/service.rs`**
   - `validate_new_listing` → `validate_listing(request: ListingRequest) -> Result<ListingFields, AppError>`.
   - `create_listing` : adapté aux nouveaux noms.
   - `update_listing(pool, id, owner_id, request) -> Result<ListingDetailDto, AppError>` : `validate_listing` → `repository::update_listing` → `ok_or(AppError::ListingNotFound)` → `get_listing_detail(pool, id, Some(owner_id))`.
5. **`backend/src/modules/listings/handler.rs`**
   - `update` : `State`, `Path(id)`, `AuthUser`, `AppJson<ListingRequest>` en dernier ; `user.require_role(OWNER_ROLES)?` puis `service::update_listing`. Retourne `Json(ListingDetailResponse { data })` (200).
   - `#[utoipa::path(put, path = "/listings/{id}", tag = "listings", params(("id" = Uuid, Path, ...)), request_body = ListingRequest, responses(200 body = ListingDetailResponse, 400, 401, 403, 404, 422))]`, sur le modèle de `create` et `get_by_id`.
   - `create` : `request_body = ListingRequest`. Renommer le test des rôles pour couvrir l'édition.
6. **`backend/src/modules/listings/router.rs`**
   - `.routes(routes!(handler::get_by_id, handler::update))` (même chemin `/listings/{id}`), commentaire « get is public, update is owner-only ». Le `TODO` ne garde que « delete ».
7. **Artefacts** : `cargo sqlx prepare` (cache `.sqlx/`), `cargo run --bin gen_openapi` (`frontend/docs-frontend/openapi.json`), TECHNICAL_SPEC §4.3 (D4).

### 2.2 MH-56-BE — Tests unitaires (logique pure, sans DB)

- `service.rs` : les tests existants du validateur suivent les renommages (`valid_request()` renvoie un `ListingRequest`).
- `service.rs` : `an_owner_id_in_the_body_is_ignored` devient un test qui désérialise un corps contenant `status`, `owner_id` et `published_at`, et vérifie que `validate_listing` renvoie les mêmes `ListingFields` que le corps propre (critère d'acceptation #142). Nécessite `#[derive(PartialEq)]` sur `ListingFields` si absent (sinon comparer champ par champ).
- `handler.rs` : le test des rôles couvre création, lecture des siens et édition (owner accepté ; seeker et admin en `Forbidden`).
- **Dette** (base réelle requise, epic tests d'intégration) : 404 pour un id inconnu et pour le bien d'un autre owner, `status`/`published_at`/`owner_id`/`created_at` inchangés, `updated_at` modifié, `search_vector` recalculé, `surface_m2`/`rooms` remis à `NULL` quand absents, brouillon édité répondant 200.

### 2.3 MH-56-FE — Fichiers, dans l'ordre (un fichier à la fois)

1. **`frontend/src/shared/api/types.ts`** : régénéré avec `npm run generate:types:ci`. Ne jamais le modifier à la main. `ListingRequest` remplace `CreateListingRequest` ; `put` apparaît sur `/api/v1/listings/{id}`.
2. **`frontend/src/features/listings/api.ts`** : `ListingRequest` remplace `CreateListingRequest` ; `createListing(body: ListingRequest)` ; `updateListing(id, body)` → `apiPut<{ data: ListingDetail }>(\`/api/v1/listings/${encodeURIComponent(id)}\`, body)`.
3. **`frontend/src/features/listings/listingFormValidation.ts`** (+ `.test.ts`)
   - `toCreateListingPayload` → `toListingPayload` (même corps).
   - `listingDetailToFormValues(listing: ListingDetail): ListingFormValues` : prix arrondi en entier, `surface_m2`/`rooms` nuls → `NaN`, `neighborhood` nul → `""`.
4. **`frontend/src/features/listings/hooks/useUpdateListing.ts`** : `useMutation({ mutationFn: (body) => updateListing(id, body), retry: false, onSuccess })`. `onSuccess` : `setQueryData(["listing", id], response)`, puis `invalidateQueries` sur `["listings"]` et `["owner-listings"]` (non attendus, comme `useCreateListing`).
5. **`frontend/src/features/listings/components/ListingForm.tsx`** (issu de `CreateListingForm.tsx`, renommé)
   - Props : `mode: "create" | "edit"`, `initialValues?: ListingFormValues`, `mutation` (résultat de `useCreateListing` ou `useUpdateListing`), `onCancel?: () => void`.
   - Textes par mode dans un `Record` : titre (« Publish a listing » / « Edit listing »), intro, bouton (« Publish listing » / « Save changes »).
   - `reset(initialValues)` une fois à l'arrivée des valeurs (voir section 1).
   - Inchangé : `fieldset disabled` pendant la requête, mapping 422 via `serverFieldToFormField`, bannière récapitulative, `requestErrorMessage`.
   - Prix : libellé « Monthly rent (XAF) » dans les deux modes.
   - En mode édition : bouton « Cancel » (`variant` secondaire, `type="button"`) qui appelle `onCancel`. Succès : `navigate(..., { replace: mode === "edit" })` (D3).
6. **`frontend/src/features/listings/components/CreateListingPage.tsx`** : `useCreateListing()` + `<ListingForm mode="create" mutation={...} />`. Remplace l'ancien `CreateListingForm` dans le router.
7. **`frontend/src/features/listings/components/EditListingPage.tsx`**
   - `useParams` → `id` ; `useListing(id)` ; `useProfile({ enabled: status === "authenticated" })`.
   - Chargement (détail ou profil) : `Spinner`, comme `ListingDetail`.
   - 404 sur le détail, `profile.id !== listing.owner.id`, ou 404 sur le PUT (`mutation.error`) : même `EmptyState` « not found » que `ListingDetail` (extraire ce bloc dans un petit composant partagé du feature si la duplication dépasse quelques lignes).
   - Autre erreur de chargement : `Alert variant="error"`.
   - Sinon : `<ListingForm mode="edit" initialValues={listingDetailToFormValues(listing)} mutation={useUpdateListing(id)} onCancel={() => navigate(\`/listings/${id}\`)} />`.
8. **`frontend/src/features/listings/components/OwnerListingRow.tsx`** : dans la zone d'actions, `<Link to={\`/owner/listings/${listing.id}/edit\`}>`« Edit » (icône`Pencil`de lucide-react,`aria-label` « Edit {title} »).
9. **`frontend/src/features/listings/components/OwnerBar.tsx`** : nouvelle prop `listingId` ; lien « Edit listing » vers `/owner/listings/:id/edit`. **`ListingDetail.tsx`** passe `listing.id`.
10. **`frontend/src/features/listings/index.ts`** : exporter `CreateListingPage`, `EditListingPage`, `useUpdateListing`, `updateListing`, `ListingRequest` ; retirer `CreateListingForm` et `CreateListingRequest`.
11. **`frontend/src/app/router.tsx`** : `owner/listings/new` pointe sur `CreateListingPage` ; nouvelle route `owner/listings/:id/edit` en `lazy`, `RequireAuth` > `RequireOwner` > `withSuspense(EditListingPage)`. Les redirections anonyme → `/login` et seeker/admin → `/` viennent de ces gardes.

### 2.4 MH-56-FE — Tests unitaires (vitest, logique pure)

- `listingDetailToFormValues` : prix entier conservé (et un prix flottant ramené à l'entier) ; `surface_m2` et `rooms` nuls → `NaN` ; `neighborhood` nul → `""` ; champs texte et type recopiés.
- `toListingPayload` : les tests existants suivent le renommage ; ajouter qu'un `NaN` de surface ou de pièces est omis (si pas déjà couvert).
- **Dette** (pas encore de React Testing Library) : préremplissage, refetch sans écrasement, verrouillage pendant la requête, 422 sous les champs, 404 et non-propriétaire → not-found, Cancel, mise à jour et invalidation des caches, liens « Edit ». À vérifier manuellement (section 5).

---

## 3. Ordre d'implémentation recommandé

1. **MH-56-BE** sur sa sous-branche : renommages d'abord (dto, model, service, handler compilent ensemble), puis `update_listing` (repository → service → handler → router), tests 2.2, artefacts (`.sqlx`, `openapi.json`, TECH_SPEC), vérifications BE, contrat qualité.
2. **Merge de MH-56-BE dans la branche parent** par l'utilisateur.
3. **MH-56-FE** sur sa sous-branche, créée depuis le parent à jour :
   1. types, `api.ts` ;
   2. `listingFormValidation` (renommage + mapping) et ses tests ;
   3. `useUpdateListing` ;
   4. `ListingForm` partagé puis `CreateListingPage`, et vérifier que la création n'a pas régressé avant d'aller plus loin ;
   5. `EditListingPage`, export, route ;
   6. points d'entrée (ligne, `OwnerBar`).
4. **Merge de MH-56-FE dans le parent**, puis une PR unique du parent vers `develop`.

---

## 4. Points d'intégration avec l'existant

| Zone                                          | Existant                                                                           | Changement MH-56                                                                          |
| --------------------------------------------- | ---------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `listings/dto.rs`                           | `CreateListingRequest` (champs `Option`, 422 plutôt que 400)                  | Renommé`ListingRequest`, partagé POST/PUT.                                            |
| `listings/service.rs`                       | `validate_new_listing`, `create_listing`, `get_listing_detail` (owner-aware) | Renommage ;`update_listing` réutilise le validateur et la lecture owner-aware.         |
| `listings/repository.rs`                    | `insert_listing` (cast `bigint::numeric`)                                      | `update_listing`, même cast, prédicat `owner_id`.                                   |
| Triggers                                      | `fn_set_updated_at`, `fn_update_listing_search_vector` sur UPDATE              | Réutilisés, aucune migration.                                                           |
| `AppJson` / `AuthUser` / `require_role` | 400 JSON mal formé ; 401 ; 403                                                    | Réutilisés tels quels.                                                                  |
| `AppError::ListingNotFound`                 | 404`LISTING_NOT_FOUND`                                                           | Réutilisé pour inconnu et non-propriétaire.                                            |
| Router listings                               | `/listings/{id}` = `get_by_id`                                                 | `update` ajouté sur le même chemin.                                                   |
| `CreateListingForm`                         | Formulaire de création                                                            | Devient`ListingForm` à deux modes + `CreateListingPage`.                             |
| `listingFormValidation.ts`                  | Règles,`toCreateListingPayload`, mapping 422                                    | `toListingPayload`, `listingDetailToFormValues`.                                      |
| `useListing`                                | Clé`["listing", id]`, pas de retry sur 404                                      | Réutilisé pour préremplir ; cache mis à jour par la réponse du PUT.                  |
| `useCreateListing`                          | Invalide`["listings"]` et `["owner-listings"]`                                 | Inchangé ;`useUpdateListing` suit le même modèle.                                    |
| `removeOwnerScopedQueries`                  | Retire`["listing"]` et `["owner-listings"]` au sign-out                        | Inchangé, couvre déjà le cache d'édition.                                             |
| `OwnerListingRow`                           | Zone d'actions vide en`relative z-10`                                            | Lien « Edit ».                                                                          |
| `OwnerBar` / `ListingDetail`              | Badge + message de visibilité ;`isOwner` via `useProfile`                     | Lien « Edit listing » ; même contrôle`isOwner` réutilisé dans la page d'édition. |
| `RequireAuth` / `RequireOwner`            | Redirections`/login` et `/`                                                    | Réutilisés pour la route d'édition.                                                    |

---

## 5. Vérifications

**BE** : `cargo fmt`, `cargo check`, `cargo clippy`. `cargo test` seulement avec l'accord explicite de l'utilisateur, ciblé sur `listings`. Dans Swagger (`/api/docs`), `PUT /listings/{id}` sous le tag listings avec 200/400/401/403/404/422 et `ListingRequest` comme corps de POST et PUT. Contrôle manuel avec `curl` :

- owner, son brouillon : PUT valide → 200, champs mis à jour, `published_at` toujours `null`, `status` inchangé ; un bien publié reste publié ;
- corps avec `status: "unavailable"`, `owner_id`, `published_at` : ignorés ;
- `surface_m2` absent → `null` dans la réponse ;
- `updated_at` modifié (requête SQL) ; recherche sur le nouveau titre via `search_vector @@ plainto_tsquery(...)` ;
- trois champs invalides → 422 avec trois entrées ; JSON mal formé → 400 ;
- id inconnu et bien d'un autre owner → même 404 `LISTING_NOT_FOUND` ;
- jeton seeker et admin → 403 ; sans jeton → 401.

**FE** : `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Vérification manuelle avec le skill `run` (ou Playwright headless, comme pour MH-54-FE) :

- création : comportement inchangé (validation, verrouillage, 422, redirection), libellé « Monthly rent (XAF) » ;
- « Edit » depuis une ligne de « My properties » et depuis l'`OwnerBar` → formulaire prérempli ; surface/pièces vides restent vides ;
- bien legacy sans quartier : champ vide, sauvegarde bloquée tant qu'il n'est pas rempli ;
- « Save changes » : champs verrouillés, détail mis à jour sans rechargement, retour arrière ne rouvre pas le formulaire ; « My properties » et le feed reflètent le changement ;
- « Cancel » : retour au détail sans sauvegarde ;
- 422 forcé (ex. titre de 4 caractères via devtools) : erreur sous le champ + bannière ;
- changement d'onglet pendant la saisie : la saisie n'est pas écrasée ;
- `/owner/listings/<id d'un autre owner>/edit` (publié ou non) et id inconnu : état not-found ; anonyme → `/login` ; seeker et admin → `/`.

---

## 6. Risques et inconnues

| #  | Risque / inconnue                                                                                                                                                           | Traitement                                                                                                                                                                                             |
| -- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| R1 | La page d'édition affiche les données d'un bien public d'un autre owner si le contrôle`profile.id === owner.id` est oublié ou évalué avant le chargement du profil. | Attendre le profil avant de rendre le formulaire ; état not-found sinon. Le PUT répond de toute façon 404 (pas de fuite en écriture).                                                              |
| R2 | Un refetch du détail (focus de l'onglet) appelle`reset` et efface la saisie.                                                                                             | Reset unique par id, ou`keepDirtyValues` ; à trancher via Context7 (react-hook-form v7).                                                                                                            |
| R3 | `NaN` comme valeur de `reset` sur un `<input type="number">`.                                                                                                         | La sanitization HTML d'un input number vide une valeur non numérique ; vérifier le rendu et`valueAsNumber` via Context7 et à la main.                                                             |
| R4 | Rôle périmé : un owner approuvé pendant la session reçoit 403 (mémoire « Role not revalidated per request »).                                                       | Déjà couvert par la fenêtre de réactivation MH-55 ;`requestErrorMessage` affiche le message `FORBIDDEN` existant.                                                                              |
| R5 | Édition concurrente (deux onglets) : dernier écrit gagnant, pas de contrôle de version.                                                                                  | Accepté au MVP ; hors ticket.                                                                                                                                                                         |
| R6 | Le MCP PostgreSQL n'a pas pu se connecter pendant l'analyse.                                                                                                                | Réessayer au début de MH-56-BE. Sinon, s'appuyer sur la migration de base et`cargo sqlx prepare`. L'`UPDATE` filtre sur la clé primaire : aucun nouvel index, pas de tri ni filtre sur le prix. |
| R7 | Renommer`CreateListingRequest` casse les imports FE tant que `types.ts` n'est pas régénéré.                                                                         | Régénération en premier fichier FE (2.3.1) ;`ts:check` le signale.                                                                                                                                |
| R8 | La création est refactorée en même temps que l'édition est ajoutée.                                                                                                    | Vérification manuelle de la création juste après l'étape 3.4, avant d'écrire la page d'édition.                                                                                                  |
| R9 | « Back to listings » sur le détail renvoie vers`/` (signalé dans MH-55).                                                                                              | Hors périmètre.                                                                                                                                                                                      |

---

## 7. Contrat qualité (à appliquer sur chaque branche, BE puis FE)

1. **Orientation** : `graphify query`, `graphify path` ou `graphify explain` avant tout grep ou lecture de fichier. Relire ce plan et le ticket (`gh issue view 142` ou `143`).
2. **Context7**, avant d'écrire le code qui utilise une librairie, pour vérifier bonnes pratiques, sécurité et idiome de la version installée :
   - BE : `query_scalar!` + `fetch_optional` sur `UPDATE … RETURNING` et cast `bigint::numeric` (sqlx 0.8) ; plusieurs méthodes sur un même chemin avec `routes!` (utoipa-axum) ; `request_body` + `params` Path dans `#[utoipa::path(put, …)]` (utoipa 5) ; ordre des extracteurs `Path`/`AppJson` (axum).
   - FE : `reset` vs `values` + `resetOptions.keepDirtyValues` et `valueAsNumber` avec `NaN` (react-hook-form v7) ; `setQueryData` + `invalidateQueries` dans `onSuccess` (TanStack Query v5) ; `navigate(..., { replace })` et `useParams` (react-router v7).
3. **Code minimal, KISS** : pas d'abstraction qui ne serve pas ce ticket. BE : une requête, une fonction de service, un handler. FE : un hook, un helper pur, un formulaire partagé et deux pages minces.
4. **Implémentation fichier par fichier**, dans l'ordre des sections 2.1 et 2.3, en montrant chaque fichier avant de passer au suivant. Règles actives :
   - BE : `.claude/rules/rust.md`, `.claude/rules/database.md`, `.claude/rules/general-coding.md` ;
   - FE : `.claude/rules/react-typecrypt.md`, `.claude/rules/general-coding.md` ;
   - commentaires d'une ou deux lignes, uniquement là où le code ne s'explique pas seul, sans citer de fichier de règles ou de doc comme justification.
5. **Tests unitaires sur la logique pure uniquement** (sections 2.2 et 2.4). Aucune dépendance à la DB, aucun mock de la DB.
6. **Vérifications** de la section 5.
7. **Première review, avant tout fix** :
   - BE : skill `code-review-backend` ; FE : skill `code-review-frontend` ;
   - reproduire chaque finding avant de le corriger, et comparer avec `HEAD`, car d'autres processus peuvent modifier l'arbre de travail ;
   - **appliquer les corrections confirmées avant de continuer**, puis relancer l'étape 6.
8. **Passe `humanizer:humanizer`** sur les commentaires, doc comments, messages d'erreur, descriptions OpenAPI et textes d'UI ajoutés : ton humain, neutre, concis, fidèle au code.
9. **`graphify update .`** pour garder le graphe à jour.
10. **Mémoire** : noter la dette de tests (2.2 et 2.4).
11. **Arrêt** : pas de commit ni de push. L'utilisateur relit, commit, merge la sous-branche dans le parent et ouvre la PR.

### Outils vérifiés pendant l'analyse

- **Skills du repo** (`.claude/skills/`) : `code-review-backend`, `code-review-frontend`, `graphify`, `github-ticket`, `docker`, `readme`.
- **Skills de plugins utilisés** : `humanizer:humanizer` (passe de ton), `run` (vérification manuelle FE).
- **MCP disponibles** : Context7 (`mcp__context7__*`, `mcp__claude_ai_Context7__*`), Git MCP (lecture seule).
- **MCP en échec** pendant l'analyse : PostgreSQL (R6), GitHub plugin (remplacé par la CLI `gh`), Sequential Thinking (pas nécessaire ici).

---

## 8. Hors périmètre

Changement de statut (MH-57) ; suppression (MH-58) ; publication (MH-61) ; écran photos (MH-59) ; format « XAF / month » des affichages (MH-62) ; contrôle de concurrence optimiste ; revalidation du rôle à chaque requête côté BE ; destination de « Back to listings » ; index sur le prix (R-07) ; tests d'intégration (epic dédiée).

---

## 9. Critères de fin

**MH-56-BE**

- [ ] `PUT /listings/:id` : même corps et même validation que `POST /listings` ; 422 `VALIDATION_FAILED` une entrée par champ ; 400 pour un JSON mal formé
- [ ] 200 avec la forme de `GET /listings/:id` (lecture owner-aware, brouillon compris)
- [ ] `status`, `published_at`, `owner_id`, `created_at` jamais modifiés, même présents dans le corps
- [ ] 404 `LISTING_NOT_FOUND` identique pour id inconnu et bien d'autrui ; 403 seeker/admin ; 401 sans jeton
- [ ] Un seul `UPDATE … WHERE id AND owner_id RETURNING id`, sans lecture préalable
- [ ] Renommages D2 ; OpenAPI 200/400/401/403/404/422 ; `openapi.json`, `.sqlx`, TECH_SPEC §4.3 à jour
- [ ] Tests de 2.2 ; fmt, check et clippy propres

**MH-56-FE**

- [ ] Types régénérés (`ListingRequest`, PUT)
- [ ] `ListingForm` partagé ; création inchangée ; « Monthly rent (XAF) » dans les deux modes
- [ ] `/owner/listings/:id/edit` : anonyme → `/login`, seeker/admin → `/`, non-propriétaire ou inconnu → not-found
- [ ] Préremplissage via `reset` ; surface/pièces vides restent vides ; quartier legacy vide et obligatoire
- [ ] « Save changes » verrouille les champs ; « Cancel » → détail ; succès → cache détail mis à jour, feed et « My properties » invalidés, redirection `replace` vers le détail
- [ ] 422 sous les champs + bannière ; 404 du PUT → not-found
- [ ] « Edit » dans chaque ligne de « My properties » et dans l'`OwnerBar`
- [ ] Tests de 2.4 ; lint, ts:check, test et prettier propres
