# MH-57 — Changer le statut d'un bien : plan d'implémentation (BE + FE)

- **Epic** : EP-09 — Listings (#129)
- **Ticket parent** : MH-57 — Change listing status (#144)
- **Sous-tickets** : MH-57-BE (#145) → MH-57-FE (#146)
- **Branches** : la branche parent `144-mh-57-…` est créée depuis `develop` **après le merge de la PR MH-56**. MH-56 modifie `OwnerListingRow`, que MH-57-FE modifie aussi. L'utilisateur crée les sous-branches BE puis FE depuis le parent et les y merge. Une seule PR part ensuite du parent vers `develop` (modèle MH-53 / MH-55 / MH-56).
- **Déjà livré, et utilisé ici** : MH-53 (validateur, `FieldErrors`), MH-55 (« My properties », zone d'actions de `OwnerListingRow`), MH-56 (`UPDATE … WHERE id AND owner_id RETURNING`, lien « Edit »).
- **Exigence CDC** : LIST-03, « Un owner peut changer le statut de son bien : Disponible / Non disponible ». Le rappel mensuel est différé en V2 (ARCHITECTURE R-10).

Chaque sous-ticket est implémenté dans sa propre session et sur sa propre sous-branche, selon le contrat qualité de la section 7.

---

## 1. Contexte et objectif

Un owner doit pouvoir signaler qu'un bien est loué, puis de nouveau libre. Le statut ne sert qu'à l'affichage : un bien `unavailable` reste dans le feed, avec une carte marquée (`ListingCard` le fait déjà) et une alerte sur le détail (`ListingDetail` aussi). Le statut ne change jamais la visibilité et ne touche jamais `published_at`.

- **BE** : `PATCH /listings/:id/status`, corps `{ "status": "available" | "unavailable" }`, réponse 200 `{ "data": { "id", "status" } }`. L'appel est idempotent et fonctionne aussi sur un brouillon.
- **FE** : un contrôle à deux options dans chaque ligne de « My properties », avec mise à jour optimiste et retour en arrière si le serveur refuse.

### Décisions prises

| #  | Sujet                     | Décision |
| -- | ------------------------- | -------- |
| D1 | Branche de base           | Parent créé depuis `develop` après le merge de MH-56. |
| D2 | Motif d'accessibilité     | Radios natifs en segments : `<fieldset>` avec une `<legend>` masquée visuellement, deux `<input type="radio">` et leurs `<label>`. Le navigateur fournit la navigation aux flèches et l'état `checked`. |
| D3 | TECHNICAL_SPEC            | Pas de mise à jour de §4.3. L'OpenAPI généré fait référence. |
| D4 | Lecture du statut         | `status: Option<String>`, parsé dans le service (comme `type` à la création). Un label inconnu ou absent donne un 422 avec une entrée `status`, pas un 400 de serde. |
| D5 | Écriture                  | Un seul `UPDATE listings SET status = $3 WHERE id = $1 AND owner_id = $2 RETURNING id, status`, sans lecture préalable. Aucune ligne → `ListingNotFound`. Une valeur identique renvoie quand même la ligne, d'où un 200 idempotent. |
| D6 | Portée de l'optimisme     | `setQueriesData` sur toutes les requêtes `["owner-listings", …]` (toutes les pages en cache), snapshot via `getQueriesData`, restauré dans `onError`. |
| D7 | Badge de disponibilité    | Dans la ligne, le contrôle remplace le badge de disponibilité. Le badge de visibilité reste. |

---

## 2. Découpage BE / FE et dépendances

```
MH-57-BE (#145)  ──►  merge dans le parent  ──►  MH-57-FE (#146)
  ListingStatus::from_label                 │      types.ts régénéré
  ListingStatusRequest / Dto / Response     │      api.updateListingStatus
  repository::update_listing_status         │      helpers purs de cache (+ tests)
  service::parse_status + set_listing_status│      useUpdateListingStatus (optimiste)
  handler PATCH + route + OpenAPI           │      ListingStatusControl
  .sqlx, openapi.json                       │      intégration dans OwnerListingRow
```

Le FE dépend du BE pour l'endpoint et pour les types générés. MH-55-FE est déjà livré.

### 2.1 MH-57-BE : fichiers, dans l'ordre (un fichier à la fois)

1. **`backend/src/modules/listings/model.rs`** : `ListingStatus::from_label(&str) -> Option<Self>`, sur le modèle de `ListingType::from_label`.
2. **`backend/src/modules/listings/dto.rs`**
   - `ListingStatusRequest { status: Option<String> }` (`Deserialize`, `ToSchema`). Le schéma marque `status` comme requis et typé `ListingStatus`, comme `ListingRequest` le fait pour ses champs requis.
   - `ListingStatusDto { id: Uuid, status: ListingStatus }` et `ListingStatusResponse { data: ListingStatusDto }` (`Serialize`, `ToSchema`).
3. **`backend/src/modules/listings/repository.rs`** : `update_listing_status(pool, id, owner_id, status) -> Result<Option<(Uuid, ListingStatus)>, AppError>`. `query!` avec `RETURNING id, status AS "status: ListingStatus"`, `fetch_optional`, même `map_err` que `update_listing`. Doc comment : `None` quand le bien n'existe pas ou appartient à un autre owner.
4. **`backend/src/modules/listings/service.rs`**
   - `parse_status(request: ListingStatusRequest) -> Result<ListingStatus, AppError>` : absent → `"status is required."` ; inconnu → `"status must be one of available, unavailable."`, via `FieldErrors` puis `finish()`.
   - `set_listing_status(pool, id, owner_id, request) -> Result<ListingStatusDto, AppError>` : `parse_status`, puis le repository, puis `ok_or(AppError::ListingNotFound)`.
5. **`backend/src/modules/listings/handler.rs`** : `update_status(State, AuthUser, Path(id), AppJson<ListingStatusRequest>)` avec `AppJson` en dernier. `user.require_role(OWNER_ROLES)?` passe avant tout. Retour `Json(ListingStatusResponse)` (200). `#[utoipa::path(patch, path = "/listings/{id}/status", tag = "listings", params(id), request_body = ListingStatusRequest, responses(200, 400, 401, 403, 404, 422))]`.
6. **`backend/src/modules/listings/router.rs`** : `.routes(routes!(handler::update_status)) // owner`.
7. **Artefacts** : `cargo sqlx prepare` (`.sqlx/`) et `cargo run --bin gen_openapi` (`frontend/docs-frontend/openapi.json`).

### 2.2 MH-57-BE : tests unitaires (logique pure, sans DB)

- `service.rs`, `parse_status` : `"available"` et `"unavailable"` sont acceptés ; `"rented"` et `"Available"` donnent un 422 sur `status` ; un `status` absent donne un 422 sur `status`. Le helper `violated_fields` existant est réutilisé.
- `handler.rs` : le test des rôles existant couvre aussi le changement de statut. Il faut le renommer, sans en écrire un nouveau.
- **Dette** (base réelle requise, epic tests d'intégration) : 200 idempotent, 404 identique pour un id inconnu et pour le bien d'un autre owner, `published_at` et les autres champs inchangés, brouillon accepté. Ces points sont vérifiés à la main avec curl (section 5).

### 2.3 MH-57-FE : fichiers, dans l'ordre (un fichier à la fois)

1. **`frontend/src/shared/api/types.ts`** : à régénérer avec `npm run generate:types:ci`, jamais à la main.
2. **`frontend/src/features/listings/api.ts`** : `updateListingStatus(id, status: ListingStatus)` appelle `apiPatch<{ data: ListingStatusDto }>(\`/api/v1/listings/${encodeURIComponent(id)}/status\`, { status })`. Il faut exporter le type `ListingStatusDto`.
3. **`frontend/src/features/listings/ownerListingsCache.ts`** (+ `.test.ts`) : deux fonctions pures sur `ListListingsResult | undefined`.
   - `withListingStatus(result, id, status)` : nouvel objet, seule la ligne visée change.
   - `withoutListing(result, id)` : nouvel objet sans la ligne visée.
4. **`frontend/src/features/listings/hooks/useUpdateListingStatus.ts`** : `useMutation({ mutationKey: ["listing-status"], mutationFn: (status) => updateListingStatus(id, status), retry: false, … })`.
   - `onMutate` : `cancelQueries(["owner-listings"])`, snapshot `getQueriesData(["owner-listings"])`, puis `setQueriesData(..., withListingStatus)`. Retourne le snapshot.
   - `onError` : restaure le snapshot. Sur un `ApiError` 404, `setQueriesData(..., withoutListing)` et un toast `info` (« This listing no longer exists. »). Sinon, un toast `error` (« Couldn't update the availability. Please try again. »).
   - `onSettled` : invalide `["listings"]` et `["listing", id]`, puis `["owner-listings"]` seulement si `queryClient.isMutating({ mutationKey: ["listing-status"] }) === 1`. Sans cette garde, un refetch écraserait l'état optimiste d'une autre ligne (R1).
5. **`frontend/src/features/listings/components/ListingStatusControl.tsx`**
   - Props : `listingId`, `listingTitle`, `status`.
   - Appelle `useUpdateListingStatus(listingId)`. Le hook est instancié par ligne, donc `isPending` ne bloque que cette ligne.
   - `<fieldset disabled={isPending}>` avec une `<legend className="sr-only">` « Availability of {title} », et deux radios `name={\`status-${listingId}\`}`, `checked={status === value}`, `onChange` → `mutate(value)`. Les libellés viennent de `AVAILABILITY_BADGE`. L'input est masqué visuellement (pas `display:none`), et le segment sélectionné porte le style. L'anneau de focus apparaît sur le segment en `focus-visible`.
6. **`frontend/src/features/listings/components/OwnerListingRow.tsx`** : le contrôle va dans la zone d'actions `relative z-10`, à côté de « Edit », et remplace le badge de disponibilité (D7). Sur mobile, la zone d'actions passe sous le contenu (R4).

### 2.4 MH-57-FE : tests unitaires (vitest, logique pure)

- `withListingStatus` : seule la ligne visée change, les autres gardent leur référence, l'entrée n'est pas mutée, un `undefined` est renvoyé tel quel, un id absent ne change rien.
- `withoutListing` : la ligne est retirée, les autres sont conservées, l'entrée n'est pas mutée, un `undefined` est renvoyé tel quel.
- **Dette** (pas encore de React Testing Library) : rendu et clavier du contrôle, verrouillage par ligne, retour en arrière et toast, retrait sur 404, invalidations. Ces points sont vérifiés à la main (section 5).

---

## 3. Ordre d'implémentation recommandé

1. Merger la PR MH-56 dans `develop`, puis créer le parent `144-mh-57-…` et la sous-branche BE (par l'utilisateur).
2. **MH-57-BE**, dans l'ordre de 2.1 : model, dto, repository, service, handler, router. Viennent ensuite les tests de 2.2, les artefacts, les vérifications de la section 5 et le contrat qualité.
3. L'utilisateur merge la sous-branche BE dans le parent et crée la sous-branche FE depuis le parent à jour.
4. **MH-57-FE**, dans l'ordre :
   1. les types et `api.ts` ;
   2. les helpers de cache et leurs tests ;
   3. le hook ;
   4. le contrôle ;
   5. l'intégration dans la ligne.
5. L'utilisateur merge la sous-branche FE dans le parent, puis ouvre une PR unique vers `develop`.

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement MH-57 |
| ---- | -------- | ---------------- |
| `listings/model.rs` | `ListingStatus` (`sqlx::Type`, serde lowercase), `ListingType::from_label` | `ListingStatus::from_label`. |
| `listings/service.rs` | `validate_listing` + `FieldErrors`, `violated_fields` dans les tests | `parse_status` suit le même modèle (422 `VALIDATION_FAILED`). |
| `listings/repository.rs` | `update_listing` : `UPDATE … WHERE id AND owner_id RETURNING`, `fetch_optional` | `update_listing_status`, même prédicat. |
| Triggers | `fn_set_updated_at`, `fn_update_listing_search_vector` (BEFORE UPDATE) | Ils se déclenchent aussi, ce qui est sans effet métier. Aucune migration. Index `idx_listings_status` inchangé. |
| `AppJson` / `AuthUser` / `require_role` / `OWNER_ROLES` | 400 / 401 / 403 | Réutilisés tels quels. |
| `AppError::ListingNotFound` | 404 `LISTING_NOT_FOUND` | Réutilisé pour un id inconnu et pour le bien d'un autre owner. |
| `shared/api/client.ts` | `apiPatch`, `ApiError.status` | Réutilisés. |
| `useOwnerListings` | Clé `["owner-listings", { page }]`, `keepPreviousData` | Patché de façon optimiste sur toutes les pages en cache. |
| `useListings` / `useListing` | Clés `["listings", …]` / `["listing", id]` | Invalidées dans `onSettled`. |
| `Toast` (`useToast().showToast`) | Variantes `error` (persistante) et `info` | Erreur → `error` ; 404 → `info`. |
| `labels.ts` | `AVAILABILITY_BADGE` (libellés et tons) | Libellés réutilisés par le contrôle. |
| `OwnerListingRow` | Lien étiré `after:inset-0`, zone d'actions `relative z-10` avec « Edit » | Le contrôle est ajouté dans la zone d'actions, au-dessus du lien étiré. |
| `ListingCard` / `ListingDetail` | Marque « Unavailable » / alerte warning | Inchangés : ils reflètent le statut après invalidation. |
| `removeOwnerScopedQueries` | Retire `["listing"]` et `["owner-listings"]` à la déconnexion | Inchangé. |

---

## 5. Vérifications

**BE** : `cargo fmt`, `cargo check`, `cargo clippy`. `cargo test` ne se lance qu'avec l'accord explicite de l'utilisateur, ciblé sur `listings`. Dans Swagger (`/api/docs`), `PATCH /listings/{id}/status` doit apparaître avec 200/400/401/403/404/422. À contrôler avec curl :

- owner, bien publié : `unavailable` → 200 `{ id, status }`. Refaire le même appel → 200. `published_at` et les autres champs sont inchangés (requête SQL).
- owner, brouillon → 200 ;
- `status` absent, `"rented"`, `"Available"` → 422 avec une entrée `status` ; JSON mal formé → 400 ;
- id inconnu et bien d'un autre owner → le même 404 `LISTING_NOT_FOUND` ;
- jeton seeker ou admin → 403 ; sans jeton → 401.

**FE** : `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Vérification à la main avec le skill `run`, ou Playwright headless :

- chaque ligne affiche le contrôle sur le bon statut. Tab entre dans le groupe, les flèches changent l'option, et le lecteur d'écran annonce la légende et l'option cochée ;
- un changement s'affiche tout de suite. Le contrôle de cette ligne seule est désactivé jusqu'à la réponse, et le statut tient après un rechargement ;
- feed et détail : la carte est marquée et l'alerte apparaît sans rechargement manuel ;
- 403 forcé (rôle périmé) ou serveur coupé : la ligne revient à son état, un toast d'erreur apparaît, les autres lignes ne changent pas ;
- bien supprimé ailleurs (SQL) puis changement de statut : la ligne disparaît et un toast info apparaît ;
- changement rapide sur deux lignes : les deux états finaux sont corrects ;
- largeur 360 px : ni débordement ni scroll horizontal.

---

## 6. Risques et inconnues

| #  | Risque / inconnue | Traitement |
| -- | ----------------- | ---------- |
| R1 | Mutations concurrentes sur deux lignes : le refetch déclenché par la première écrase l'état optimiste de la seconde. | Garde `isMutating({ mutationKey }) === 1` avant d'invalider `["owner-listings"]`. À confirmer via Context7 (TanStack Query v5). |
| R2 | `keepPreviousData` : une page en placeholder peut ne pas être celle qui est patchée. | `setQueriesData` patche toutes les pages en cache, et l'invalidation finale réconcilie. |
| R3 | Rôle périmé : un owner approuvé pendant la session reçoit 403 (mémoire « Role not revalidated per request »). | Retour en arrière et toast d'erreur. Pas de traitement spécifique. |
| R4 | Mise en page : contrôle + « Edit » dans une ligne déjà dense, sur mobile. | La zone d'actions passe à la ligne sous 640 px ; à vérifier à 360 px. |
| R5 | Accessibilité : des radios dans une `<li>` dont le lien est étiré par `::after`. | Le `relative z-10` de la zone d'actions garde les radios cliquables. L'anneau de focus de la ligne (`has-[a:focus-visible]`) ne s'applique pas aux radios, il faut donc un focus visible propre au segment. |
| R6 | Le MCP PostgreSQL n'a pas pu se connecter pendant l'analyse. | Réessayer au début de MH-57-BE. Sinon, s'appuyer sur la migration de base. L'`UPDATE` filtre sur la clé primaire : aucun nouvel index, et rien ne trie ni ne filtre sur le prix. |
| R7 | Ordre de validation : un corps invalide sur l'id d'un autre owner répond 422, pas 404. | Même comportement que le PUT de MH-56. Pas de fuite, car rien n'est écrit et l'existence du bien n'est pas révélée. |
| R8 | Monétisation non définie. | Aucune dépendance pour ce ticket. |

---

## 7. Contrat qualité (sur chaque branche, BE puis FE)

1. **Orientation** : `graphify query`, `path` ou `explain` avant tout grep ou lecture. Relire ce plan et le ticket (`gh issue view 145` ou `146`).
2. **Context7**, avant d'écrire le code qui utilise une librairie, pour vérifier les bonnes pratiques, la sécurité et l'idiome de la version installée :
   - BE : `query!` avec override de type `"status: ListingStatus"` sur `UPDATE … RETURNING` et `fetch_optional` (sqlx 0.8) ; `#[utoipa::path(patch, …)]` avec `params` Path et `request_body`, et un champ `Option<String>` exposé comme enum requis dans le schéma (utoipa 5) ; ordre des extracteurs `Path`/`AppJson` (axum).
   - FE : `onMutate` / `onError` / `onSettled`, `cancelQueries`, `getQueriesData` / `setQueriesData`, `isMutating` avec `mutationKey` (TanStack Query v5) ; motif radio group (WAI-ARIA APG) avec input masqué mais focusable.
3. **Code minimal, KISS** : rien d'autre que ce dont le ticket a besoin. BE : une requête, deux fonctions de service, un handler. FE : deux helpers purs, un hook, un composant.
4. **Fichier par fichier**, dans l'ordre de 2.1 puis 2.3, en montrant chaque fichier avant de passer au suivant. Les règles actives sont `.claude/rules/rust.md`, `database.md` et `general-coding.md` pour le BE, `react-typecrypt.md` et `general-coding.md` pour le FE. Les commentaires font une ou deux lignes, seulement là où le code ne s'explique pas seul, et ne citent jamais un fichier de règles ou de doc comme justification.
5. **Tests unitaires sur la logique pure uniquement** (2.2 et 2.4) : aucune dépendance à la DB, aucun mock de la DB.
6. **Vérifications** de la section 5.
7. **Première review, avant tout fix** :
   - BE avec le skill `code-review-backend`, FE avec `code-review-frontend` ;
   - reproduire chaque finding avant de le corriger, et comparer avec `HEAD` ;
   - **appliquer les corrections confirmées avant de continuer**, puis refaire l'étape 6.
8. **Passe `humanizer:humanizer`** sur les commentaires, doc comments, messages d'erreur, descriptions OpenAPI, toasts et textes d'UI ajoutés : ton humain, neutre, concis et fidèle au code.
9. **`graphify update .`**
10. **Mémoire** : noter la dette de tests de 2.2 et 2.4.
11. **Arrêt** : ni commit ni push. L'utilisateur relit, commit, merge et ouvre la PR.

### Outils vérifiés pendant l'analyse

- **Skills du repo** (`.claude/skills/`) : `code-review-backend`, `code-review-frontend`, `graphify`, `github-ticket`, `docker`, `readme`.
- **Skills de plugins utilisés** : `humanizer:humanizer`, `run`.
- **MCP disponibles** : Context7 (`mcp__context7__*`, `mcp__claude_ai_Context7__*`), Git MCP (lecture seule).
- **MCP en échec pendant l'analyse** : PostgreSQL (R6), le plugin GitHub (remplacé par la CLI `gh`), Sequential Thinking (pas nécessaire ici).

---

## 8. Hors périmètre

Filtre du feed par disponibilité (autre ticket EP-09) ; rappel mensuel LIST-03 (V2) ; suppression (MH-58) ; changement de statut depuis le détail ou l'`OwnerBar` (le ticket limite le contrôle à « My properties ») ; mise à jour de TECHNICAL_SPEC (D3) ; revalidation du rôle à chaque requête ; tests d'intégration (epic dédiée).

---

## 9. Critères de fin

**MH-57-BE**

- [ ] `PATCH /listings/:id/status` → 200 `{ data: { id, status } }`, idempotent, brouillons acceptés
- [ ] Statut absent ou label inconnu → 422 `VALIDATION_FAILED` avec une entrée `status` ; JSON mal formé → 400
- [ ] 404 `LISTING_NOT_FOUND` identique pour un id inconnu et pour le bien d'un autre owner ; 403 pour seeker et admin ; 401 sans jeton
- [ ] Un seul `UPDATE … WHERE id AND owner_id RETURNING id, status` ; `published_at` et les autres champs intacts
- [ ] OpenAPI 200/400/401/403/404/422 ; `openapi.json` et `.sqlx` à jour
- [ ] Tests de 2.2 ; fmt, check et clippy propres

**MH-57-FE**

- [ ] Types régénérés ; `updateListingStatus`
- [ ] Contrôle à radios natifs dans chaque ligne, utilisable au clavier, état annoncé, désactivé seulement pour la ligne en cours
- [ ] Mise à jour optimiste ; en cas d'erreur, retour en arrière et toast ; sur 404, ligne retirée et toast info
- [ ] `onSettled` invalide le feed, le détail et « My properties » (avec la garde R1)
- [ ] Tests de 2.4 ; lint, ts:check, test et prettier propres
