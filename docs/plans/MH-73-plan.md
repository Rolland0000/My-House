# MH-73 : liste et suppression des comptes depuis le back-office, plan d'implémentation (BE + FE)

- **Epic** : EP-12 (#207)
- **Tickets** : MH-73 (#212), MH-73-BE (#213), MH-73-FE (#214, dépend de MH-73-BE)
- **Branches** :
  - BE : `mh-73-be-admin-user-list-and-account-deletion` (origin, à jour avec `develop`).
  - FE : pas encore créée. À récupérer ou à demander avant de coder.
  - Ni commit ni push automatique.
- **CDC** :
  - ADMIN-02 (Must) : l'admin peut lister, suspendre ou réactiver et supprimer tout compte.
  - ADMIN-05 : un seul admin, créé au bootstrap, jamais par une route applicative.
- **Documentation** (spec §4.7 et §1.1, ARCHITECTURE §5.2) : dette portée par MH-76. Rien n'est modifié ici.

## 1. Contexte et objectif
L'admin n'a aujourd'hui aucun moyen de retrouver ou de supprimer un compte sans passer par la base. Le ticket ajoute :
- **BE** : `GET /admin/users` (liste paginée, filtrable et cherchable) et `DELETE /admin/users/{id}`. Les deux routes vivent dans le module `users`, qui possède déjà la suppression de compte et le nettoyage du stockage. `admin` n'ajoute donc aucun import croisé (R-11).
- **FE** : une page `/admin/users` sous forme de tableau, avec filtres et recherche dans l'URL et une suppression confirmée. MH-74 et MH-75 y ajouteront leurs actions par ligne.

### Décisions
| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Emplacement | Handlers, service, repository et DTO dans `modules/users/`. `#[utoipa::path(tag = "admin")]`. Deux appels `.routes(routes!(…))` dans `users/router.rs`, un par chemin. |
| D2 | Garde | `user.require_role(&[Role::Admin])?` en première ligne de chaque handler, comme dans `admin/handler.rs`. 401 sans jeton, 403 `FORBIDDEN` pour un seeker ou un owner. |
| D3 | Query | `AppQuery<AdminUserListQuery>` plutôt que `Query` : une valeur invalide donne 400 `INVALID_QUERY_PARAM`. `role: Option<Role>` et `state: Option<AccountState>` sont typés (changé à la review) : serde rejette les valeurs inconnues, sans parseur à maintenir, et l'OpenAPI publie les enums pour le FE. `state=active` donne `is_active = true` et `state=suspended` donne `false`. |
| D4 | `q` | `shared::validation::keyword_filter` est réutilisé tel quel : trim, blanc ignoré, NUL rejeté, plus de 100 caractères donne 400. SQL : `strpos(lower(immutable_unaccent(col)), lower(immutable_unaccent($3))) > 0` sur `email`, `first_name` et `last_name`, reliés par OR. Pas de `LIKE` (changé à la review) : `unaccent` convertit `％`, `＿` et `＼` pleine chasse en caractères spéciaux de `LIKE` après l'échappement, ce qui cassait l'AC « match literally ». Un nom NULL donne NULL, donc pas de correspondance, sans `COALESCE`. |
| D5 | Requête liste | `query_as!` statique, sur le modèle de `owner_requests/repository.rs`. Filtres optionnels en `($1::user_role IS NULL OR u.role = $1)`, `($2::bool IS NULL OR u.is_active = $2)` et `($3::text IS NULL OR …)`. `listing_count` vient de `(SELECT COUNT(*) FROM listings l WHERE l.owner_id = u.id)`, qui utilise `idx_listings_owner` et compte les brouillons. Tri `created_at DESC, id DESC`. `created_at` est formaté avec le `to_char` existant. `count_users_for_admin` reprend les mêmes filtres. Flux `PaginationMeta::new` puis `PaginatedResponse::new`, comme dans `owner_requests/service.rs:118-133`. Mettre à jour `.sqlx/` avec `cargo sqlx prepare`. |
| D6 | DTO | `AdminUserDto { id, email, first_name, last_name: Option, phone: Option, role, is_active, created_at, listing_count: i64 }`, en `ToSchema`. Aucune donnée d'identité des demandes owner. L'enveloppe paginée passe par le même mécanisme que la liste owner-requests. |
| D7 | Suppression | Le corps de `users::service::delete_account` (clés avatar, photos et documents, `delete_storage_objects` puis `delete_by_id`) est extrait dans une fonction interne qui prend la `UserRow` déjà chargée. `delete_account` (`/users/me`) garde son comportement. `admin_delete_user(…, target_id)` suit cet ordre : `find_by_id` (absent donne `UserNotFound`), puis `ensure_deletable(role)` (pure : `Admin` donne `AdminAccountProtected`), puis la fonction commune, fichiers avant lignes. Aucune course sur le rôle admin n'est possible puisqu'aucune route ne promeut un admin (ADMIN-05). |
| D8 | Session | Choix validé : `is_active_status().invalidate(&target_id)` après le DELETE, dans le service qui reçoit le cache en paramètre. La coupure est immédiate au lieu d'attendre jusqu'à 8 s. MH-74 réutilisera l'appel. |
| D9 | Réponse et erreurs | `204 No Content`, sans cookie touché. Nouveau variant `AppError::AdminAccountProtected` (403 `ADMIN_ACCOUNT_PROTECTED`) placé dans le bloc 403 de `errors.rs`. Un id inconnu donne 404 `USER_NOT_FOUND`. Un id malformé est rejeté par `Path<Uuid>`, comme ailleurs. |
| D10 | Log | `tracing::info!(admin_id = %…, deleted_user_id = %…, "user account deleted by admin")`. Jamais d'email. |
| D11 | Layout FE | Choix validé : `<table>` natif sémantique (`<th scope="col">`), dans un conteneur `overflow-x-auto`. Colonnes : nom, email, téléphone, rôle, état, biens, inscription, action. Une valeur nulle s'affiche « — ». |
| D12 | État URL | `adminUsersCriteria.ts` (pur), sur le modèle de `search/searchCriteria.ts` : `parse(URLSearchParams)` remplace une valeur invalide par la valeur par défaut, `toUrlSearchParams` omet les valeurs vides et la page 1. Un changement de filtre utilise `replace` et remet la page à 1. La recherche passe par `useDebouncedValue(draft, draft ? TEXT_FILTER_DEBOUNCE_MS : 0)`, avec une soumission immédiate sur Enter (`<form role="search">`). `placeholderData: keepPreviousData` garde les résultats précédents, avec `aria-busy` et `opacity-60`. Une page hors bornes affiche « Go to page 1 ». |
| D13 | Lien biens | Choix validé : pour un owner, le compteur est un `Link` vers `/admin/listings?owner_id=<id>`. Il reste mort jusqu'au merge de MH-72-FE. Pour les autres rôles, le compteur est du texte. |
| D14 | Suppression FE | `DeleteUserAction` sur le modèle de `DeleteListingAction` : `Modal`, bouton `danger` avec `isLoading`, fermeture bloquée pendant l'envoi. Verrou par ligne avec `useIsMutating({ mutationKey: adminUserMutationKey(id) })`. Le succès est géré dans le hook (la ligne se démonte) : `setQueriesData` avec `withoutRow` (partagé), invalidation du préfixe `["admin"]` (les demandes owner du compte partent aussi) et toast de succès. Un 404 n'est pas un succès : la ligne sort du cache, toast « This account no longer exists » puis invalidation. Toute autre erreur s'affiche en `Alert` dans la modale. Une ligne admin n'a aucune action. |
| D15 | Nav admin | `AdminHeader` utilise `NavLink`, qui gère l'état actif et `aria-current`. « Owner requests » est actif sur `/admin/owner-requests*` et « Users » sur `/admin/users`. « Properties » reste un placeholder (MH-72-FE). |

## 2. Découpage BE / FE et dépendances
- **MH-73-BE** n'a pas de dépendance. Il livre le contrat OpenAPI (`frontend/docs-frontend/openapi.json`).
- **MH-73-FE** dépend du BE pour `types.ts`, régénéré par `generate:types:ci`. La logique pure (D12, badges, helpers de suppression) peut démarrer avant, mais l'intégration attend le BE.
- **Contrat** :
  - `GET /api/v1/admin/users?role=&state=&q=&page=&per_page=` renvoie `200 { data: AdminUserDto[], pagination }`, ou 400, 401, 403.
  - `DELETE /api/v1/admin/users/{id}` renvoie 204, ou 401, 403 (`FORBIDDEN` ou `ADMIN_ACCOUNT_PROTECTED`), 404 `USER_NOT_FOUND`.
- **MH-72 (en parallèle)** : MH-72-FE fournira `/admin/listings?owner_id=`. Fichiers communs à fusionner avec soin : `errors.rs`, `api_doc.rs`, `SiteHeader.tsx` (AdminHeader) et `router.tsx`.

## 3. Ordre d'implémentation
### MH-73-BE
1. `shared/errors.rs` : variant `AdminAccountProtected`, son bras dans `status_and_code` et son test de mapping.
2. `users/dto.rs` : `AdminUserListQuery` (`IntoParams`), `AdminUserDto` et l'enveloppe paginée.
3. `users/repository.rs` : `count_users_for_admin` et `list_users_for_admin` (D5).
4. `users/service.rs` :
   - `ensure_deletable` (pure, avec test) ;
   - `list_users_for_admin` ;
   - extraction de la fonction commune de suppression ;
   - `admin_delete_user` avec l'invalidation du cache (D7, D8).
5. `users/handler.rs` : `list_users` et `delete_user` (D2, D9, D10). `users/router.rs` : les deux routes.
6. `cargo sqlx prepare`, puis `cargo run --bin gen_openapi -- ../frontend/docs-frontend/openapi.json` (le job CI échoue en cas de dérive).
7. Retirer le `// TODO EP-12` de `admin/router.rs` seulement s'il ne concerne que la suppression. La partie `deactivate_user` appartient à MH-74 et reste.

### MH-73-FE
1. `generate:types:ci`.
2. `features/admin/api.ts` : `listAdminUsers(params)`, `deleteAdminUser(id)` (avec `encodeURIComponent`) et la clé `adminUsersQueryKey`.
3. Fichiers purs avec leurs `.test.ts` :
   - `adminUsersCriteria.ts` ;
   - `adminUserBadges.ts` (rôle et état, source unique des valeurs et des libellés) ;
   - dans `shared/` : `paginatedRows.ts` (`withoutRow`), `isNotFound` dans `client.ts`, `pageParam.ts` (déplacé, avec la limite u32), `useDebouncedUrlDraft` et `useUrlDraft`.
4. Hooks : `useAdminUsers.ts` (avec `keepPreviousData`) et `useDeleteAdminUser.ts` (D14).
5. Composants : `AdminUsersPage.tsx` (filtres, tableau, pagination, état vide, erreur avec Retry), `AdminUsersSearch.tsx`, `AdminUserRow.tsx` et `DeleteUserAction.tsx`.
6. `app/router.tsx` : route lazy `users` sous `/admin`. `SiteHeader.tsx` : D15. `removeAdminQueries` à la déconnexion dans `AdminLayout` et `RootLayout`.

## 4. Points d'intégration avec l'existant
| Zone | Existant | Changement |
| ---- | -------- | ---------- |
| `users/service.rs::delete_account` | Clés avatar, médias et documents, fichiers puis `DELETE` | Corps extrait et partagé avec l'admin |
| `infra/cache` | `is_active_status()` (TTL 8 s, jamais invalidé) | `invalidate` après la suppression admin |
| `shared/extractors.rs` | `AuthUser`, `AppQuery` | Utilisés tels quels |
| `shared/validation.rs` | `keyword_filter` | Réutilisé |
| `shared/pagination.rs` | `PaginatedResponse`, `PaginationMeta` | Utilisés tels quels |
| Migrations | `immutable_unaccent`, `idx_listings_owner` | Aucune migration |
| `SiteHeader.tsx` AdminHeader | Actif codé en dur, « Users » en placeholder | Lien et état actif par chemin |
| `shared/components` | `Modal`, `Toast`, `Badge`, `EmptyState`, `Alert`, `Pagination`, `Select`, `Input` | Réutilisés, aucun nouveau composant partagé |
| `shared/hooks/useDebouncedValue` | Délai de 400 ms | Réutilisé |

## 5. Risques et inconnues
- **R1** : la spec §4.7 ne connaît ni `q`, ni `state`, ni `listing_count`, ni le 204, ni `ADMIN_ACCOUNT_PROTECTED`. À ajouter à la dette MH-76.
- **R2** : `DELETE /users/me` renvoie 200 (cookie vidé) et l'admin renvoie 204. L'écart est assumé : pas de cookie côté admin.
- **R3** : fenêtre d'orphelins existante. Un upload commité entre l'énumération des clés et le `DELETE` laisse un fichier orphelin. C'est déjà accepté pour `/users/me`, rien n'est ajouté.
- **R4** : le lien `/admin/listings?owner_id=` est mort jusqu'à MH-72-FE (D13).
- **R5** : MH-72 avance en parallèle sur les mêmes fichiers (voir §2). Risque de conflits de merge.
- **R6** : aucun index sur `users` pour `q`, le tri ou les filtres. Un scan séquentiel est accepté au MVP. `EXPLAIN` via `psql`, puisque le MCP Postgres n'est pas connecté.
- **R7** : la nav admin est `hidden sm:flex`, donc il n'y a pas d'accès mobile à la page. Hors périmètre.
- **R8** : l'admin et les comptes anciens peuvent avoir `last_name` ou `phone` à NULL. Le DTO les rend optionnels et le FE affiche « — ».
- **R9** : pas de React Testing Library, donc aucun test de composant. La dette sera notée en mémoire.

## 6. Vérifications
**BE**
- `cargo fmt`, `cargo check` et `cargo clippy`.
- Tests unitaires : `AccountState`, `ensure_deletable`, le mapping de `ADMIN_ACCOUNT_PROTECTED` et la sérialisation de `AdminUserDto` (sans champ d'identité).
- Curl scripté sur la stack locale :
  - 401 et 403 ;
  - filtres seuls et combinés ;
  - `q` avec accents, casse, `%`, `_`, blanc, 101 caractères ;
  - valeurs inconnues de `role` et `state` ;
  - tri ;
  - `listing_count` avec brouillons ;
  - suppression d'un owner : fichiers et lignes supprimés, session coupée immédiatement ;
  - 403 sur l'admin sans aucun changement ;
  - 404 ;
  - log sans email.
- `openapi.json` sans dérive.

**FE**
- `npm run lint`, `ts:check`, `test` et `prettier:check`.
- Tests vitest des trois fichiers purs.
- Playwright sur l'app réelle (OTP admin via Mailhog) :
  - nav active ;
  - filtres et recherche (debounce, Enter, rechargement, retour arrière) ;
  - suppression avec toast, 404, verrou de ligne ;
  - état vide et Retry.

## 7. Contrat qualité (sur chaque branche)
- **Context7** pour vérifier les bonnes pratiques, la sécurité et l'idiomatisme :
  - BE : `query_as!`, `utoipa::path` et `IntoParams`, invalidation moka ;
  - FE : `useMutation`/`useIsMutating` et `keepPreviousData` de TanStack Query v5, `useSearchParams` de React Router.
- **KISS** : code minimal, aucune librairie ajoutée.
- **Tests** : unitaires sur la logique pure uniquement, sans dépendance à la DB.
- **Première review** avant tout fix. Chaque finding est reproduit, puis le fix est appliqué avant de continuer.
- **Review dédiée par domaine** avec les skills du repo vérifiés : `code-review-backend` et `code-review-frontend`. Puis une passe `humanizer:humanizer` sur les commentaires et les textes visibles : ton humain, neutre, concis et fidèle au code.
- **Après le code** : `graphify update .` et une note de dette de tests en mémoire.
- Ni commit ni push.
