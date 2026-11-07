# MH-72 — Modération des biens depuis le back-office : plan d'implémentation (BE + FE)

- **Epic** : EP-12 (#207)
- **Tickets** : MH-72 (#209), MH-72-BE (#210), MH-72-FE (#211, dépend de MH-72-BE)
- **Branches** :
  - BE : `mh-72-be-admin-listing-list-status-change-and-deletion` (origin, à jour avec `develop`)
  - FE : pas encore sur origin. À récupérer ou à demander avant de coder.
  - Ni commit ni push automatique.
- **CDC** : ADMIN-01, « L'admin peut lister, modifier le statut et supprimer tout bien de la plateforme ».
- **Documentation** (spec §1.1, §1.2, §4.7, ARCHITECTURE §5.2, §9) : traitée par MH-76, rien n'est modifié ici.

---

## 1. Contexte et objectif

L'admin ne peut aujourd'hui modérer un bien qu'en base. « Properties » est un libellé inerte dans l'en-tête admin, et `GET`, `PATCH`, `DELETE /admin/listings` figurent dans la spec §4.7 sans exister dans le code. Le ticket ajoute :

- **BE** : trois routes admin servies par le module `listings`. Une liste de tous les biens, brouillons et biens sans photo compris, avec une valeur `visibility` dérivée de la règle du feed. Un changement de statut et une suppression sur n'importe quel bien.
- **FE** : une page `/admin/listings` avec filtres dans l'URL, changement de statut et suppression par ligne.

L'admin modère sans éditer. Il ne voit ni la description ni les photos d'un bien, et `GET /listings/:id` continue de lui répondre 404 pour un bien non public.

### Décisions BE

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Emplacement | Les routes vivent dans `listings` (précédent ADR-12), avec le tag OpenAPI `admin`. `admin/router.rs` ne change pas, et aucun import inter-modules n'est ajouté (R-11). `PATCH` et `DELETE` partagent le chemin `/admin/listings/{id}`, donc un seul `routes!` pour les deux, comme `get_by_id`, `update`, `delete`. |
| D2 | Une seule règle de visibilité | `repository.rs` définit deux constantes SQL, `HAS_COVER` et `IS_PUBLIC` (le prédicat actuel de `push_public_visibility`). `push_public_visibility` garde sa signature et pousse `IS_PUBLIC`, donc le feed, la recherche et `contact` ne changent pas. La requête admin sélectionne `HAS_COVER AS has_photo` et `IS_PUBLIC AS is_public`. MH-74 et MH-75 modifieront `IS_PUBLIC` à un seul endroit, et `hidden` apparaîtra sans toucher à MH-72. |
| D3 | Valeur `visibility` | Fonction pure `admin_listing_visibility(is_published, has_photo, is_public)` dans `service.rs` : non publié donne `draft`, publié sans photo `no_photo`, `is_public` donne `public`, sinon `hidden`. Le type Rust s'appelle `AdminListingVisibility` (`serde(rename_all = "snake_case")`, `ToSchema`). C'est un enum Rust, pas un `ENUM` PostgreSQL : la valeur est dérivée, jamais stockée. |
| D4 | Filtres | Struct `AdminListingsQuery { status, visibility, owner_id, page, per_page }` lue par `AppQuery`, pas `Query` : c'est `AppQuery` qui donne `400 INVALID_QUERY_PARAM` sur une valeur inconnue ou un UUID invalide. `status` et `owner_id` réutilisent `ListingFilters` et `push_filters`. `visibility` passe par `push_visibility_filter`, un `match` sur quatre prédicats construits avec `HAS_COVER` et `IS_PUBLIC`. Un `owner_id` inconnu donne une page vide sans traitement particulier. |
| D5 | Forme d'un item | `AdminListingDto { id, title, type, city, neighborhood, price, status, visibility, published_at, created_at, owner }`, avec `owner: AdminListingOwnerDto { id, first_name, last_name, email }`, imbriqué comme `OwnerSummaryDto`. Ni `description` ni photo dans `AdminListingRow` : la garantie vient de la structure. Tri `l.created_at DESC, l.id DESC`, comme les autres listes. |
| D6 | Requêtes de lecture | `QueryBuilder`, car les filtres se combinent. Le `SELECT` est écrit une seule fois (`push_admin_listing_select`) et sert à la page et à la lecture par id. Pas de `JOIN listing_media`, le `EXISTS` suffit. `count_admin_listings` applique les mêmes filtres. |
| D7 | `PATCH` | Corps `ListingStatusRequest` et `parse_status` réutilisés tels quels (422 sur `status`). Nouvelle fonction `set_listing_status_by_id(pool, id, status) -> bool`, un `UPDATE … WHERE id = $1`, puis relecture de l'item. Aucune ligne, ou relecture vide, donne `ListingNotFound`. `update_listing_status` ne reçoit pas un `owner_id: Option<Uuid>` : un `None` oublié sauterait le contrôle de propriété sur la route owner. |
| D8 | `DELETE` | Nouvelle fonction `lock_listing(executor, id)`, sur le modèle de `lock_owned_listing`. La suite commune (liste des clés, `DELETE`, commit, `delete_storage_objects`) devient une fonction privée `delete_locked_listing`, appelée par `delete_listing` et `admin_delete_listing`. L'ordre SQL puis fichiers reste celui de `DELETE /listings/:id`. |
| D9 | Autorisation | `const ADMIN_ROLES: &[Role] = &[Role::Admin]` dans `listings/handler.rs`, `AuthUser` puis `user.require_role(ADMIN_ROLES)?` en tête de chaque handler. |
| D10 | Log | Dans le handler, après succès, une ligne par action : `tracing::info!(admin_id = %…, listing_id = %…, action = "set_status", status = ?…, "listing moderated")`, ou `action = "delete"`. Un `PATCH` qui remet le statut courant répond 200 et journalise aussi. |

### Décisions FE

| #   | Sujet | Décision |
| --- | ----- | -------- |
| D11 | Mise en page | Vrai `<table>` dans un conteneur `overflow-x-auto`, sur une largeur `max-w-6xl`. `<caption>` en `sr-only` et `<th scope="col">`. Le conteneur de défilement est focusable (`tabIndex={0}`, `role="region"`, `aria-label`) pour défiler au clavier. Pas de composant `Table` partagé tant que MH-73-FE n'en a pas besoin. |
| D12 | État dans l'URL | `useSearchParams` est la seule source pour `status`, `visibility`, `owner_id` et `page`. Module pur `adminListingsCriteria.ts` : `parse` ignore toute valeur inconnue, un `owner_id` qui n'est pas un UUID et une page hors de la plage `u32`, car l'API répondrait 400. Changer un filtre retire `page` et ajoute une entrée d'historique. Une page au-delà de la fin est remplacée par la dernière avec `replace`, comme dans `OwnerListings`. |
| D13 | Filtre owner | Le nom de l'owner dans chaque ligne est un lien vers `/admin/listings?owner_id=<id>` (ajout validé, hors AC de #211). Le chip reprend `OwnerFilterChip` et `ownerFilterLabel`. Le repli « this owner » s'applique quand aucune ligne ne porte le nom (validé). |
| D14 | Changement de statut | Pas d'optimisme : le radio reste contrôlé par la donnée serveur, il retrouve donc seul l'ancienne valeur en cas d'échec. `onSuccess` renvoie la promesse d'`invalidateQueries`, ce qui garde la mutation en cours jusqu'au refetch. Aucune fonction de patch du cache, et aucune garde `isMutating` : un refetch ne peut pas écraser un état local. Échec : toast d'erreur. |
| D15 | Réutilisation du contrôle | `ListingStatusControl` embarque le hook owner optimiste. Sa partie visuelle est extraite dans `ListingStatusSwitch` (`listingId`, `listingTitle`, `status`, `isPending`, `onChange`), exporté par `features/listings`. `ListingStatusControl` garde le hook et le rend. L'accessibilité existante (`aria-disabled`, radios `sr-only`) est conservée. |
| D16 | Suppression | Composant admin propre, car un 404 ne se traite pas comme côté owner, où il compte comme un succès. `mutationFn` attrape le 404 avec `isAlreadyDeleted` et renvoie `"gone"`, sinon `"deleted"`. `onSuccess` choisit le toast (« Listing deleted. » ou « This property no longer exists ») et invalide. Le modal nomme le bien, indique que la suppression est définitive, et reste verrouillé pendant la requête. Une autre erreur laisse le modal ouvert avec une `Alert`. |
| D17 | 404 sur le statut | Toast « This property no longer exists » et invalidation de la liste. |
| D18 | Verrou de ligne | Une clé de mutation commune `["admin-listing-action", id]` pour le statut et la suppression. La ligne lit `useIsMutating` et désactive ses actions. |
| D19 | Badges | Nouveau `adminListingVisibilityBadge.ts`, `Record` exhaustif sur le type généré, sur le modèle de `ownerRequestStatusBadge.ts`. `VISIBILITY_BADGE` n'est pas réutilisé : son `hidden` veut dire « sans photo », qui correspond au `no_photo` de l'admin. Le filtre propose les quatre valeurs, `hidden` compris, même s'il reste vide avant MH-74. |
| D20 | Cache | Clé `["admin", "listings", params]`, avec `keepPreviousData`. Une fonction `invalidatePublicListingQueries(queryClient, id)` exportée par `features/listings` invalide `["listings"]` (feed et recherche) et `["listing", id]`, pour que l'admin ne duplique pas ces clés. |
| D21 | Purge à la déconnexion | `AdminLayout.handleSignOut` ajoute `queryClient.removeQueries({ queryKey: ["admin"] })` (ajout validé, hors AC de #211). Cela couvre aussi les demandes owner déjà en cache. |
| D22 | En-tête | `AdminHeader` lit `useLocation`. « Owner requests » et « Properties » deviennent des liens avec un état actif calculé et `aria-current="page"`. Aujourd'hui « Owner requests » est actif en dur. « Users » reste inerte jusqu'à MH-73-FE. |

---

## 2. Découpage BE / FE et dépendances

- **MH-72-BE** n'a pas de dépendance. Il livre le contrat OpenAPI.
- **MH-72-FE** dépend du BE : `shared/api/types.ts` est généré depuis `frontend/docs-frontend/openapi.json` et ne s'édite pas à la main. Le FE peut démarrer sur ce qui ne dépend pas du contrat (étapes 2 et 3 de MH-72-FE). Le reste utilise les types générés et attend le BE.
- **Contrat** :
  - `GET /api/v1/admin/listings?status=&visibility=&owner_id=&page=&per_page=` : `200 PaginatedResponse<AdminListingDto>`, 400 `INVALID_QUERY_PARAM`, 401, 403.
  - `PATCH /api/v1/admin/listings/{id}` avec `{ "status": "available" | "unavailable" }` : `200 { "data": AdminListingDto }`, 400 (JSON invalide), 401, 403, 404 `LISTING_NOT_FOUND`, 422 `VALIDATION_FAILED`.
  - `DELETE /api/v1/admin/listings/{id}` : 204, 401, 403, 404 `LISTING_NOT_FOUND`.
  - `visibility` : `draft`, `no_photo`, `public`, `hidden`.
- **Tickets voisins** :
  - MH-73-FE ajoutera le second point d'entrée vers `?owner_id=`, sans changement ici.
  - MH-74 et MH-75 étendent la règle publique. Voir R2.
  - MH-76 documente le contrat.

---

## 3. Ordre d'implémentation recommandé

### MH-72-BE

1. `listings/model.rs` : `AdminListingVisibility` et `AdminListingRow` (`FromRow`, sans description ni photo).
2. `listings/dto.rs` : `AdminListingsQuery` (`IntoParams`), `AdminListingOwnerDto`, `AdminListingDto`, `AdminListingResponse`, et la conversion depuis la ligne.
3. `listings/repository.rs` :
   - constantes `HAS_COVER` et `IS_PUBLIC`, `push_public_visibility` réécrit dessus (D2) ;
   - `push_visibility_filter`, `push_admin_listing_select` ;
   - `count_admin_listings`, `list_admin_listings`, `find_admin_listing` ;
   - `set_listing_status_by_id`, `lock_listing`.
4. `listings/service.rs` :
   - `admin_listing_visibility` (pure) ;
   - `list_admin_listings`, `admin_set_listing_status`, `admin_delete_listing` ;
   - extraction de `delete_locked_listing`.
5. `listings/handler.rs` : `ADMIN_ROLES`, `admin_list`, `admin_update_status`, `admin_delete`, avec `#[utoipa::path(tag = "admin", …)]` et les logs (D10).
6. `listings/router.rs` : `routes!(handler::admin_list)` et `routes!(handler::admin_update_status, handler::admin_delete)`.
7. `cargo sqlx prepare` : les deux requêtes `query!` nouvelles entrent dans `.sqlx/`, versionné et lu par le build Docker (`SQLX_OFFLINE=true`).
8. `cargo run --bin gen_openapi -- ../frontend/docs-frontend/openapi.json`.

Aucune migration, aucune variable d'environnement, aucun nouvel index.

### MH-72-FE

1. `npm run generate:types:ci` pour obtenir les routes et `AdminListingDto`.
2. Déplacements vers `shared/`, sans changement de comportement :
   - `OwnerFilterChip` vers `shared/components/` ;
   - `ownerFilterLabel` vers `shared/utils/`, typé sur `{ owner: { id, first_name, last_name } }` ;
   - la lecture de `page` et le test d'UUID de `searchCriteria.ts` vers `shared/utils/`.

   `features/search` se met à jour et ses tests suivent. Le passage par `search/index.ts` tirerait `SearchPage` dans le chunk admin.
3. `features/listings` : `ListingStatusSwitch` (D15), `invalidatePublicListingQueries` (D20), export de `isAlreadyDeleted`.
4. `features/admin/adminListingsCriteria.ts` (D12), `adminListingVisibilityBadge.ts` (D19), avec leurs `.test.ts`.
5. `features/admin/api.ts` : types, `adminListingsQueryKey`, `listAdminListings`, `updateAdminListingStatus`, `deleteAdminListing`.
6. `features/admin/hooks/` : `useAdminListings`, `useUpdateAdminListingStatus` (D14, D17), `useDeleteAdminListing` (D16). Comme dans `useDeleteListing`, les effets de succès restent dans le hook, car la ligne se démonte.
7. `features/admin/components/` : `AdminDeleteListingAction`, `AdminListingRow`, `AdminListingsPage`. La page porte les états : chargement, erreur avec bouton Retry (`refetch`), vide, vide filtré avec effacement des filtres.
8. `app/router.tsx` : route paresseuse `listings` sous `/admin`. Le composant n'est pas réexporté par `features/admin/index.ts` (voir le commentaire de ce fichier).
9. `shared/components/SiteHeader.tsx` (D22) et `app/layout/AdminLayout.tsx` (D21).

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement |
| ---- | -------- | ---------- |
| `listings/repository.rs` | `push_public_visibility`, utilisé par `count_listings`, `list_listings` et `contact/repository.rs` | Prédicat déplacé dans `IS_PUBLIC`, signature inchangée |
| `listings/repository.rs` | `ListingFilters`, `push_filters` | Réutilisés pour `status` et `owner_id` |
| `listings/service.rs` | `parse_status`, `delete_listing` | `parse_status` réutilisé, `delete_listing` s'appuie sur `delete_locked_listing` |
| `listings/service.rs` | `is_visible_to` | Inchangé : `GET /listings/:id` reste en 404 pour l'admin sur un bien non public |
| `media/repository.rs` | Verrou `SELECT owner_id FROM listings WHERE id = $1 FOR UPDATE` | Inchangé : c'est la même ligne que `lock_listing`, donc upload et suppression admin restent sérialisés |
| `infra/storage/cleanup.rs` | `delete_storage_objects`, best-effort | Réutilisé |
| `shared/extractors.rs`, `shared/rbac.rs` | `AppQuery`, `AppJson`, `AuthUser`, `require_role` | Utilisés tels quels |
| `shared/errors.rs` | `InvalidQueryParam`, `ListingNotFound`, `Validation`, `Forbidden` | Aucun code d'erreur nouveau |
| `admin/router.rs`, `route.rs`, `api_doc.rs` | Tag `admin` et merge de `listings` déjà présents | Aucun changement |
| `ListingStatusControl.tsx` | Contrôle et hook owner réunis | Partie visuelle extraite (D15), comportement owner identique |
| `features/search` | `OwnerFilterChip`, `ownerFilterLabel`, helpers d'URL | Déplacés vers `shared/` |
| `shared/api/client.ts` | `apiGet`, `apiPatch`, `apiDelete`, `ApiError` | Utilisés tels quels |
| `shared/components` | `Modal`, `Badge`, `Pagination`, `Select`, `EmptyState`, `Alert`, `useToast` | Utilisés tels quels |
| `shared/utils/format.ts`, `features/listings` | `formatPrice`, `formatFullName`, `typeLabels`, `AVAILABILITY_BADGE` | Utilisés tels quels |
| `AdminHeader` | « Owner requests » actif en dur, « Properties » inerte | D22 |
| `AdminLayout` | `removeOwnerScopedQueries` seul à la déconnexion | D21 |

---

## 5. Risques et inconnues

- **R1** : `hidden` n'est pas atteignable avant MH-74 et MH-75. Aujourd'hui `IS_PUBLIC` vaut exactement « publié avec photo », donc le filtre `hidden` renvoie une page vide. Ce comportement est attendu par #210. La branche `hidden` n'est couverte que par le test de la fonction pure.
- **R2** : deux contraintes à transmettre à MH-74 et MH-75.
  - `IS_PUBLIC` doit rester autonome sur l'alias `l`, car `count_listings` et les comptages ne joignent pas `users`. Une extension passera par un `EXISTS` sur `users`, pas par `u.is_active`.
  - `is_visible_to` est une seconde copie de la règle, en Rust, pour `GET /listings/:id`. Modifier `IS_PUBLIC` ne suffira pas.
- **R3** : la valeur `visibility` et son filtre sont écrits à deux endroits (D2, D3, D4). Ils partagent les mêmes constantes, mais rien ne teste leur accord sans base. La matrice curl du §6 le vérifie. La dette sera notée pour l'epic d'intégration.
- **R4** : `cargo sqlx prepare` exige une base locale à jour. Une entrée manquante casse le build Docker, pas `cargo check` avec une base joignable.
- **R5** : pas d'index sur `created_at`, comme pour le feed. Aucune requête ne filtre sur le prix, R-07 n'est pas concerné. Le MCP PostgreSQL ne s'est pas connecté pendant l'analyse : `EXPLAIN` à faire via `psql` à l'implémentation, sans ajouter d'index au passage.
- **R6** : les extracteurs passent avant `require_role`. Un seeker qui envoie un filtre invalide ou un JSON malformé reçoit 400 et non 403. C'est déjà le cas des routes owner et cela ne divulgue rien. L'AC reste vrai sur une requête bien formée.
- **R7** : le tableau avec défilement horizontal (D11) s'écarte du dernier AC de #211, « same responsive behavior as the owner-request queue », et du contexte qui décrit cette file comme une table. Choix validé. #211 est à mettre à jour, comme pour les deux ajouts D13 et D21.
- **R8** : la note technique de #210 dit que le statut owner verrouille par `id AND owner_id`. Dans le code, seul le `DELETE` prend un verrou. Le statut est un `UPDATE` unique. Le `PATCH` admin suit le même modèle (D7).
- **R9** : avec un filtre de statut actif, une ligne qui change de statut quitte la liste après le refetch. C'est cohérent avec le filtre, mais peut surprendre. Pas de traitement spécial.
- **R10** : un refetch en échec après un `PATCH` réussi laisse l'ancien statut affiché avec l'erreur de chargement, jusqu'au Retry (conséquence de D14).
- **R11** : la navigation admin est masquée sous `sm`, sans menu mobile. C'est déjà le cas, et l'AC vise la largeur tablette. Rien à changer ici.
- **R12** : `RootLayout` pour un admin qui suit le lien « View » n'a pas été vérifié dans le code. À contrôler pendant le test Playwright.
- **R13** : le FE n'a ni React Testing Library ni jsdom, donc aucun test de composant. La dette sera notée en mémoire.

---

## 6. Vérifications

**BE**
- `cargo fmt`, `cargo check` et `cargo clippy`.
- Tests unitaires :
  - `admin_listing_visibility` : table de vérité complète, `hidden` compris ;
  - libellés JSON de `AdminListingVisibility` (`no_photo`) et rejet d'une valeur inconnue ;
  - `ADMIN_ROLES` : admin passe, seeker et owner donnent `Forbidden` ;
  - conversion ligne vers DTO : `owner` imbriqué, aucune clé `description` ni `cover_photo_url`.
- Curl scripté sur la stack locale :
  - 401 sans jeton et 403 seeker et owner, sur les trois routes ;
  - liste : brouillon, bien sans photo, bien public, tri, `total` aligné sur les pages ;
  - filtres seuls et combinés, chaque `visibility` en accord avec la valeur renvoyée, `hidden` vide ;
  - 400 sur `status`, `visibility` et `owner_id` invalides, page vide sur un `owner_id` inconnu ;
  - `PATCH` sur un bien publié et sur un brouillon, 422, 404, `published_at` et contenu inchangés ;
  - feed, recherche et détail à jour après un `PATCH` et un `DELETE` ;
  - `DELETE` : 204, fichiers supprimés, second appel en 404 ;
  - panne de stockage simulée : au pire un fichier orphelin, jamais une ligne vers un fichier absent ;
  - `GET /listings/:id` en 404 pour l'admin sur un brouillon ;
  - une ligne de log par action, avec admin, bien et action.
- `frontend/docs-frontend/openapi.json` régénéré, sans dérive.

**FE**
- `npm run lint`, `ts:check`, `test` et `prettier:check`.
- Tests vitest : `adminListingsCriteria` (lecture, écriture, valeurs inconnues, UUID et page invalides), `adminListingVisibilityBadge`, et les tests déplacés vers `shared/`.
- Playwright sur l'app réelle (connexion OTP via Mailhog) :
  - lien « Properties » et état actif, « Owner requests » inactif sur cette page ;
  - filtres, page et owner restaurés au rechargement et au retour arrière ;
  - lien sur le nom de l'owner, chip, retrait du chip, repli « this owner » ;
  - statut confirmé, échec avec retour à l'ancien statut et toast ;
  - suppression avec toast, y compris la dernière ligne de la dernière page ;
  - 404 sur les deux actions ;
  - actions d'une ligne désactivées pendant une requête ;
  - « View » présent seulement sur les lignes `public`, et rendu de la page détail pour l'admin (R12) ;
  - état vide, erreur et Retry ;
  - largeur tablette : défilement du tableau à la souris et au clavier ;
  - déconnexion : plus aucune clé `["admin", …]` en cache.
- Non-régression owner : contrôle de statut de « My properties ». Non-régression recherche : chip owner de `/search`.

---

## 7. Contrat qualité (sur chaque branche)

- **Context7** pour vérifier bonnes pratiques, sécurité et idiomatisme, via le connecteur claude.ai (le serveur local `context7` ne s'est pas connecté pendant l'analyse) :
  - BE : `QueryBuilder` et `Transaction` de sqlx, `utoipa::path` et `routes!`, extracteurs Axum ;
  - FE : promesse renvoyée par `onSuccess` et `useIsMutating` en TanStack Query v5, `useSearchParams` de React Router.
- **Code minimal, KISS** : aucune librairie ajoutée, aucun composant partagé sans second usage réel.
- **Tests** : unitaires sur la logique pure uniquement, sans dépendance à la DB.
- **Première review** avant tout fix (`code-review-backend` ou `code-review-frontend`, skills du repo). Chaque finding est reproduit, et le fix est appliqué avant de continuer.
- **Review dédiée par domaine** (backend sur la branche BE, frontend sur la branche FE), puis une passe `humanizer:humanizer` sur les commentaires et les textes visibles : ton humain, neutre, concis et fidèle au code.
- **Après le code** : `graphify update .` et note de dette de tests en mémoire.
- Ni commit ni push.
