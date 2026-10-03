# MH-55 — Les biens du propriétaire (« My properties ») : plan d'implémentation (BE + FE)

- **Epic** : EP-09 — Listings (#129)
- **Ticket parent** : MH-55 — Owner's listings ("My properties") (#138)
- **Sous-tickets** : MH-55-BE (#139) → MH-55-FE (#140)
- **Branches** : `mh-55-owners-listings-my-properties` est la branche parent, alignée sur `develop` (`f6fbb89`). Les sous-branches BE puis FE sont créées par l'utilisateur et mergées dans la branche parent. Une seule PR part ensuite du parent vers `develop` (modèle MH-53).
- **Livré, sur quoi ce ticket s'appuie** : MH-53 (création, `POST /listings`), MH-54 (`published_at`, règle de visibilité publique, `listingVisibility`, `OwnerBar`, `removeOwnerScopedQueries`).
- **Tickets qui étendront cette page** : MH-56 (édition), MH-57 (statut), MH-58 (suppression), MH-62 (format de prix « XAF / month »).

Ce plan couvre les deux sous-tickets. Chacun est implémenté dans sa propre session, sur sa propre sous-branche, en suivant le contrat qualité de la section 7.

---

## 1. Contexte et objectif

Depuis MH-54, un bien créé reste un brouillon. Le feed public (`GET /listings`, filtre `owner_id` compris) ne renvoie que les biens publics. Un owner n'a donc aucun endroit pour retrouver ses brouillons ou ses biens publiés sans photo. Dans le header, « My properties » et « Add a property » sont encore du texte inerte.

MH-55 ajoute :

- **BE** : `GET /users/me/listings`. Il renvoie les biens de l'appelant, tous états confondus (brouillon, publié, masqué, disponible ou non), triés par `created_at` décroissant et paginés comme le feed. L'owner est toujours `AuthUser.user_id` ; il n'est jamais lu dans la requête. `published_at` est ajouté à `ListingSummaryDto`.
- **FE** : la page `/owner/listings`. Chaque ligne affiche la vignette, le titre, « ville · quartier », le prix, un badge de visibilité et un badge de disponibilité. La page gère le squelette de chargement, l'état vide, l'erreur et la pagination. La ligne réserve une zone d'actions à droite pour MH-56, MH-57 et MH-58. Les deux liens du header owner deviennent actifs.
- **FE, ajouté par décision (D3)** : une fenêtre qui demande à un owner tout juste approuvé de se reconnecter pour activer son compte owner.

### Décisions tranchées

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Branches | Sous-branches BE puis FE, mergées dans `mh-55-owners-listings-my-properties`. Le FE régénère ses types depuis l'`openapi.json` du BE mergé dans le parent. Une seule PR vers `develop`. |
| D2 | Page courante | Elle est stockée dans l'URL (`/owner/listings?page=2`). Elle survit au retour depuis la page de détail et au rechargement. Une valeur absente ou invalide vaut 1. Une page au-delà de `total_pages` est remplacée par la dernière page. |
| D3 | Rôle périmé après approbation | Un owner approuvé pendant sa session garde un jeton `seeker` pendant environ 15 min, et le BE lui renvoie 403 sur les routes owner. Quand le rôle du jeton vaut `seeker` et que le profil (lu en DB via `GET /users/me`) vaut `owner`, une fenêtre lui demande de se reconnecter pour activer son compte owner. Le bouton déconnecte puis redirige vers `/login`. Ce point ne figure pas dans les critères d'acceptation de #140 ; il faut l'y ajouter (voir R1). |
| D4 | Specs | Dans MH-55-BE, TECHNICAL_SPEC §4.3 reçoit l'exemple de réponse de `GET /users/me/listings` et le champ `published_at` du résumé. |

### Décisions de conception prises pendant l'analyse

- **Requête statique `query_as!` / `query_scalar!`** pour la liste et le compte. Il n'y a aucun filtre optionnel, donc pas de `QueryBuilder` (règles base de données). La requête reprend la forme du feed (mêmes colonnes, même struct `ListingSummaryRow`), avec un `LEFT JOIN` sur la couverture au lieu d'un `INNER JOIN`, et sans prédicat de visibilité.
- **Un seul DTO de résumé** : `published_at: Option<String>` est ajouté à `ListingSummaryDto`, formaté par `to_char` comme `created_at`. Dans le feed, il n'est jamais `null`.
- **Le handler vit dans le module listings**, monté sur `/users/me/listings`. Le module media fait déjà de même avec `/listings/{id}/cover`. Il n'y a pas de conflit avec le router `users`, qui ne déclare que `/users/me` et `/users/me/avatar`.
- **Pas de nouvel index** : le filtre `owner_id` s'appuie sur `idx_listings_owner`, vérifié dans la migration de base. Aucun tri ni filtre sur le prix.
- **Ligne cliquable sans lien englobant** : le titre est un `<Link>` étiré (pseudo-élément `after:absolute after:inset-0`) sur une ligne en `relative`. La zone d'actions est en `relative z-10`. Les boutons de MH-56, MH-57 et MH-58 ne seront donc jamais imbriqués dans un `<a>`, ce qui serait du HTML invalide et casserait la navigation clavier.
- **Les configurations de badges sont centralisées** dans `features/listings/labels.ts` : `VISIBILITY_BADGE` sort d'`OwnerBar`, et un nouveau `AVAILABILITY_BADGE` le rejoint. Les badges ont toujours un libellé texte.
- **Le signal photo** sur un résumé est `cover_photo_url !== null`, lu sur la valeur brute. Le garde `isRemoteMediaUrl` ne sert qu'à l'affichage de la vignette, comme dans `ListingCard`.
- **Le prix** utilise `formatPrice` et le suffixe « FCFA / month », comme `ListingCard`. MH-62 passera tous les affichages en « XAF / month ».
- **Le lien actif du header** n'est recalculé que dans le header owner, avec `useLocation` : « Listings » est actif sur `/`, « My properties » sur `/owner/listings`. Les headers public et seeker ne changent pas.
- **La page reste dans `features/listings`**, comme le formulaire de création (MH-53). Le wireframe MH-14 mentionne un dossier `features/owner-listings`, mais ce découpage n'a jamais été appliqué. La clé de cache `["owner-listings", params]` suit le ticket.
- **Le rôle lu dans le jeton ne sert qu'à l'affichage** (D3). Le front décode le payload du JWT sans le vérifier, et ne s'en sert que pour décider d'afficher la fenêtre. Le BE reste seul juge des droits. Un jeton illisible donne `null`, et aucune fenêtre ne s'affiche.

---

## 2. Découpage BE / FE et dépendances

```
MH-55-BE (#139)  ──►  merge dans la branche parent  ──►  MH-55-FE (#140)
  published_at dans le résumé (feed inclus)  │              types.ts régénéré
  repository : list/count des biens d'un owner│              api + hook useOwnerListings
  service + handler GET /users/me/listings    │              page /owner/listings (?page=)
  OpenAPI 200/401/403, TECH_SPEC, .sqlx       │              badges partagés, ligne extensible
                                              │              liens du header, invalidation du cache
                                              │              fenêtre de réactivation owner (D3)
```

Le FE dépend du BE pour l'endpoint et pour `published_at` dans `ListingSummaryDto`. La fenêtre D3 n'a besoin d'aucun changement BE : le rôle courant vient déjà de `GET /users/me`, lu en DB, et le rôle du jeton vient du claim `role` du JWT (`shared/crypto/jwt.rs`).

### 2.1 MH-55-BE — Fichiers, dans l'ordre (un fichier à la fois)

1. **`backend/src/modules/listings/model.rs`**
   - Ajouter `published_at: Option<String>` à `ListingSummaryRow`. Doc comment d'une ligne : formaté comme `created_at` ; `NULL` signifie brouillon.
2. **`backend/src/modules/listings/repository.rs`**
   - Feed (`list_listings`) : ajouter `to_char(l.published_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS published_at` au `SELECT`.
   - `count_owner_listings(pool, owner_id) -> Result<i64, AppError>` : `query_scalar!` avec `SELECT COUNT(*) AS "count!" FROM listings WHERE owner_id = $1`.
   - `list_owner_listings(pool, owner_id, limit, offset) -> Result<Vec<ListingSummaryRow>, AppError>` : `query_as!` avec les colonnes du feed et `published_at`. `LEFT JOIN listing_media lm ON lm.listing_id = l.id AND lm.is_cover`, `WHERE l.owner_id = $1`, `ORDER BY l.created_at DESC LIMIT $2 OFFSET $3`. Surcharges de type à vérifier via Context7 : `"listing_type: ListingType"`, `"status: ListingStatus"`, `"price!"`, `"cover_photo_url?"` (venant du `LEFT JOIN`), `"owner_id!"`.
   - Doc comment : toutes les lignes de l'owner, sans règle de visibilité publique.
3. **`backend/src/modules/listings/dto.rs`**
   - `ListingSummaryDto.published_at: Option<String>` (doc : ISO 8601 UTC, ou `null` pour un brouillon), recopié dans `From<ListingSummaryRow>`.
   - `OwnerListingsQuery { page: Option<u32>, per_page: Option<u32> }` avec `Deserialize` et `IntoParams` (`parameter_in = Query`). Il n'existe pas de struct de pagination partagée : `ListOwnerRequestsQuery` déclare aussi ses propres champs.
4. **`backend/src/modules/listings/service.rs`**
   - `list_owner_listings(pool, owner_id, query) -> Result<PaginatedResponse<ListingSummaryDto>, AppError>` : compte, `PaginationMeta::new`, liste, mapping. Même structure que `list_listings`.
5. **`backend/src/modules/listings/handler.rs`**
   - `list_mine` : `AuthUser`, `user.require_role(...)` puis `service::list_owner_listings(state.db(), user.user_id, query)`.
   - `#[utoipa::path(get, path = "/users/me/listings", tag = "listings", params(OwnerListingsQuery), responses(200 body = PaginatedResponse<ListingSummaryDto>, 401, 403))]`, sur le modèle de `create`.
   - Renommer `LISTING_WRITE_ROLES` en `OWNER_ROLES`, puisque la constante sert maintenant à une lecture. Adapter le test existant.
6. **`backend/src/modules/listings/router.rs`**
   - `.routes(routes!(handler::list_mine))`, avec un commentaire d'une ligne : servi sur le chemin users pour que la lecture des biens reste dans le module listings. Mettre à jour le `TODO`.
7. **Artefacts** : `cargo sqlx prepare` (cache `.sqlx/`), `cargo run --bin gen_openapi` (`frontend/docs-frontend/openapi.json`), TECHNICAL_SPEC §4.3 (D4).

### 2.2 MH-55-BE — Tests unitaires (logique pure, sans DB)

- `handler.rs` : le test des rôles, renommé, couvre maintenant aussi la lecture (owner accepté ; seeker et admin en `Forbidden`).
- `dto.rs` : un `ListingSummaryDto` construit depuis une ligne avec `published_at: None` se sérialise en `"published_at": null`, pas en champ absent. La valeur `Some` est recopiée telle quelle.
- La pagination est déjà couverte par les tests de `PaginationMeta`. Pas de nouveau test.
- **Dette** (base réelle requise, epic tests d'intégration) : pas de fuite vers les biens d'un autre owner, brouillons et biens sans photo inclus, ordre `created_at DESC`, `total` aligné sur les pages, `published_at` non nul dans le feed.

### 2.3 MH-55-FE — Fichiers, dans l'ordre (un fichier à la fois)

1. **`frontend/src/shared/api/types.ts`** : régénéré avec `npm run generate:types:ci`. Ne jamais le modifier à la main. `published_at` doit apparaître sur `ListingSummaryDto`, et `/users/me/listings` dans `paths`.
2. **`frontend/src/features/listings/api.ts`** : `ListOwnerListingsParams { page?: number; perPage?: number }` et `listOwnerListings(params)` → `apiGet<ListListingsResult>("/api/v1/users/me/listings", { page, per_page })`.
3. **`frontend/src/features/listings/hooks/useOwnerListings.ts`** : `useQuery({ queryKey: ["owner-listings", params], queryFn, placeholderData: keepPreviousData })`.
4. **`frontend/src/features/listings/ownerListingsPage.ts`** (+ `.test.ts`) : `parsePageParam(value: string | null): number`, fonction pure qui renvoie un entier ≥ 1 et 1 pour une valeur absente, non numérique, nulle, négative ou décimale.
5. **`frontend/src/features/listings/labels.ts`** : `VISIBILITY_BADGE` (déplacé depuis `OwnerBar`) et `AVAILABILITY_BADGE` (`available` : success, « Available » ; `unavailable` : error, « Unavailable », même ton que `ListingCard`).
6. **`frontend/src/features/listings/components/OwnerBar.tsx`** : importe `VISIBILITY_BADGE` depuis `labels.ts`. Aucun changement visible.
7. **`frontend/src/features/listings/components/OwnerListingRow.tsx`** : une ligne en `relative` avec, dans l'ordre :
   - la vignette de couverture (garde `isRemoteMediaUrl`) ou un placeholder `ImageOff` ;
   - le titre, en `<Link>` étiré vers `/listings/:id` ;
   - « ville · quartier » ;
   - le prix (`formatPrice` et « FCFA / month ») ;
   - le badge de visibilité (`listingVisibility({ publishedAt, hasPhoto: cover_photo_url !== null })`) et le badge de disponibilité ;
   - à droite, une zone d'actions en `relative z-10`, vide pour l'instant.
8. **`frontend/src/features/listings/components/OwnerListings.tsx`** : la page.
   - En-tête : titre « My properties », `DimensionRule`, et un bouton-lien « + Publish a listing » vers `/owner/listings/new`.
   - `useSearchParams` puis `parsePageParam` donnent `page` ; `onPageChange` met à jour `?page=`.
   - Chargement : une liste de `Skeleton` (`line` ou `block`).
   - Erreur : `Alert variant="error"`.
   - `total === 0` : `EmptyState` avec l'action « Publish your first listing » vers `/owner/listings/new`.
   - `total > 0` mais `page > total_pages` : `setSearchParams` en mode `replace` vers la dernière page.
   - `Pagination` si `total_pages > 1`.
9. **`frontend/src/features/listings/index.ts`** : exporter `OwnerListings`, `useOwnerListings`, `listOwnerListings` et les types.
10. **`frontend/src/app/router.tsx`** : route `owner/listings` en `lazy`, `RequireAuth` > `RequireOwner` > `withSuspense(OwnerListings)`. `RequireAuth` redirige déjà l'anonyme vers `/login`, et `RequireOwner` le seeker et l'admin vers `/`.
11. **`frontend/src/features/listings/hooks/useCreateListing.ts`** : invalider aussi `["owner-listings"]` dans `onSuccess`.
12. **`frontend/src/features/listings/removeOwnerScopedQueries.ts`** : retirer aussi `["owner-listings"]`.
13. **`frontend/src/shared/components/SiteHeader.tsx`** (header owner seulement) :
    - `useLocation` détermine le lien actif (« Listings » sur `/`, « My properties » sur `/owner/listings`) ;
    - « My properties » pointe vers `/owner/listings` ;
    - « Add a property » devient un `<Link>` vers `/owner/listings/new`, en desktop comme dans le menu mobile (qui se referme au clic).
14. **Fenêtre de réactivation owner (D3)** :
    - `frontend/src/features/auth/tokenRole.ts` (+ `.test.ts`) : `readTokenRole(token: string): Role | null`. Décode le payload base64url du JWT sans le vérifier et renvoie `role` s'il vaut `seeker`, `owner` ou `admin`, sinon `null`.
    - `frontend/src/features/auth/AuthContext.tsx` : un état `sessionRole` calculé dans `setSession` et remis à `null` dans `clearSession`, exposé dans la valeur du contexte.
    - `frontend/src/features/auth/components/OwnerActivationModal.tsx` : `Modal` intitulée par exemple « Your owner account is ready ». Le texte demande de se reconnecter pour l'activer. L'action principale est « Sign in again » ; la croix ou Échap ferme la fenêtre pour la session.
    - `frontend/src/app/layout/RootLayout.tsx` : la fenêtre s'affiche quand `status === "authenticated"`, `sessionRole === "seeker"` et `profile?.role === "owner"`. « Sign in again » réutilise le même enchaînement que le sign-out (`logout`, `clearSession`, `removeOwnerScopedQueries`), puis navigue vers `/login`. Extraire cet enchaînement dans une fonction qui reçoit la destination.

### 2.4 MH-55-FE — Tests unitaires (vitest, logique pure)

- `parsePageParam` : `null`, `""`, `"abc"`, `"0"`, `"-2"`, `"1.5"`, `"1"`, `"7"`.
- `readTokenRole` : jeton valide pour chacun des trois rôles ; rôle inconnu ; claim absent ; moins de trois segments ; base64 invalide ; JSON invalide.
- `listingVisibility` est déjà couvert (MH-54).
- **Dette** (pas encore de React Testing Library dans le projet) : rendu de la page (squelette, vide, erreur, pagination), lien actif du header, invalidation après création, affichage et action de la fenêtre de réactivation. À vérifier manuellement (section 5).

---

## 3. Ordre d'implémentation recommandé

1. **MH-55-BE** sur sa sous-branche : fichiers 2.1 dans l'ordre (le modèle d'abord, sinon le feed ne compile plus), tests 2.2, artefacts (`.sqlx`, `openapi.json`, TECH_SPEC), vérifications BE (section 5), puis contrat qualité (section 7).
2. **Merge de MH-55-BE dans la branche parent** par l'utilisateur.
3. **MH-55-FE** sur sa sous-branche, créée depuis le parent à jour :
   1. types, `api.ts`, hook ;
   2. `parsePageParam` et son test ;
   3. badges (`labels.ts`, `OwnerBar`) ;
   4. ligne, puis page ;
   5. export, route ;
   6. cache (`useCreateListing`, `removeOwnerScopedQueries`) ;
   7. header ;
   8. fenêtre de réactivation (`tokenRole`, `AuthContext`, modale, `RootLayout`).

   La fenêtre vient en dernier parce qu'elle est indépendante de la page et qu'elle touche le contexte d'auth partagé.
4. **Merge de MH-55-FE dans le parent**, puis une PR unique du parent vers `develop`.

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement MH-55 |
| ---- | -------- | ---------------- |
| `listings/repository.rs` | `list_listings` / `count_listings` (`QueryBuilder`, `push_public_visibility`, `INNER JOIN` sur la couverture) | Le feed gagne `published_at` dans son `SELECT`. Deux nouvelles requêtes statiques, sans prédicat de visibilité. `push_public_visibility` ne change pas. |
| `ListingSummaryRow` / `ListingSummaryDto` | Partagés par le feed | `published_at` ajouté. Le feed FE (`ListingCard`) ne le lit pas, donc aucun impact. |
| `PaginationMeta` / `PaginatedResponse` | `shared/pagination.rs`, 20 par défaut et 50 au plus | Réutilisés tels quels. |
| `AuthUser` + `require_role` | `shared/extractors.rs`, `shared/rbac.rs` ; `is_active` revérifié, rôle lu dans le JWT | Réutilisés. Le rôle périmé est traité côté FE (D3). |
| Router `users` | `/users/me`, `/users/me/avatar` | Inchangé. Le chemin `/users/me/listings` est monté par le router listings. |
| `RequireAuth` / `RequireOwner` | Redirections `/login` et `/` | Réutilisés pour `/owner/listings`. |
| `listingVisibility` | MH-54, exporté pour MH-55 | Appelé avec `hasPhoto = cover_photo_url !== null`. |
| `OwnerBar` | Contient `VISIBILITY_BADGE` | Le badge part dans `labels.ts`. |
| `removeOwnerScopedQueries` | Retire `["listing"]` au sign-out (`RootLayout`, `AdminLayout`) | Retire aussi `["owner-listings"]`. |
| `useCreateListing` | Invalide `["listings"]` | Invalide aussi `["owner-listings"]`. |
| `SiteHeader` | `OWNER_LINKS` statique, « Listings » toujours actif, « Add a property » en `<span>` | Lien actif selon la route ; deux liens actifs. |
| `AuthContext` | Jeton dans un `ref`, jamais décodé | Expose `sessionRole`, utilisé pour l'affichage seulement. |
| `RootLayout.handleSignOut` | `logout`, `clearSession`, purge, `navigate("/")` | Enchaînement extrait, réutilisé par « Sign in again » avec `/login`. |
| `ListingDetail` | « Back to listings » renvoie vers `/` | Inchangé (hors périmètre, R9). |

---

## 5. Vérifications

**BE** : `cargo fmt`, `cargo check`, `cargo clippy`. `cargo test` seulement avec l'accord explicite de l'utilisateur, ciblé sur `listings`. Dans Swagger (`/api/docs`), vérifier `GET /users/me/listings` sous le tag listings (200, 401, 403) et `published_at` sur `ListingSummaryDto`. Contrôle manuel avec `curl` :

- owner avec un brouillon, un bien publié sans photo et un bien publié avec photo : les trois sont renvoyés, du plus récent au plus ancien, avec `published_at` et `cover_photo_url` cohérents ;
- un deuxième owner ne voit aucun de ces biens ;
- `?per_page=1&page=2` : pagination et `total` corrects ; `?per_page=500` est ramené à 50 ;
- jeton seeker et jeton admin : 403 ; sans jeton : 401 ;
- `GET /listings` : `published_at` présent et non nul sur chaque élément.

**FE** : `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Vérification manuelle avec le skill `/run` (ou Playwright headless, comme pour MH-54-FE) :

- owner : « My properties » dans le header (desktop et mobile) ouvre `/owner/listings` et devient actif ; « Listings » ne l'est plus ;
- « Add a property » ouvre le formulaire ; après une création, le brouillon apparaît dans « My properties » sans rechargement ;
- badges : Draft, Published, « Hidden — no photos », Available, Unavailable, chacun avec son libellé ;
- clic sur une ligne : page de détail ; retour arrière : même `?page=` ;
- `?page=abc` affiche la page 1 ; `?page=99` est remplacé par la dernière page ; owner sans bien : état vide avec son CTA ;
- anonyme sur `/owner/listings` : `/login` ; seeker et admin : `/` ;
- sign-out puis retour arrière sur `/owner/listings` : pas de liste en cache ;
- réactivation : un seeker connecté est approuvé par l'admin. Au retour sur l'onglet, la fenêtre s'affiche ; « Sign in again » mène à `/login`. Après la reconnexion, la fenêtre ne revient pas et « My properties » répond 200.

---

## 6. Risques et inconnues

| #   | Risque / inconnue | Traitement |
| --- | ----------------- | ---------- |
| R1  | D3 dépasse les critères d'acceptation de #140 (estimation d'environ +0,5 jour). | Ajouter un critère à #140 avant l'implémentation FE. Formulation proposée : *"An owner approved during their session is asked, in a dialog, to sign in again to activate owner access; the action signs them out and opens /login."* |
| R2  | La fenêtre n'apparaît qu'une fois `GET /users/me` relu : par défaut au retour sur l'onglet ou au montage (QueryClient sans `staleTime`). | Accepté. Un rechargement de page corrige aussi la situation tout seul, puisque le refresh initial émet un jeton avec le rôle en DB. |
| R3  | Fenêtre fermée par l'utilisateur : les routes owner répondent 403 et la page affiche l'erreur générique jusqu'au prochain refresh du jeton. | Accepté (la fermeture vaut pour la session). La correction transverse côté BE (rôle revalidé à chaque requête) reste hors ticket. |
| R4  | Décoder le JWT côté client peut être pris pour un contrôle d'accès. | Usage limité à l'affichage, documenté en une ligne dans `tokenRole.ts`. Le BE reste seul juge. |
| R5  | Le MCP PostgreSQL n'a pas pu se connecter pendant l'analyse. Les règles base de données exigent de le consulter avant une requête sur `listings`. | Réessayer au début de MH-55-BE. Sinon, s'appuyer sur la migration de base (`idx_listings_owner` confirmé) et sur `cargo sqlx prepare`. |
| R6  | Nullabilité inférée par `query_as!` sur le `LEFT JOIN` et sur `to_char(NULL)`. | Vérifier via Context7 (sqlx 0.8) ; surcharges `?` et `!` explicites. |
| R7  | Pas de critère de départage dans `ORDER BY created_at DESC`, comme dans le feed. | Accepté : des égalités à la microseconde sont improbables pour un même owner. |
| R8  | La page stockée dans l'URL peut dépasser `total_pages`, par exemple après les suppressions de MH-58. | Remplacement par la dernière page (2.3, point 8). Squelette ou liste précédente pendant la requête, grâce à `keepPreviousData`. |
| R9  | « Back to listings » sur le détail renvoie vers `/`, même quand on vient de « My properties ». | Hors périmètre. À signaler pour MH-56 ou MH-61. |
| R10 | Le wireframe MH-14 prévoit `features/owner-listings`, alors que le code utilise `features/listings`. | Garder `features/listings`, en continuité avec MH-53 et MH-54. |
| R11 | Le module `owner_requests` affiche « You now have owner access. » sur la page de statut, alors que l'accès n'est effectif qu'après reconnexion. | Hors périmètre ; à signaler. La fenêtre D3 couvre le parcours principal. |

---

## 7. Contrat qualité (à appliquer sur chaque branche, BE puis FE)

1. **Orientation** : `graphify query`, `graphify path` ou `graphify explain` avant tout grep ou lecture de fichier. Relire ce plan et le ticket GitHub (`gh issue view 139` ou `140`).
2. **Context7**, avant d'écrire le code qui utilise une librairie, pour vérifier les bonnes pratiques, la sécurité et l'idiome de la version installée :
   - BE : nullabilité de `query_as!` et surcharges `"col?"` et `"col!"` dans sqlx 0.8 (`LEFT JOIN`, `COUNT(*)`, `to_char`) ; `IntoParams` et `routes!` dans utoipa 5 / utoipa-axum ; extracteur `Query` d'axum.
   - FE : `useSearchParams` (mode `replace`) dans react-router v7 ; `keepPreviousData`, `invalidateQueries` et `removeQueries` dans TanStack Query v5 ; décodage base64url (`atob` et padding) sans dépendance.
3. **Code minimal, KISS** : pas d'abstraction qui ne serve pas ce ticket. Deux requêtes, un service, un handler côté BE. Un hook, une ligne, une page, deux helpers purs et une fenêtre côté FE.
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
- **MCP disponibles** : Context7 (`mcp__context7__*`), Git MCP (lecture seule).
- **MCP en échec** pendant l'analyse : PostgreSQL (voir R5), GitHub (plugin ; remplacé par la CLI `gh`), Sequential Thinking (pas nécessaire ici).

---

## 8. Hors périmètre

Les actions d'édition (MH-56), de statut (MH-57) et de suppression (MH-58) dans la zone d'actions ; le format « XAF / month » (MH-62) ; la publication (MH-61) ; l'écran photos (MH-59) ; des filtres ou un tri sur « My properties » ; la revalidation du rôle à chaque requête côté BE ; la destination de « Back to listings » sur le détail ; le texte de la page de statut de la demande owner ; l'index sur le prix (R-07).

---

## 9. Critères de fin

**MH-55-BE**

- [ ] `GET /users/me/listings` : 200, enveloppe paginée, `page` et `per_page` avec les valeurs par défaut et les bornes du feed
- [ ] Tous les biens de l'appelant (brouillons et biens sans photo compris), `created_at DESC`, jamais ceux d'un autre owner
- [ ] Même forme qu'un élément du feed, plus `published_at` ; `published_at` aussi présent dans le feed
- [ ] 403 pour seeker et admin ; 401 sans jeton
- [ ] OpenAPI : tag listings, réponses 200, 401 et 403 ; `openapi.json` régénéré ; `.sqlx` à jour ; TECH_SPEC §4.3 à jour
- [ ] Tests de 2.2 ; fmt, check et clippy propres

**MH-55-FE**

- [ ] Types régénérés (`published_at` et `/users/me/listings`)
- [ ] `/owner/listings` : anonyme vers `/login`, seeker et admin vers `/`
- [ ] Lignes : vignette ou placeholder, titre, « ville · quartier », prix, badge de visibilité et badge de disponibilité avec libellé texte, zone d'actions vide à droite
- [ ] Clic sur une ligne : détail ; « + Publish a listing » : `/owner/listings/new`
- [ ] Squelette, état vide avec CTA, alerte d'erreur ; pagination si plus d'une page ; `?page=` dans l'URL
- [ ] Header owner (desktop et mobile) : « My properties » actif sur sa page, « Add a property » vers le formulaire, plus de texte inerte
- [ ] Création : cache `["owner-listings"]` invalidé ; sign-out : cache retiré
- [ ] Fenêtre de réactivation owner (D3) et critère ajouté à #140
- [ ] Tests de 2.4 ; lint, ts:check, test et prettier propres
