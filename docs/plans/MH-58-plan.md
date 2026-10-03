# MH-58 — Supprimer un bien : plan d'implémentation (BE + FE)

- **Epic** : EP-09 — Listings (#129)
- **Ticket parent** : MH-58 — Delete a listing (#147)
- **Sous-tickets** : MH-58-BE (#148) → MH-58-FE (#149)
- **Branches** : le parent `mh-58-delete-a-listing` existe déjà, créé depuis `develop` après le merge de MH-57 (#163). L'utilisateur crée la sous-branche BE depuis le parent, la merge, puis crée la sous-branche FE depuis le parent à jour. Une seule PR part ensuite du parent vers `develop` (modèle MH-55 / MH-56 / MH-57). Aucun code n'est écrit sur le parent.
- **Déjà livré, et utilisé ici** : MH-50/51/52 (photos, verrou du bien sur upload/cover/suppression de photo, nettoyage d'orphelin à l'upload), MH-55 (« My properties », zone d'actions de `OwnerListingRow`), MH-57 (`withoutListing`, garde `isMutating` sur `["listing-status"]`), suppression de compte (helper best-effort `delete_storage_objects` et double `RecordingStorage`).
- **Exigence CDC** : LIST-02, « Un owner peut modifier et supprimer uniquement ses propres biens » (Must).

Chaque sous-ticket est implémenté dans sa propre session et sur sa propre sous-branche, selon le contrat qualité de la section 7.

---

## 1. Contexte et objectif

Un owner doit pouvoir retirer définitivement un bien, avec ses photos. Pas de soft delete, pas d'annulation. La base supprime les lignes `listing_media` par la cascade existante (`listing_media.listing_id … ON DELETE CASCADE`, seule FK vers `listings`). Les fichiers sont supprimés par l'application **après le commit** : une panne du storage laisse au pire un fichier orphelin, jamais une ligne qui pointe vers un fichier absent. La suppression n'est proposée que dans « My properties ».

- **BE** : `DELETE /listings/:id` → 204 sans corps. Verrou du bien, collecte des clés, `DELETE`, commit, puis suppression best-effort des fichiers.
- **FE** : action « Delete » sur chaque ligne de « My properties », modale de confirmation qui nomme le bien, ligne retirée et caches mis à jour sur 204 (et sur 404).

### Décisions prises

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Branche de base | Parent `mh-58-delete-a-listing`, déjà à jour de `develop` (MH-57 inclus). |
| D2 | Emplacement du helper | `delete_storage_objects` et `RecordingStorage` quittent `users/service.rs` pour `backend/src/infra/storage/` (à côté du trait `StorageProvider`). Users et listings l'importent, comme media importe déjà `StorageProvider`. |
| D3 | Signature du helper | `delete_storage_objects(storage: &dyn StorageProvider, keys: &[String])`. Le paramètre `user_id`, utilisé seulement dans le log, disparaît : la clé porte déjà l'entité (`listings/{id}/…`, `avatars/{user_id}/…`) et le `request_id` (MH-30) corrèle la ligne de log à la requête. |
| D4 | Ordre storage / SQL | Celui du ticket : commit, puis storage. `delete_account` garde son ordre actuel (storage avant SQL). ARCHITECTURE §8.1 et `.claude/rules/database.md` sont amendés dans MH-58-BE pour distinguer les deux cas. |
| D5 | Verrou + propriété | Une requête `SELECT id FROM listings WHERE id = $1 AND owner_id = $2 FOR UPDATE` dans le module listings. Aucune ligne → `ListingNotFound` (id inconnu, bien d'un autre owner, bien déjà supprimé : même 404). `media::repository::lock_listing_owner` n'est pas réutilisé : un module n'importe pas l'implémentation concrète d'un autre. |
| D6 | TECHNICAL_SPEC | Pas de mise à jour : §4.3 liste déjà `DELETE /listings/:id`. L'OpenAPI généré fait référence. |
| D7 | Modale non fermable | Garde dans `handleClose` (motif de `DeleteAccountSection`) : rien ne se passe si `isPending`. `Modal` partagé inchangé ; la croix reste visible mais sans effet pendant la requête. |
| D8 | Retrait de la ligne | Pas d'optimisme : la ligne est retirée sur 204/404 via `withoutListing` (déjà testé), puis le cache est réconcilié par invalidation. |

---

## 2. Découpage BE / FE et dépendances

```
MH-58-BE (#148)  ──►  merge dans le parent  ──►  MH-58-FE (#149)
  infra/storage : helper + double partagés  │      types.ts régénéré
  users/service : import du helper partagé  │      api.deleteListing
  repository : lock / keys / delete         │      isAlreadyDeleted (+ test)
  service::delete_listing                   │      useDeleteListing
  handler DELETE + route + OpenAPI          │      DeleteListingAction (bouton + Modal)
  doc §8.1 + database.md, .sqlx, openapi    │      intégration dans OwnerListingRow
```

Le FE dépend du BE pour l'endpoint et pour les types générés. MH-55-FE et MH-57-FE sont déjà livrés.

### 2.1 MH-58-BE : fichiers, dans l'ordre (un fichier à la fois)

1. **`backend/src/infra/storage/cleanup.rs`** (nouveau) : `pub async fn delete_storage_objects(storage: &dyn StorageProvider, keys: &[String])`, corps repris de `users/service.rs:128`. Chaque échec (fichier absent compris) est logué en `warn` avec la clé et l'erreur, puis la boucle continue. Les trois tests existants y déménagent (voir 2.2).
2. **`backend/src/infra/storage/test_support.rs`** (nouveau, `#[cfg(test)]`) : `RecordingStorage` repris tel quel de `users/service.rs:166` (`new(fails)`, `deleted_keys()`).
3. **`backend/src/infra/storage/mod.rs`** : `mod cleanup; pub use cleanup::delete_storage_objects;` et `#[cfg(test)] pub(crate) mod test_support;`.
4. **`backend/src/modules/users/service.rs`** : suppression du helper local, de `RecordingStorage` et des trois tests déplacés ; `delete_account` appelle `crate::infra::storage::delete_storage_objects(storage, &keys)`. Les tests de `delete_previous_avatar` importent `RecordingStorage` depuis `test_support`. Aucun changement de comportement.
5. **`backend/src/modules/listings/repository.rs`** : trois fonctions génériques sur `sqlx::PgExecutor<'e>` (même forme que `media/repository.rs`), `query!`/`query_scalar!` compilés :
   - `lock_owned_listing(executor, id, owner_id) -> Result<bool, AppError>` : `SELECT id FROM listings WHERE id = $1 AND owner_id = $2 FOR UPDATE`, `fetch_optional`, `.is_some()`.
   - `list_media_keys(executor, listing_id) -> Result<Vec<String>, AppError>` : `SELECT storage_key FROM listing_media WHERE listing_id = $1`.
   - `delete_listing(executor, id) -> Result<(), AppError>` : `DELETE FROM listings WHERE id = $1`. La cascade retire les photos.
6. **`backend/src/modules/listings/service.rs`** : `delete_listing(pool, storage: &dyn StorageProvider, id, owner_id) -> Result<(), AppError>`.
   - `pool.begin()` ; `lock_owned_listing` → `false` → `ListingNotFound` (la transaction abandonnée fait le rollback) ;
   - `list_media_keys` ; `repository::delete_listing` ; `commit` ;
   - `delete_storage_objects(storage, &keys).await` ; `Ok(())`.
   - Doc comment d'une ou deux lignes : pourquoi le commit précède le storage, et que le verrou sérialise avec upload / cover / suppression de photo.
7. **`backend/src/modules/listings/handler.rs`** : `delete(State, AuthUser, Path(id)) -> Result<StatusCode, AppError>`. `user.require_role(OWNER_ROLES)?` d'abord, puis `service::delete_listing(state.db(), state.storage().as_ref(), id, user.user_id)`, puis `StatusCode::NO_CONTENT` (motif de `media/handler.rs::delete_media`). `#[utoipa::path(delete, path = "/listings/{id}", tag = "listings", params(id), responses(204, 401, 403, 404))]`.
8. **`backend/src/modules/listings/router.rs`** : `routes!(handler::get_by_id, handler::update, handler::delete)` (même chemin), commentaire « delete is owner-only », retrait du `// TODO: owner delete.`.
9. **Docs** (D4) :
   - `docs/ARCHITECTURE_v1.2.md` §8.1 : une phrase après « Suppression de compte » : la suppression d'un bien ou d'une photo commit d'abord, puis supprime les fichiers en best-effort (orphelin possible, jamais de ligne vers un fichier absent).
   - `.claude/rules/database.md`, « Cascades and cleanup » : le même distinguo, en une ligne.
10. **Artefacts** : `cargo sqlx prepare` (`.sqlx/`, trois nouvelles requêtes) et `cargo run --bin gen_openapi` (`frontend/docs-frontend/openapi.json`).

### 2.2 MH-58-BE : tests unitaires (logique pure, sans DB)

- `infra/storage/cleanup.rs`, contre `RecordingStorage` : ce sont les trois tests existants de `users/service.rs`, déplacés (`deletes_every_enumerated_key`, `a_missing_or_failing_key_does_not_abort_the_remaining_keys`, `no_keys_is_a_no_op`). Ils couvrent exactement les trois cas du ticket ; aucun nouveau test à écrire.
- `users/service.rs` : les tests de `delete_previous_avatar` restent et doivent passer avec le double partagé.
- `handler.rs` : `only_owners_may_manage_their_listings` couvre déjà la règle de rôle de la suppression. Rien à ajouter.
- **Dette** (base réelle requise, epic tests d'intégration) : 204 et disparition des lignes `listings`/`listing_media`, 404 identique (inconnu / autre owner / déjà supprimé), fichiers supprimés après commit, upload concurrent → 404 sans fichier restant. Vérifiés à la main (section 5).

### 2.3 MH-58-FE : fichiers, dans l'ordre (un fichier à la fois)

1. **`frontend/src/shared/api/types.ts`** : régénéré avec `npm run generate:types:ci`, jamais à la main.
2. **`frontend/src/features/listings/api.ts`** : `deleteListing(id): Promise<void>` → `apiDelete<void>(\`/api/v1/listings/${encodeURIComponent(id)}\`)`. `request` renvoie déjà `undefined` sur 204.
3. **`frontend/src/features/listings/listingDeletion.ts`** (+ `.test.ts`) : `isAlreadyDeleted(error: unknown): boolean` → `error instanceof ApiError && error.status === 404`. C'est la seule règle métier pure du FE (le 404 compte comme un succès).
4. **`frontend/src/features/listings/hooks/useUpdateListingStatus.ts`** : exporter la clé de mutation (`LISTING_STATUS_MUTATION_KEY`) pour la garde du hook suivant. Aucun autre changement.
5. **`frontend/src/features/listings/hooks/useDeleteListing.ts`** : `useMutation({ mutationFn, retry: false, onSuccess })`.
   - `mutationFn` : `await deleteListing(id)` ; une erreur pour laquelle `isAlreadyDeleted` est vrai est avalée, toute autre est relancée.
   - `onSuccess` (au niveau du hook, donc exécuté même si la ligne est démontée) :
     - `setQueriesData(["owner-listings"], (r) => withoutListing(r, id))` ;
     - `removeQueries({ queryKey: ["listing", id] })` ;
     - `invalidateQueries(["listings"])` ;
     - `invalidateQueries(["owner-listings"])` seulement si `isMutating({ mutationKey: LISTING_STATUS_MUTATION_KEY }) === 0` (sinon le `onSettled` du changement de statut s'en charge, voir R2) ;
     - toast `success` « Listing deleted. ».
6. **`frontend/src/features/listings/components/DeleteListingAction.tsx`** (nouveau)
   - Props : `listingId`, `listingTitle`.
   - Déclencheur : bouton texte avec icône `Trash2` (lucide), « Delete », `aria-label={\`Delete ${listingTitle}\`}`, même style que le lien « Edit ».
   - `Modal` partagé, titre « Delete this listing? ». Corps : le titre du bien, puis « This listing and all its photos will be permanently deleted. This can't be undone. »
   - Footer : `Cancel` (`secondary`, `disabled={isPending}`) et `Delete permanently` (`danger`, `isLoading={isPending}`, `disabled={isPending}`).
   - `handleClose` ne fait rien si `isPending` (D7) ; `mutation.reset()` à l'ouverture pour ne pas réafficher une erreur passée.
   - Erreur autre que 404 : `Alert` `error` dans la modale (« Couldn't delete the listing. Please try again. »), modale ouverte, ligne en place.
   - Succès : la ligne disparaît du cache, le composant est démonté avec elle, la modale aussi.
7. **`frontend/src/features/listings/components/OwnerListingRow.tsx`** : `<DeleteListingAction listingId={listing.id} listingTitle={listing.title} />` dans la zone d'actions `relative z-10`, après « Edit ». La modale est rendue en portail, donc hors du lien étiré `::after`.

### 2.4 MH-58-FE : tests unitaires (vitest, logique pure)

- `isAlreadyDeleted` : `ApiError` 404 → `true` ; `ApiError` 403 et 500 → `false` ; `TypeError` (réseau) et `undefined` → `false`.
- `withoutListing` : déjà couvert par MH-57, rien à ajouter.
- **Dette** (pas encore de React Testing Library) : ouverture/fermeture de la modale, verrouillage pendant la requête, Escape/overlay ignorés, message d'erreur, retrait de la ligne, invalidations, toast. Vérifiés à la main (section 5).

---

## 3. Ordre d'implémentation recommandé

1. L'utilisateur crée la sous-branche BE depuis `mh-58-delete-a-listing` (`git fetch` puis checkout ; demander si elle manque).
2. **MH-58-BE**, dans l'ordre de 2.1 :
   1. extraction du helper et du double (1 → 4), puis `cargo check` et les tests users, avant d'écrire du code listings : le refactor est validé seul ;
   2. repository, service, handler, router (5 → 8) ;
   3. docs (9), artefacts (10), vérifications (section 5), contrat qualité (section 7).
3. L'utilisateur merge la sous-branche BE dans le parent et crée la sous-branche FE depuis le parent à jour.
4. **MH-58-FE**, dans l'ordre de 2.3 : types et `api.ts` ; `isAlreadyDeleted` et son test ; export de la clé ; hook ; composant ; intégration dans la ligne.
5. L'utilisateur merge la sous-branche FE dans le parent, puis ouvre une PR unique vers `develop`.

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement MH-58 |
| ---- | -------- | ---------------- |
| `users/service.rs` | `delete_storage_objects` (privé, `user_id` pour le log), `RecordingStorage` et 3 tests | Déplacés dans `infra/storage/` (D2, D3). `delete_account` inchangé fonctionnellement. |
| `infra/storage/` | Trait `StorageProvider`, `LocalFsStorage::delete` → erreur typée si la clé n'existe pas | Accueille le helper et le double. Le fichier absent passe par la branche « warn et continue ». |
| Migration de base | `listing_media.listing_id … ON DELETE CASCADE` ; aucune autre FK vers `listings` | Aucune migration. |
| `media/service.rs::upload` | `persist_media` reprend le verrou du bien ; bien absent → `ListingNotFound`, puis `storage.delete(key)` de l'objet déjà écrit | Rien à coder : l'upload concurrent finit en 404 sans fichier restant. À vérifier (R1). |
| `media` : `promote_cover`, `delete` | Verrouillent la ligne `listings` (`FOR UPDATE` / `FOR UPDATE OF l, m`) | Sérialisés avec la suppression du bien ; après elle → 404. |
| `listings/handler.rs` | `OWNER_ROLES`, `require_role`, test des rôles | Réutilisés : 403 pour seeker et admin, 401 sans jeton via `AuthUser` (`is_active` revérifié). |
| `AppError::ListingNotFound` | 404 `LISTING_NOT_FOUND` | Réutilisé pour les trois cas de 404. |
| `AppState::storage()` | `&Arc<dyn StorageProvider>` | Passé en `.as_ref()`, comme pour la suppression de photo. |
| `shared/api/client.ts` | `apiDelete`, 204 → `undefined`, `ApiError.status` | Réutilisés. |
| `ownerListingsCache.ts` | `withoutListing` | Réutilisé sur succès. |
| `useUpdateListingStatus` | Clé `["listing-status"]`, refetch de la liste par la dernière mutation en cours | Clé exportée ; garde symétrique dans `useDeleteListing`. |
| Clés de cache | `["owner-listings", { page }]`, `["listings", …]`, `["listing", id]` | Retrait de la ligne + invalidation ; feed invalidé ; détail supprimé du cache. |
| `OwnerListings.tsx` | Page au-delà de la dernière → remplacée par la dernière (« after deletions ») | Couvre le cas « dernière ligne d'une page supprimée ». |
| `Modal`, `Button`, `Alert`, `useToast` | `DeleteAccountSection` : même motif de confirmation destructive | Réutilisés, aucun nouveau primitif. |
| `removeOwnerScopedQueries` | Purge à la déconnexion | Inchangé. |

---

## 5. Vérifications

**BE** : `cargo fmt`, `cargo check`, `cargo clippy`. `cargo test` ne se lance qu'avec l'accord explicite de l'utilisateur, ciblé sur `infra::storage`, `users` et `listings`. Swagger (`/api/docs`) : `DELETE /listings/{id}` avec 204/401/403/404. À contrôler avec curl (APP_ENV=development, LocalFsStorage) :

- owner, bien avec 2–3 photos : 204 ; `SELECT` sur `listings` et `listing_media` vide pour cet id ; dossier `listings/{id}/` vide sous le chemin de stockage local ; `GET /listings/{id}` → 404 ; le bien n'est plus dans `GET /listings` ni dans `GET /users/me/listings` ;
- bien sans photo / brouillon → 204 sans appel storage ;
- un fichier supprimé à la main avant l'appel → 204 et un `warn` dans les logs, les autres fichiers supprimés ;
- second `DELETE` sur le même id, id inconnu, bien d'un autre owner → le même 404 `LISTING_NOT_FOUND` ;
- jeton seeker ou admin → 403 ; sans jeton → 401 ;
- course upload/suppression : relecture du chemin `upload` → `persist_media` → `check_upload_slot` (verrou, `ListingNotFound`, `storage.delete`). Essai manuel possible : un upload lent (gros fichier) lancé, puis le `DELETE` ; l'upload doit finir en 404 et aucun fichier ne doit rester.

**FE** : `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Vérification à la main avec le skill `run`, ou Playwright headless :

- chaque ligne affiche « Delete » ; la modale nomme le bien et annonce la suppression des photos ; le focus entre dans la modale et revient au déclencheur sur « Cancel » ;
- requête ralentie (throttling) : bouton de confirmation en chargement, les deux boutons désactivés, Escape / clic overlay / croix sans effet ;
- 204 : modale fermée, ligne retirée, toast de succès ; feed et URL du détail → le bien n'apparaît plus, sans rechargement manuel ;
- bien supprimé ailleurs (SQL) puis suppression → même issue qu'un 204 ;
- serveur coupé ou 403 forcé : modale ouverte avec le message d'erreur, ligne en place ; une réouverture n'affiche plus l'ancienne erreur ;
- dernière ligne de la page 2 supprimée → bascule sur la page 1 ;
- changement de statut sur une ligne puis suppression d'une autre : les deux états finaux sont corrects ;
- largeur 360 px : contrôle de statut + « Edit » + « Delete » sans débordement ni scroll horizontal.

---

## 6. Risques et inconnues

| #  | Risque / inconnue | Traitement |
| -- | ----------------- | ---------- |
| R1 | Upload concurrent : le fichier écrit avant le second verrou doit être supprimé. | Déjà géré par `upload` (orphan cleanup). Vérification seulement, pas de nouveau code (note technique du ticket). |
| R2 | Refetch de « My properties » déclenché par la suppression pendant un changement de statut sur une autre ligne : il écraserait l'état optimiste. | Garde `isMutating({ mutationKey: LISTING_STATUS_MUTATION_KEY }) === 0` avant d'invalider ; le statut en cours refetchera de lui-même. À confirmer via Context7 (TanStack Query v5). |
| R3 | Focus après succès : le déclencheur est démonté avec la ligne, le retour de focus de `Modal` vise un élément absent et le focus tombe sur `body`. | Accepté pour le MVP (le toast annonce le résultat). À noter dans la dette d'accessibilité si la review FE le relève. |
| R4 | Callbacks de `mutate()` non appelés après démontage (TanStack v5). | Toute la logique de succès vit dans le `onSuccess` du hook, pas dans `mutate(…, { onSuccess })`. |
| R5 | Divergence doc : ARCHITECTURE §8.1 et `database.md` imposent « storage avant SQL ». | Amendés dans MH-58-BE (D4). `CLAUDE.md` cite `docs/ARCHITECTURE.md` alors que le fichier est `ARCHITECTURE_v1.2.md` : hors périmètre, à signaler seulement. |
| R6 | Perte de contexte dans le log de `delete_account` (`user_id` retiré, D3). | Clé et `request_id` suffisent à retrouver l'appel. Si la review BE le conteste, ajouter un champ de contexte `&str` plutôt qu'un `Uuid` typé par module. |
| R7 | Mise en page : trois actions dans la zone d'actions sur mobile. | La zone passe déjà en pleine largeur sous 640 px ; à vérifier à 360 px. |
| R8 | MCP PostgreSQL et Sequential Thinking en échec pendant l'analyse ; plugin GitHub en échec (remplacé par `gh`). | Réessayer le MCP PostgreSQL au début de MH-58-BE, sinon s'appuyer sur la migration de base. Les requêtes filtrent sur des clés (PK, `listing_id` de `listing_media`) : aucun index nouveau, rien sur le prix. |
| R9 | Admin `DELETE /admin/listings/:id` (ADMIN-01, EP-12). | Hors périmètre. Le helper partagé et `service::delete_listing` serviront de base ; ne pas anticiper. |
| R10 | Monétisation non définie. | Aucune dépendance pour ce ticket. |

---

## 7. Contrat qualité (sur chaque branche, BE puis FE)

1. **Orientation** : `graphify query`, `path` ou `explain` avant tout grep ou lecture. Relire ce plan et le ticket (`gh issue view 148` ou `149`).
2. **Context7**, avant d'écrire le code qui utilise une librairie, pour vérifier bonnes pratiques, sécurité et idiome de la version installée :
   - BE : transaction sqlx 0.8 (`pool.begin()`, `&mut *tx` comme `PgExecutor`, rollback au drop), `query_scalar!` avec `FOR UPDATE`, `query!` sur `DELETE` ; `#[utoipa::path(delete, …)]` avec une réponse 204 sans corps (utoipa 5) ; handler axum 0.8 renvoyant `StatusCode`.
   - FE : `useMutation` (callbacks du hook vs de `mutate` après démontage), `setQueriesData`, `removeQueries`, `invalidateQueries`, `isMutating` avec `mutationKey` (TanStack Query v5).
3. **Code minimal, KISS** : rien d'autre que ce dont le ticket a besoin. BE : un helper déplacé, trois requêtes, une fonction de service, un handler. FE : une fonction d'API, un prédicat pur, un hook, un composant.
4. **Fichier par fichier**, dans l'ordre de 2.1 puis 2.3, en montrant chaque fichier avant de passer au suivant. Règles actives : `.claude/rules/rust.md`, `database.md`, `general-coding.md` (BE) ; `react-typecrypt.md`, `general-coding.md` (FE). Commentaires d'une ou deux lignes, seulement là où le code ne s'explique pas seul, sans citer un fichier de règles ou de doc comme justification.
5. **Tests unitaires sur la logique pure uniquement** (2.2 et 2.4) : aucune dépendance à la DB, aucun mock de la DB.
6. **Vérifications** de la section 5.
7. **Première review, avant tout fix** :
   - BE avec le skill `code-review-backend`, FE avec `code-review-frontend` (review dédiée par domaine) ;
   - reproduire chaque finding avant de le corriger, et comparer avec `HEAD` ;
   - **appliquer les corrections confirmées avant de continuer**, puis refaire l'étape 6.
8. **Passe `humanizer:humanizer`** sur les commentaires, doc comments, messages de log, descriptions OpenAPI, toasts, textes de la modale et amendements de doc : ton humain, neutre, concis et fidèle au code.
9. **`graphify update .`**
10. **Mémoire** : noter la dette de tests de 2.2 et 2.4.
11. **Arrêt** : ni commit ni push. L'utilisateur relit, commit, merge et ouvre la PR.

### Outils vérifiés pendant l'analyse

- **Skills du repo** (`.claude/skills/`) : `code-review-backend`, `code-review-frontend`, `graphify`, `github-ticket`, `docker`, `readme`.
- **Skills de plugins utilisés** : `humanizer:humanizer`, `run`.
- **MCP disponibles** : Context7 (`mcp__context7__*`, `mcp__claude_ai_Context7__*`), Git MCP (lecture seule).
- **MCP en échec pendant l'analyse** : PostgreSQL (R8), plugin GitHub (remplacé par la CLI `gh`), Sequential Thinking (pas nécessaire ici).

---

## 8. Hors périmètre

Suppression par l'admin (EP-12) ; soft delete ou annulation ; suppression depuis le détail, l'`OwnerBar` ou le formulaire d'édition (le ticket la limite à « My properties ») ; job de nettoyage des orphelins ; changement de `Modal` partagé (D7) ; mise à jour de TECHNICAL_SPEC (D6) ; tests d'intégration (epic dédiée).

---

## 9. Critères de fin

**MH-58-BE**

- [ ] `DELETE /listings/:id` → 204 pour l'owner ; bien et photos supprimés en base par la cascade
- [ ] Fichiers supprimés après le commit ; échec ou fichier absent → `warn`, les autres continuent, réponse 204
- [ ] 404 `LISTING_NOT_FOUND` identique : id inconnu, bien d'un autre owner, bien déjà supprimé ; 403 seeker/admin ; 401 sans jeton
- [ ] Upload concurrent → 404 sans fichier restant (vérifié)
- [ ] Helper et double partagés dans `infra/storage/`, utilisés par users et listings ; aucune copie
- [ ] OpenAPI 204/401/403/404 ; `openapi.json` et `.sqlx` à jour ; §8.1 et `database.md` amendés
- [ ] Tests de 2.2 ; fmt, check et clippy propres

**MH-58-FE**

- [ ] Types régénérés ; `deleteListing`
- [ ] « Delete » sur chaque ligne ; modale avec le titre et l'avertissement sur les photos
- [ ] Pendant la requête : chargement, boutons désactivés, modale non fermable
- [ ] 204 ou 404 : modale fermée, ligne retirée, toast, feed et « My properties » invalidés, détail retiré du cache
- [ ] Autre erreur : modale ouverte avec message, ligne en place
- [ ] Tests de 2.4 ; lint, ts:check, test et prettier propres
