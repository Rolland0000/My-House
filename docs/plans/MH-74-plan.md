# MH-74 : suspension et réactivation d'un compte, plan d'implémentation (BE + FE)

- **Epic** : EP-12 (#207)
- **Tickets** : MH-74 (#215), MH-74-BE (#216), MH-74-FE (#217, dépend de MH-74-BE et MH-73-FE)
- **Branches** :
  - BE : `mh-74-be-suspension-across-sessions-sign-in-and-public-visibility`, créée en local depuis `origin/develop` (`6dcfa02`, MH-73 inclus). Pas encore sur origin.
  - FE : pas encore créée. À récupérer ou à demander avant de coder.
  - Ni commit ni push automatique.
- **CDC** : ADMIN-02 (Must) : l'admin peut lister, suspendre ou réactiver et supprimer tout compte.
- **Documentation** (spec §4.1, §4.3, §4.4, §4.6, §4.7, ARCHITECTURE §8.1) : dette portée par MH-76. Rien n'est modifié ici.

## 1. Contexte et objectif
Aujourd'hui `AuthUser` rejette un compte suspendu, mais rien ne permet de suspendre, `/auth/otp/verify` et `/auth/refresh` ne lisent pas `is_active`, et la règle de visibilité publique ignore le propriétaire. Le ticket ajoute :
- **BE** : `PATCH /admin/users/:id`, le refus `ACCOUNT_SUSPENDED` à la connexion et au refresh, et l'owner actif dans la règle de visibilité.
- **FE** : l'action Suspend / Reactivate dans `/admin/users`, et le message affiché à la personne suspendue. Aucun email n'est envoyé à la MVP, ce message est donc le seul avis.

### Décisions
| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Emplacement | Handler, service, repository et DTO dans `modules/users/`, comme MH-73. `PATCH` rejoint `delete_user` dans le même `routes!` (un seul appel par chemin, sinon une méthode disparaît de l'OpenAPI). Retirer le `TODO EP-12` de `admin/router.rs`. |
| D2 | Corps invalide | L'AC demande 422, or `AppJson` répond 400 sur un mauvais type. Comme `ListingStatusRequest`, le champ est lu largement : `is_active: Option<serde_json::Value>` avec `#[schema(required = true, value_type = bool)]`, puis `parse_is_active` (pure) donne 422 `VALIDATION_FAILED` si la valeur manque ou n'est pas un booléen. Un JSON illisible reste 400, comme partout. |
| D3 | Service | `set_active_for_admin` : validation, lecture de la cible (absente : `UserNotFound`), garde admin, écriture, invalidation du cache, log. `ensure_deletable` devient `ensure_not_admin` (MH-75 en sera le troisième appelant). La réponse réutilise la ligne lue avec `AdminUserRow { is_active, ..row }`, sans seconde requête. |
| D4 | Écriture | `UPDATE users SET is_active = $2 WHERE id = $1 AND is_active <> $2`. Zéro ligne signifie que la valeur est déjà en place : succès, aucun log, `updated_at` intact. Une nouvelle lecture `find_for_admin` reprend les colonnes de `list_for_admin`. |
| D5 | Sessions | Choix validé. La suspension ne touche pas aux tokens : le contrôle `is_active` les refuse tous. La réactivation fait, dans la même transaction que l'UPDATE, `DELETE FROM refresh_tokens WHERE user_id = $1`. Un vieux cookie devient inconnu (401 `UNAUTHORIZED`) et ne déclenche pas la réponse anti-vol, qui couperait la nouvelle session d'un autre appareil. Le SQL vit dans `users/repository.rs` : il partage la transaction, et `users` n'importe pas `auth`. |
| D6 | Cache | `is_active_status().invalidate(&id)` après le commit, dans les deux sens, comme D8 de MH-73. La coupure est donc immédiate dans le cas courant, et 8 s reste la borne si une requête concurrente repeuple le cache. |
| D7 | Refresh | `RefreshTokenLookup` gagne `is_active` (`find_by_hash` joint déjà `users`). Le contrôle vient juste après la lecture, avant `is_revoked` et `is_expired` : l'AC vise tous les tokens de la personne, et la fenêtre de grâce ne doit rien réémettre. Le même contrôle s'applique à la relecture de la branche `None` (course de rotation perdue), sinon `handle_revoked` émettrait un token. |
| D8 | Cookie | Choix validé. Sur `AccountSuspended`, le handler `refresh` retire le cookie avec `clear_refresh_cookie`. Le message n'apparaît qu'une fois, puis la personne navigue en anonyme. |
| D9 | OTP | `/otp/request` ne change pas. `find_user_by_email` renvoie aussi `is_active` ; `verify_otp` répond `AccountSuspended` après la preuve du code, avant `mint` et `insert_refresh_token`. Le code est déjà consommé à ce stade. Un mauvais code reste `OTP_INVALID`, donc rien ne révèle la suspension sans le code. |
| D10 | Erreurs | Retirer `AppError::AccountInactive`, sans aucun appelant (`errors.rs:78` et `:193`). `ACCOUNT_SUSPENDED` devient le seul code. |
| D11 | Règle publique | Nouveau fragment `owner_is_eligible_sql!()` : `EXISTS (SELECT 1 FROM users o WHERE o.id = l.owner_id AND o.is_active)`, intégré à `IS_PUBLIC`. Il reste autonome sur l'alias `l`, car les requêtes de comptage ne joignent pas `users`. Le feed, son total, `owner_id`, la recherche, le contact et les filtres admin en héritent. MH-75 y ajoutera le rôle, à cet endroit seulement. |
| D12 | Détail | `find_listing_by_id` passe à `QueryBuilder`, comme la liste admin, pour sélectionner `{IS_PUBLIC} AS is_public` et `{OWNER_IS_ELIGIBLE} AS owner_is_eligible`, ce que `query_as!` ne permet pas sans dupliquer le SQL. `is_visible_to(caller, owner_id, owner_is_eligible, is_public)` vaut `is_public \|\| (owner_is_eligible && caller == Some(owner_id))`. La règle n'a plus de seconde définition en Rust. `has_photo` reste, `publish_decision` s'en sert. Retirer l'entrée `.sqlx/` devenue inutile. |
| D13 | Contact | Retirer `u.is_active` de `find_owner_contact` : la règle le couvre. `u.phone IS NOT NULL` reste. |
| D14 | Admin listings | Rien à coder : `Hidden` vaut déjà `published AND HAS_COVER AND NOT IS_PUBLIC`, et `is_public` est calculé par `IS_PUBLIC`. Un brouillon reste `draft`. Le lien « View » de `AdminListingRow` n'apparaît que pour `public`, donc pas de 404 côté admin. |
| D15 | Log | `tracing::info!(admin_id, user_id, is_active, "account state changed by admin")`, émis uniquement sur un vrai changement. Jamais d'email. |
| D16 | Client FE | Sur un 401, `sendWithRetry` lit le code dans `response.clone()`. `ACCOUNT_SUSPENDED` appelle un handler enregistré par `AuthProvider` et saute le refresh. Le `catch` de `refresh()` traite le même code, pour le cas du jeton expiré et pour le démarrage de l'app. Nouveau helper `isAccountSuspended(error)` à côté de `isNotFound`. |
| D17 | Sortie FE | `AuthContext` expose `wasSuspended`, posé par `markSuspended` (qui appelle `clearSession`) et remis à zéro par `setSession`. `RootLayout` purge les caches comme `signOutTo`, puis navigue vers `LOGIN_PATH` avec `replace`, sans `redirect`. `AdminLayout` n'est pas concerné : un admin n'est pas suspendable. |
| D18 | Messages | Constante unique `"Your account has been suspended."`. `AuthFlow` l'affiche en `Alert` sur l'écran email si `wasSuspended`, `OtpVerifyForm` sur l'erreur `ACCOUNT_SUSPENDED`. `useOtpVerify` n'appelle `setSession` qu'en succès, donc rien à changer. |
| D19 | Action admin | `AccountStateAction` sur le modèle de `DeleteUserAction`, dans la cellule d'action, absente pour un admin. « Suspend » ouvre une `Modal` avec le texte de l'AC, l'erreur en `Alert` dans la modale. « Reactivate » part sans dialogue, l'erreur en toast. |
| D20 | Hook | `useUpdateAdminUserState(id)` sur le modèle de `useUpdateAdminListingStatus`, sans mise à jour optimiste. `onSuccess` retourne l'invalidation du préfixe `[ADMIN_QUERY_ROOT]`, ce qui garde la ligne verrouillée jusqu'au nouveau badge et rafraîchit aussi les biens. Il partage `adminUserMutationKey(id)` avec la suppression. Un 404 reprend le chemin « This account no longer exists ». |

## 2. Découpage BE / FE et dépendances
- **MH-74-BE** n'a pas de dépendance : MH-73 est mergé. Il livre le contrat OpenAPI (`frontend/docs-frontend/openapi.json`).
- **MH-74-FE** dépend du BE pour `types.ts`, régénéré par `generate:types:ci`, et pour le code `ACCOUNT_SUSPENDED` sur verify et refresh. La logique pure de D16 peut démarrer avant, l'intégration attend le BE.
- **MH-75** étend `owner_is_eligible_sql!()` et `is_visible_to`, et réutilise `ensure_not_admin`. **MH-76** porte la documentation.

## 3. Ordre d'implémentation recommandé
### MH-74-BE
1. `listings/repository.rs`, `listings/service.rs`, `listings/model.rs`, `contact/repository.rs` : D11 à D13, puis contrôle du plan de requête (R5).
2. `auth/repository.rs`, `auth/model.rs`, `auth/service.rs`, `auth/handler.rs` : D7 à D9.
3. `shared/errors.rs` : D10.
4. `users/dto.rs`, `users/repository.rs`, `users/service.rs`, `users/handler.rs`, `users/router.rs`, `admin/router.rs` : D1 à D6, D15.
5. Commentaires à corriger : `extractors.rs:19-22` (« not proactively invalidated on suspend » devient faux), doc de `IS_PUBLIC`, doc de `find_owner_contact`.
6. `cargo sqlx prepare`, export OpenAPI, puis contrat qualité (§6) et scénario curl (§6).

### MH-74-FE
1. `shared/api/client.ts` et son test : D16.
2. `features/auth/AuthContext.tsx`, `AuthFlow.tsx`, `OtpVerifyForm.tsx`, `app/layout/RootLayout.tsx` : D16 à D18.
3. `features/admin/api.ts`, `hooks/useUpdateAdminUserState.ts`, `components/AccountStateAction.tsx`, `AdminUserRow.tsx` : D19 et D20.
4. Contrat qualité et parcours Playwright.

## 4. Points d'intégration avec l'existant
| Zone | Fichier (sur `develop`) | Effet |
| ---- | ----------------------- | ----- |
| Règle publique | `listings/repository.rs:15-26`, `:47` | Un seul point de modification pour le feed, la recherche, le contact et l'admin |
| Détail | `listings/repository.rs:330`, `listings/service.rs:116`, `:134`, `:300` | Requête réécrite ; `publish` lit aussi cette ligne |
| Contact | `contact/repository.rs:32` | Filtre redondant retiré |
| Refresh | `auth/service.rs:43-115`, `auth/repository.rs:15`, `auth/handler.rs:183` | Contrôle ajouté, cookie retiré sur refus |
| Connexion | `auth/service.rs:276`, `auth/repository.rs:127` | Contrôle après la preuve du code |
| Extracteur | `shared/extractors.rs:187-199` | Comportement inchangé, commentaire à corriger |
| Admin users | `users/service.rs:134-178`, `users/router.rs` | Nouvelle route, garde partagée |
| Client HTTP | `shared/api/client.ts:94-101` | 401 suspendu sans refresh |
| Session FE | `AuthContext.tsx:47-63`, `RootLayout.tsx:54` | Sortie et purge des caches |
| Table admin | `AdminUserRow.tsx:52`, `api.ts:61` | Action ajoutée, clé de mutation partagée |

## 5. Risques et inconnues
| #  | Risque | Traitement |
| -- | ------ | ---------- |
| R1 | Un access token émis avant la suspension redevient valide si la réactivation arrive avant son expiration (15 min au plus). L'AC « sessions not restored » n'est donc tenue que pour les refresh tokens. | Choix validé : accepté, sans changement de schéma. À consigner dans MH-76, avec le risque équivalent de MH-75. |
| R2 | Un token de refresh émis par une rotation concurrente à la suspension survit. | Sans effet : `is_active` le refuse, et la réactivation supprime toutes les lignes (D5). |
| R3 | `bootstrap_admin` remet `is_active = true` en promouvant un compte existant, sans purger ses tokens (`users/repository.rs:241`). | Seulement s'il n'existe aucun admin, au démarrage. Accepté, à vérifier à la review. |
| R4 | Les photos restent accessibles par URL directe : nginx les sert sans passer par le backend. | Conséquence de la décision verrouillée. L'AC vise le feed, la recherche, le détail et le contact. À consigner dans MH-76. |
| R5 | Le feed et la recherche dépendent maintenant de `users`. | `EXPLAIN ANALYZE` du comptage et de la liste, avec et sans mots-clés : attendu, un accès par clé primaire `users`, `idx_listings_search` conservé. Aucun index ajouté sans mesure. Le MCP PostgreSQL ne s'est pas connecté pendant cette session : si c'est encore le cas, passer par `psql`. |
| R6 | `find_listing_by_id` perd la vérification de sqlx à la compilation (D12). | Le mapping passe par `FromRow` au runtime. La lecture du détail et `publish` entrent dans le scénario curl. |
| R7 | Sur `/login`, la route est hors de `RootLayout` (`router.tsx:151`) : un rechargement perd `wasSuspended`, et le cookie est déjà retiré. | Accepté : le message réapparaît à la vérification du code (D18). |
| R8 | Plusieurs requêtes en vol reçoivent 401 en même temps. | `markSuspended` est idempotent, et aucune ne déclenche de refresh (D16). Dans le cas du jeton expiré, `refreshInFlight` déduplique ; une réponse tardive peut relancer un refresh isolé, jamais une boucle. |
| R9 | L'issue parente #215 est fermée en `COMPLETED` alors que #216 et #217 sont ouverts. | À rouvrir ou à confirmer par l'équipe. Aucun impact sur le code. |

## 6. Contrat qualité (à appliquer sur chaque branche)
Skills et outils vérifiés dans cette session : `code-review-backend` et `code-review-frontend` (`.claude/skills/`), `humanizer:humanizer` (plugin), Context7 (le connecteur répond à la résolution d'une bibliothèque).

1. **Avant de coder** : `git fetch`, puis se placer sur la sous-branche du ticket. Ni commit ni push automatique.
2. **Context7** : vérifier bonnes pratiques, sécurité et idiomatisme sur les API concernées. BE : retour d'un `CookieJar` sur le chemin d'erreur Axum, `QueryBuilder` et `FromRow` sqlx, transactions. FE : `invalidateQueries` et `useIsMutating` de TanStack Query, navigation React Router.
3. **Code minimal, KISS** : rien au-delà des AC, pas d'abstraction pour un seul appelant.
4. **Tests unitaires sur la logique pure uniquement**, sans dépendance à la base :
   - BE : `parse_is_active`, `ensure_not_admin`, nouvelle matrice de `is_visible_to`.
   - FE : `isAccountSuspended`, le 401 suspendu sans refresh dans `client.test.ts`.
   - Les cas qui exigent une vraie base vont à l'epic d'intégration et sont notés comme dette de tests.
5. **Première review avant tout fix** : review complète d'abord, puis fixes appliqués avant de continuer.
6. **Review dédiée par domaine** : `code-review-backend` sur la branche BE, `code-review-frontend` sur la branche FE.
7. **Passe `humanizer:humanizer`** sur les commentaires : ton humain, neutre, concis, fidèle au code. Pas de référence à un fichier de règles.
8. **Vérifications de fin** :
   - BE : `cargo check`, `cargo fmt`, `cargo clippy`, sans `cargo test` complet. Scénario curl : 200 et idempotence, 403, 404, 422, refresh avec token actif, révoqué et expiré, OTP correct et incorrect, feed et total, `owner_id`, recherche, détail, contact, `hidden`, réactivation.
   - FE : lint, typecheck, tests purs. Parcours Playwright : suspension en session, code correct d'un compte suspendu, absence de boucle de refresh dans le réseau, badge après échec.
   - `graphify update .` après les changements de code.
