# MH-75 : retrait du rôle owner, plan d'implémentation (BE + FE)

- **Epic** : EP-12
- **Tickets** : MH-75 (#218), MH-75-BE (#219), MH-75-FE (#220, dépend de MH-75-BE et MH-73-FE)
- **Branches** :
  - BE : `mh-75-be-owner-role-revocation`, créée en local depuis `develop` (`700ff5e`, MH-74 inclus). Pas encore sur origin.
  - FE : pas encore créée. À récupérer ou à demander avant de coder.
  - Ni commit ni push automatique.
- **ARCHITECTURE §8.1** : la transition `owner --> seeker : Admin révoque` figure déjà dans le diagramme d'états. Le CDC (ADMIN-01 à ADMIN-05) ne la détaille pas.
- **Documentation** (spec §4.3, §4.7, ARCHITECTURE §8.1, §11) : dette portée par MH-76. Rien n'est modifié ici.

## 1. Contexte et objectif
L'admin peut suspendre un compte (MH-74), mais rien ne lui permet de retirer à un owner le droit de publier sans fermer le compte. Le ticket ajoute :
- **BE** : `POST /admin/users/:id/revoke-owner`, et le rôle owner dans la règle de visibilité publique. Les biens restent en base, intacts, et redeviennent publics si la personne est de nouveau approuvée.
- **FE** : l'action « Revoke owner role » dans `/admin/users`, et un profil qui dit à la personne rétrogradée que son accès a été retiré et qu'elle peut refaire une demande.

### Décisions
| #   | Sujet | Décision |
| --- | ----- | -------- |
| D1  | Emplacement | Handler, service et repository dans `modules/users/`, comme MH-73 et MH-74. Nouveau chemin, donc un nouvel appel `routes!(handler::revoke_owner)` dans `users/router.rs`. Le handler commence par `require_role(&[Role::Admin])`. Pas de corps de requête. |
| D2  | Garde | Fonction pure `ensure_revocable(role)` : appelle `ensure_not_admin` (403 `ADMIN_ACCOUNT_PROTECTED`, troisième appelant comme prévu en MH-74), puis `Seeker` donne 409 `NOT_AN_OWNER`, `Owner` passe. `is_active` n'est pas lu : un owner suspendu se révoque comme un autre. |
| D3  | Service | `revoke_owner_for_admin(pool, admin_id, user_id)` : `find_for_admin` (absent : `UserNotFound`), `ensure_revocable`, écriture, log. La réponse réutilise la ligne lue avec `AdminUserRow { role: Role::Seeker, ..row }`, sans seconde requête. |
| D4  | Écriture | `UPDATE users SET role = 'seeker' WHERE id = $1 AND role = 'owner'`, une seule requête, sans transaction. Zéro ligne (révocation concurrente, ou suppression entre la lecture et l'écriture) donne `NotAnOwner`. Aucun refresh token supprimé, aucun cache invalidé : le cache ne porte que `is_active`, et `/auth/refresh` relit le rôle en base. |
| D5  | Erreur | Nouvelle variante `AppError::NotAnOwner` → 409 `NOT_AN_OWNER`, dans la section 409 de `errors.rs`, avec son test de mapping à côté des existants. |
| D6  | Règle publique | Choix validé : `owner_is_eligible_sql!()` devient `EXISTS (SELECT 1 FROM users owner WHERE owner.id = l.owner_id AND owner.is_active AND owner.role = 'owner')`. Seul point modifié : `IS_PUBLIC` et `OWNER_IS_ELIGIBLE` en héritent, donc le feed et son total, le filtre `owner_id`, la recherche, le contact, l'état `hidden` de l'admin et le détail. Mettre à jour la doc des deux constantes. |
| D7  | Détail | Rien à coder. `find_listing_by_id` sélectionne déjà `OWNER_IS_ELIGIBLE` ; avec D6 il vaut `false` pour la personne rétrogradée, et `is_visible_to` répond 404 `LISTING_NOT_FOUND` à elle aussi. Matrice de tests de `is_visible_to` inchangée. |
| D8  | Refresh | Rien à coder. `find_by_hash` joint `users` et lit `u.role` (`auth/repository.rs:25`), et `mint` reçoit ce rôle (`auth/service.rs:86`, `:134`). Le prochain token porte donc `seeker`. |
| D9  | Nouvelle demande | Rien à coder. Un seeker peut soumettre (`owner_requests/handler.rs:45`). Une demande `pending` au moment du retrait reste telle quelle et bloque une nouvelle soumission (`OWNER_REQUEST_ALREADY_PENDING`). L'approbation fait `SET role = 'owner' WHERE role = 'seeker'` (`owner_requests/repository.rs:208`). Le retrait ne touche aucune ligne `listings` ni `listing_media` : statut, `published_at` et photos reviennent tels quels. |
| D10 | Numéro secondaire | Rien à coder. `find_owner_contact` prend déjà la dernière demande approuvée par `reviewed_at DESC NULLS LAST, created_at DESC` (`contact/repository.rs:23-27`). À vérifier au scénario curl après une seconde approbation avec un autre numéro. |
| D11 | Log | `tracing::info!(%admin_id, %user_id, "owner role revoked by admin")`, une ligne par révocation réussie. Jamais d'email. |
| D12 | OpenAPI | `#[utoipa::path(post, path = "/admin/users/{id}/revoke-owner", tag = "admin")]` : 200 `AdminUserDto`, 401, 403 (`FORBIDDEN` ou `ADMIN_ACCOUNT_PROTECTED`), 404, 409 `NOT_AN_OWNER`. Export vers `frontend/docs-frontend/openapi.json`. |
| D13 | Client FE | `revokeOwnerRole(id): Promise<AdminUser>` dans `features/admin/api.ts`, à côté de `updateAdminUserState`. Types régénérés par `generate:types:ci`. Helper pur pour reconnaître `NOT_AN_OWNER` à côté de `isNotFound`, sauf si un test de code d'erreur générique existe déjà dans `client.ts`. |
| D14 | Hook | `useRevokeOwnerRole(id)` sur le modèle de `useUpdateAdminUserState`, sans mise à jour optimiste, avec `adminUserMutationKey(id)` : la ligne reste verrouillée pour les trois actions. `onSuccess` retourne l'invalidation de `[ADMIN_QUERY_ROOT]` (le badge de rôle et l'état `hidden` des biens). 404 : chemin `USER_GONE_MESSAGE` existant. Choix validé pour 409 : erreur dans la modale, puis même invalidation, pour que le badge montre `seeker`. |
| D15 | Action admin | `RevokeOwnerAction` sur le modèle de la modale « Suspend » de `AccountStateAction`, rendue dans la cellule d'action de `AdminUserRow` seulement si `user.role === "owner"` (suspendu ou non). La modale dit que les biens sont masqués mais conservés et que la personne peut refaire une demande. Erreur en `Alert` dans la modale ; le badge garde l'ancien rôle tant que rien n'a réussi. |
| D16 | Vue du statut | Choix validé : `ownerRequestStatusView(data, role)` gagne l'état `withdrawn` quand la dernière demande est `approved` et que `role === "seeker"`. Titre « Your owner access was withdrawn by an administrator », CTA « Submit a new request » vers `/owner-request`. Le type `role` vient de `shared/api/types.ts`, pas de `features/profile`. |
| D17 | Appelants | `OwnerRequestStatusBlock` (profil, `role` déjà en prop ; le libellé du lien couvre `withdrawn` comme `rejected`), `OwnerRequestStatus` (page de statut, lit `useProfile`), `RootLayout` (CTA du header, `"seeker"` puisque `isSeeker`). |
| D18 | Navigation | Rien à coder. `RootLayout` et `RequireOwner` lisent `profile.role` (`GET /users/me`, donc la base), pas le rôle du token. Au prochain refetch du profil (pas de `staleTime`, refetch au montage et au focus), la navigation passe en seeker et les pages owner redirigent vers `/`. `OwnerActivationModal` reste valable pour une nouvelle approbation. |

## 2. Découpage BE / FE et dépendances
- **MH-75-BE** n'a pas de dépendance ouverte : MH-74-BE est mergé (`700ff5e`). Il livre la route, `NOT_AN_OWNER` et le contrat OpenAPI.
- **MH-75-FE** dépend du BE pour `types.ts` et la route. D16 et D17 n'utilisent aucune API nouvelle et peuvent démarrer avant ; l'action admin (D13 à D15) attend le BE.
- **MH-76** porte la documentation : spec §4.7 (route, `NOT_AN_OWNER`), §4.3 (règle de visibilité), ARCHITECTURE §8.1 (transition), §11 (risques R1 et R3 ci-dessous).

## 3. Ordre d'implémentation recommandé
### MH-75-BE
1. `listings/repository.rs` : D6, puis contrôle du plan de requête (R5).
2. `shared/errors.rs` : D5.
3. `users/repository.rs`, `users/service.rs`, `users/handler.rs`, `users/router.rs` : D1 à D4, D11, D12.
4. `cargo sqlx prepare` (nouvelle requête `query!` dans `users/repository.rs`), export OpenAPI.
5. Contrat qualité et scénario curl (§6), dont les vérifications de D7 à D10.

### MH-75-FE
1. `features/owner-request/ownerRequestStatusView.ts` et son test : D16.
2. `OwnerRequestStatusBlock.tsx`, `OwnerRequestStatus.tsx`, `app/layout/RootLayout.tsx` : D17.
3. Régénération de `types.ts`, puis `features/admin/api.ts`, `hooks/useRevokeOwnerRole.ts`, `components/RevokeOwnerAction.tsx`, `AdminUserRow.tsx` : D13 à D15.
4. Contrat qualité et parcours navigateur (§6).

## 4. Points d'intégration avec l'existant
| Zone | Fichier (sur `develop`) | Effet |
| ---- | ----------------------- | ----- |
| Règle publique | `listings/repository.rs:22-26`, `:32`, `:36-42` | Un seul fragment modifié pour le feed, la recherche, le contact, l'admin et le détail |
| Détail | `listings/repository.rs:369`, `listings/service.rs:116-122`, `:138` | Inchangé, hérite de D6 |
| Contact | `contact/repository.rs:19-32` | Inchangé, hérite de D6 ; numéro secondaire déjà pris sur la dernière approbation |
| Garde admin | `users/service.rs:218-223` | `ensure_not_admin` réutilisé par `ensure_revocable` |
| Admin users | `users/service.rs:156-176`, `users/handler.rs:181-212`, `users/router.rs:18` | Nouvelle route à côté de `PATCH` et `DELETE` |
| Erreurs | `shared/errors.rs:105-126`, `:191-211` | `NOT_AN_OWNER` ajouté |
| Refresh | `auth/repository.rs:13-25`, `auth/service.rs:86`, `:134` | Inchangé, relit le rôle |
| Ré-approbation | `owner_requests/repository.rs:206-208`, `owner_requests/service.rs:52` | Inchangé |
| Extracteur | `shared/extractors.rs:186-203` | Inchangé : le rôle vient du token (R1) |
| Vue du statut | `ownerRequestStatusView.ts`, `OwnerRequestStatusBlock.tsx:36`, `:50`, `OwnerRequestStatus.tsx:32`, `RootLayout.tsx:31-34` | Nouvel état `withdrawn` |
| Garde owner | `app/RequireOwner.tsx:23`, `RootLayout.tsx:23-28` | Inchangé, lit le profil |
| Table admin | `AdminUserRow.tsx:51-58`, `api.ts:63`, `:120`, `hooks/useUpdateAdminUserState.ts` | Action ajoutée, clé de mutation partagée |

## 5. Risques et inconnues
| #   | Risque | Traitement |
| --- | ------ | ---------- |
| R1  | `AuthUser` ne relit pas le rôle : jusqu'à l'expiration du token d'accès (15 min au plus), la personne garde les droits owner sur l'API. | Accepté par le ticket : tout ce qu'elle crée ou publie reste masqué par D6. À consigner dans MH-76. |
| R2  | Révocation et approbation d'une demande `pending` concurrentes : le résultat dépend de l'ordre. | Les deux sont des actions de l'admin unique. Accepté. |
| R3  | Avec `role = 'owner'`, les biens d'un owner promu admin par `bootstrap_admin` (`users/repository.rs:302-309`) deviennent masqués. | Choix validé. Cas limité au démarrage sans admin existant. À consigner dans MH-76. |
| R4  | Les photos restent accessibles par URL directe : nginx les sert sans passer par le backend. | Même risque que MH-74 R4, conséquence de la décision verrouillée. |
| R5  | Le fragment de visibilité filtre maintenant aussi sur le rôle. | `EXPLAIN ANALYZE` du comptage et de la liste, avec et sans mots-clés : attendu, le même accès par clé primaire `users`, `idx_listings_search` conservé. Aucun index ajouté. Le MCP PostgreSQL ne s'est pas connecté pendant cette session : passer par `psql` si c'est encore le cas. |
| R6  | Après succès, le bouton « Revoke owner role » disparaît de la ligne et le focus retombe sur `body`. | À trancher à l'implémentation : focus sur l'action voisine de la ligne, ou accepté comme en MH-58-FE. |
| R7  | Le compteur de biens de `AdminUserRow` n'est un lien que pour un owner : les biens masqués d'une personne rétrogradée ne sont plus atteignables depuis `/admin/users`. | Hors AC, rien changé. Restent visibles dans `/admin/listings` avec le filtre `hidden`. |
| R8  | La page owner ouverte au moment du retrait reste affichée jusqu'au prochain refetch du profil. | Accepté : les écritures restent masquées (R1), et la redirection suit au focus ou à la navigation. |
| R9  | Les issues #218 à #220 n'ont aucun label et aucune branche liée. | Sans impact sur le code. Branche BE créée en local, FE à créer. |

## 6. Contrat qualité (à appliquer sur chaque branche)
Skills et outils vérifiés dans cette session :
- Skills du repo (`.claude/skills/`) : `code-review-backend`, `code-review-frontend`, `docker`, `github-ticket`, `graphify`, `readme`.
- Plugin : `humanizer:humanizer`.
- Context7 : le connecteur claude.ai répond (résolution de `sqlx`). Le MCP local `context7` ne s'est pas connecté, comme `postgres` et `sequential-thinking`.

1. **Avant de coder** : `git fetch`, puis se placer sur la sous-branche du ticket. Ni commit ni push automatique.
2. **Context7** : vérifier bonnes pratiques, sécurité et idiomatisme sur les API concernées. BE : `rows_affected` sur un `UPDATE` conditionnel sqlx, comparaison d'un enum PostgreSQL à un littéral, `utoipa_axum::routes!`. FE : `useMutation` et `invalidateQueries` de TanStack Query, `Navigate` de React Router.
3. **Code minimal, KISS** : rien au-delà des AC, pas d'abstraction pour un seul appelant. D7 à D10 et D18 ne demandent aucun code, seulement une vérification.
4. **Tests unitaires sur la logique pure uniquement**, sans dépendance à la base :
   - BE : `ensure_revocable` (trois rôles), mapping de `AppError::NotAnOwner` en 409.
   - FE : `ownerRequestStatusView` (approuvée et seeker donne `withdrawn` avec CTA ; approuvée et owner reste `approved` ; `pending` et `rejected` inchangés), helper `NOT_AN_OWNER`.
   - Les cas qui exigent une vraie base vont à l'epic d'intégration et sont notés comme dette de tests.
5. **Première review avant tout fix** : review complète d'abord, puis fixes appliqués avant de continuer.
6. **Review dédiée par domaine** : `code-review-backend` sur la branche BE, `code-review-frontend` sur la branche FE.
7. **Passe `humanizer:humanizer`** sur les commentaires : ton humain, neutre, concis, fidèle au code. Pas de référence à un fichier de règles.
8. **Vérifications de fin** :
   - BE : `cargo check`, `cargo fmt`, `cargo clippy`, sans `cargo test` complet. Scénario curl : 200 sur un owner actif et sur un owner suspendu, 403 admin, 409 seeker et seconde révocation, 404, 401, 403 non-admin ; feed et total, `owner_id`, recherche, détail en anonyme et par la personne rétrogradée, contact, `hidden` dans `/admin/listings` ; refresh qui émet `seeker` ; demande `pending` conservée ; nouvelle demande, approbation, retour des biens avec statut, `published_at` et photos inchangés, numéro secondaire de la dernière approbation ; ligne de log.
   - FE : lint, typecheck, tests purs. Parcours navigateur : révocation depuis `/admin/users` et badge, échec 409 avec rafraîchissement, profil et page de statut de la personne rétrogradée, CTA du header, redirection depuis une page owner, nouvelle demande.
   - `graphify update .` après les changements de code.
