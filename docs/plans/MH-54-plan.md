# MH-54 — Garder les brouillons privés jusqu'à publication : plan d'implémentation (BE + FE)

- **Epic** : EP-09 — Listings (#129)
- **Ticket parent** : MH-54 — Keep draft listings private until published (#135)
- **Sous-tickets** : MH-54-BE (#136) → MH-54-FE (#137)
- **Branches** : créées par l'utilisateur, toutes deux depuis `develop`.
- **Hors ticket, mais lié** : l'action « Publish » (MH-61), l'écran photos (MH-59), « Mes biens » (MH-55), l'édition (MH-56).

Ce plan couvre les deux sous-tickets. Chaque sous-ticket est implémenté dans sa propre session, sur sa propre branche, en suivant le contrat qualité de la section 7.

---

## 1. Contexte et objectif

Depuis MH-53, `POST /listings` crée un bien qui apparaît immédiatement dans le feed public, sans photo. MH-54 introduit un état de publication :

- `listings.published_at TIMESTAMPTZ NULL`. `NULL` signifie brouillon.
- **Règle de visibilité publique** : un bien est public si et seulement si `published_at IS NOT NULL` **et** il a au moins une photo. Au moins une photo équivaut à avoir une ligne de couverture : la première photo uploadée devient la couverture, et la couverture ne peut être supprimée que si c'est la dernière photo.
- La règle s'applique à toute lecture publique : page du feed, total du feed, filtre `owner_id`, détail.
- `GET /listings/:id` passe en authentification **optionnelle**. Le propriétaire voit toujours son bien. Tout autre appelant (anonyme, seeker, admin, autre owner) reçoit un 404 identique à celui d'un id inconnu.
- Côté front, la page de détail devient la page de relecture du propriétaire : une barre propriétaire affiche l'état (brouillon, publié, masqué).

Ce ticket n'ajoute pas l'action de publication (MH-61). Après MH-54, un bien créé reste donc un brouillon jusqu'à MH-61.

### Décisions tranchées

| #  | Sujet                      | Décision                                                                                                                                                                                                                                                                                                          |
| -- | -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| D1 | Specs                      | TECHNICAL_SPEC est mis à jour dans MH-54-BE : colonne`published_at` dans le DDL (§2), authentification optionnelle et 401 sur `GET /listings/:id`, champ `published_at` dans la réponse (§4.3), règle de visibilité.                                                                                   |
| D2 | Biens existants sans photo | La règle du ticket BE s'applique. La migration renseigne`published_at` pour toutes les lignes existantes, mais une ligne sans couverture reste masquée. Le critère parent « les biens existants restent publics » se lit comme « restent publiés ». Les 6 biens du seed ont tous une couverture.         |
| D3 | Cache au sign-out          | Les requêtes liées au propriétaire (`["listing", …]`) sont retirées dans les handlers de sign-out explicites (`RootLayout`, `AdminLayout`), via un helper unique. `AuthContext.clearSession` n'est pas modifié : le retirer là casserait la séquence 401 → refresh raté → retry anonyme → 200. |
| D4 | Branches                   | Les deux branches partent de`develop`. Le FE régénère les types à partir du BE mergé.                                                                                                                                                                                                                       |

### Décisions de conception prises pendant l'analyse

- **Pas d'index sur `published_at`.** Le volume MVP ne le justifie pas, et les règles base de données interdisent d'ajouter un index sans l'avoir signalé. Le point est listé en risque (R5).
- **Pas de `WHERE` dépendant de l'appelant dans la requête de détail.** Une seule requête, puis une décision pure dans le service. « Masqué » et « inexistant » produisent ainsi la même réponse par construction.
- **Pas de nouvel état métier côté Rust** (pas d'enum `Draft/Published`). Le service dérive `is_published = published_at.is_some()`. Le trio brouillon, publié, masqué n'existe que côté front, où il sert à l'affichage.
- **`cover_photo_url` reste `Option<String>`** dans `ListingSummaryDto`, même si le feed ne renvoie plus de ligne sans couverture. MH-55 (« Mes biens ») réutilisera ce DTO pour des biens sans photo.
- **Le filtre `owner_id` du feed reste public** : un owner qui filtre sur son propre id ne voit pas ses brouillons. Ses brouillons relèvent de MH-55.
- **Un admin reçoit aussi le 404** (critère explicite du ticket BE). La modération des biens relève d'EP-12.

---

## 2. Découpage BE / FE et dépendances

```
MH-54-BE (#136)  ──►  merge develop  ──►  MH-54-FE (#137)
  migration + seed            │              types.ts régénéré (published_at)
  prédicat de visibilité      │              helper listingVisibility
  MaybeAuthUser               │              barre propriétaire
  décision d'accès (service)  │              purge du cache au sign-out
  published_at dans le DTO    │              texte du formulaire de création
  openapi.json + TECH_SPEC    │
```

Le FE dépend du BE pour deux choses : le champ `published_at` dans `ListingDetailDto`, et le comportement de `GET /listings/:id` pour un appelant authentifié. Le token est déjà envoyé par `client.ts` quand il existe, le FE n'a rien à changer pour cela.

### 2.1 MH-54-BE — Fichiers, dans l'ordre (un fichier à la fois)

1. **`backend/migrations/<timestamp>_listings_published_at.sql`** (`sqlx migrate add listings_published_at`)

   - `ALTER TABLE listings ADD COLUMN published_at TIMESTAMPTZ NULL;`
   - `UPDATE listings SET published_at = created_at;`
   - Migration forward-only, sans index (voir R5).
2. **`backend/seed/seed_listings.sql`** : ajouter `published_at` à l'`INSERT INTO listings` (valeur `NOW()`), pour qu'un seed sur une base neuve, appliqué après les migrations, remplisse encore le feed. Mettre à jour l'en-tête du fichier si nécessaire.
3. **`backend/src/modules/listings/model.rs`** : `ListingDetailRow` reçoit `published_at: Option<String>` et `has_photo: bool`.
4. **`backend/src/modules/listings/repository.rs`**

   - Une fonction unique `push_public_visibility(qb)`, qui ajoute `AND l.published_at IS NOT NULL AND EXISTS (SELECT 1 FROM listing_media c WHERE c.listing_id = l.id AND c.is_cover)`. Elle est appelée par `count_listings` et par `list_listings`, juste avant `push_filters`. Elle est `pub` pour être réutilisable par EP-10 et EP-11 (voir R4).
   - `list_listings` : le `LEFT JOIN listing_media lm … is_cover` devient `JOIN`.
   - `count_listings` : n'a aujourd'hui aucune jointure ; le prédicat partagé aligne `total` et `total_pages` sur les pages renvoyées.
   - `find_listing_by_id` : ajouter `to_char(l.published_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS published_at` (nullable, donc `Option<String>`) et `EXISTS (SELECT 1 FROM listing_media c WHERE c.listing_id = l.id AND c.is_cover) AS "has_photo!"`. Le `WHERE` reste `l.id = $1`.
   - `cargo sqlx prepare` ensuite (cache `.sqlx/`).
5. **`backend/src/shared/extractors.rs`**

   - Nouvel extracteur `MaybeAuthUser(pub Option<AuthUser>)`.
   - Pas d'en-tête `Authorization` : `None`.
   - En-tête présent : même chemin que `AuthUser` (`bearer_token` puis `resolve_identity`, avec la revérification `is_active`). Toute erreur est propagée : en-tête mal formé, jeton invalide ou expiré, compte suspendu donnent un 401, jamais un repli sur l'anonyme.
   - Pas d'`Option<AuthUser>` générique d'axum : il avale le rejet, et un owner au jeton expiré recevrait un 404 silencieux au lieu du 401 qui déclenche le refresh côté front.
   - Pour rester testable sans DB, isoler la lecture de l'en-tête dans une fonction pure (par exemple `optional_bearer_token(&Parts) -> Result<Option<&str>, AppError>`).
6. **`backend/src/modules/listings/service.rs`**

   - Fonction pure `is_visible_to(caller: Option<Uuid>, owner_id: Uuid, is_published: bool, has_photo: bool) -> bool` : vrai si l'appelant est le propriétaire, sinon `is_published && has_photo`.
   - `get_listing_detail(pool, id, caller: Option<Uuid>)` : ligne absente ou non visible → `AppError::ListingNotFound`. Les médias ne sont chargés qu'après la décision, pour que « masqué » et « inexistant » coûtent une seule requête chacun.
   - `create_listing` appelle `get_listing_detail(pool, id, Some(owner_id))`. La réponse 201 porte alors `"published_at": null`.
7. **`backend/src/modules/listings/dto.rs`** : `ListingDetailDto.published_at: Option<String>`, sérialisé en `null` (pas de `skip_serializing_if`), documenté comme ISO 8601 UTC nullable dans le schéma utoipa.
8. **`backend/src/modules/listings/handler.rs`** : `get_by_id` prend `MaybeAuthUser` et transmet `caller.map(|user| user.user_id)`. Annotation utoipa : ajouter la réponse 401 et la sécurité optionnelle du bearer (syntaxe à vérifier via Context7).
9. **Contrat OpenAPI** : `cargo run --bin gen_openapi` pour régénérer `frontend/docs-frontend/openapi.json`.
10. **`docs/TECHNICAL_SPEC_MVP_v1.2.md`** (D1) : DDL de `listings`, tableau des endpoints (auth « optionnelle » sur `GET /listings/:id`), exemple de réponse du détail avec `published_at`, règle de visibilité publique.

### 2.2 MH-54-BE — Tests unitaires (logique pure, sans DB)

| Cible                | Cas                                                                                                                                                                                                                                                                    |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `is_visible_to`    | Table des 12 combinaisons : appelant (anonyme, propriétaire, autre compte) × état (brouillon, publié) × photos (aucune, au moins une). Seuls « propriétaire × tout » et « non-propriétaire × publié × photo » sont visibles.                            |
| Extracteur optionnel | Pas d'en-tête →`None`. En-tête présent mais invalide (préfixe autre que `Bearer`, jeton vide, jeton non décodable) → 401. Le cas « jeton non décodable » passe par `resolve_identity`, qui échoue au décodage avant de toucher au cache ou à la DB. |
| 404 identique        | Pas de test dédié : les deux chemins renvoient la même variante`AppError::ListingNotFound`, déjà couverte dans `errors.rs`.                                                                                                                                   |

Les cas qui demandent une DB (filtrage du feed, cohérence de `total`, backfill de la migration, 200 pour le propriétaire) restent pour l'epic de tests d'intégration et sont notés en mémoire comme dette.

### 2.3 MH-54-FE — Fichiers, dans l'ordre (un fichier à la fois)

1. **Préalable, `frontend/src/shared/api/types.ts`** : `npm run generate:types:ci` sur l'`openapi.json` mergé depuis le BE (ou `npm run generate:types` avec le backend lancé). `published_at` doit apparaître dans `ListingDetailDto`. Le fichier n'est jamais modifié à la main.
2. **`frontend/src/features/listings/listingVisibility.ts`** + **`listingVisibility.test.ts`**

   - `listingVisibility({ publishedAt, hasPhoto }): "draft" | "published" | "hidden"`.
   - `publishedAt === null` → `draft` (avec ou sans photo) ; `publishedAt` renseigné et photo → `published` ; renseigné sans photo → `hidden`.
   - Exporté par `features/listings/index.ts` pour MH-55.
3. **`frontend/src/features/listings/components/OwnerBar.tsx`**

   - Badge avec libellé texte (jamais la couleur seule) : Draft, Published, Hidden.
   - Messages : brouillon → « Only you can see this listing. Add at least one photo, then publish it. » ; publié → aucun message ; masqué → « This listing is published but has no photos, so it's hidden from the public. »
   - Composant simple, sans action : MH-56, MH-59 et MH-61 y ajouteront leurs boutons.
4. **`frontend/src/features/listings/components/ListingDetail.tsx`**

   - `isOwner = status === "authenticated" && profile?.id === listing.owner.id`, avec `useAuth()` et `useProfile({ enabled: status === "authenticated" })`, importés via les `index.ts` publics de `auth` et `profile`. Ni le rôle ni le jeton ne servent au contrôle.
   - La condition sur `status` évite qu'un profil resté en cache après sign-out fasse passer un visiteur pour le propriétaire.
   - `hasPhoto = listing.media.length > 0`.
   - Barre propriétaire rendue seulement si `isOwner`. Un non-propriétaire sur un bien non public reçoit un 404 et voit l'état « not found » existant, sans changement.
5. **Purge du cache au sign-out (D3)**

   - Helper unique dans `features/listings` (par exemple `removeOwnerScopedQueries(queryClient)`), qui fait `removeQueries({ queryKey: ["listing"] })`. MH-55 y ajoutera `["owner-listings"]`. `removeQueries`, pas `invalidateQueries` : une invalidation laisserait la vue propriétaire en cache affichée jusqu'à la fin du refetch.
   - Appelé dans `handleSignOut` de `app/layout/RootLayout.tsx` et `app/layout/AdminLayout.tsx`. `useDeleteAccount` fait déjà `queryClient.clear()`, rien à ajouter.
6. **`frontend/src/features/listings/components/CreateListingForm.tsx`**

   - Sous-titre : le bien est enregistré en brouillon ; photos et publication se font depuis sa page (plus de « on the next screen »).
   - Retirer le commentaire « MH-58 will replace this target… ».
   - Redirection vers `/listings/:id` inchangée.
7. **Expiration de session : vérification, pas de code a priori.** Séquence attendue avec l'existant : requête avec jeton expiré → 401 → `handleUnauthorized` → refresh raté → `clearSession()` → pas de rejeu dans `sendWithRetry` → `ApiError 401` → le `retry` de `useListing` (jusqu'à 3, 404 exclu) relance sans jeton → 200 public. On n'ajoute un traitement explicite que si la vérification manuelle montre le contraire.

### 2.4 MH-54-FE — Tests unitaires (vitest, logique pure)

| Cible                 | Cas                                                                                 |
| --------------------- | ----------------------------------------------------------------------------------- |
| `listingVisibility` | Les 4 combinaisons`publishedAt` (null, renseigné) × `hasPhoto` (false, true). |

Pas de test de composant : le projet n'utilise pas encore React Testing Library. La barre propriétaire, la purge au sign-out et l'expiration de session sont vérifiées manuellement et notées comme dette.

---

## 3. Ordre d'implémentation recommandé

**MH-54-BE**

1. Migration, puis seed. Appliquer la migration en local pour que les macros `query!` compilent.
2. Modèle et repository : prédicat partagé, `JOIN`, colonnes du détail. `cargo sqlx prepare`.
3. Extracteur `MaybeAuthUser` et ses tests.
4. Service : `is_visible_to` et ses 12 cas, puis `get_listing_detail` avec l'appelant, puis `create_listing`.
5. DTO, handler, annotations utoipa.
6. `gen_openapi`, puis mise à jour de TECHNICAL_SPEC.
7. Vérifications (section 5), contrat qualité (section 7).

**MH-54-FE** (après le merge du BE dans `develop`)

1. Régénération des types.
2. `listingVisibility` et ses tests.
3. `OwnerBar`, puis intégration dans `ListingDetail`.
4. Helper de purge et appels dans les deux layouts.
5. Texte du formulaire de création.
6. Vérifications manuelles (section 5), contrat qualité (section 7).

---

## 4. Points d'intégration avec l'existant

**Backend**

- `shared/extractors.rs` : `MaybeAuthUser` réutilise `bearer_token`, `resolve_identity` et `AuthState` sans les dupliquer. `AuthUser` reste inchangé pour les routes protégées.
- `shared/errors.rs` : aucune nouvelle variante. `Unauthorized` et `AccountSuspended` (401) couvrent les rejets ; `ListingNotFound` (404, `LISTING_NOT_FOUND`) couvre « masqué » et « inexistant ».
- `modules/listings/service.rs` : `get_listing_detail` a un seul autre appelant, `create_listing`. Aucun autre module n'importe le repository ou le service des listings.
- `modules/media/` : la règle « première photo = couverture » et « couverture supprimable seulement en dernier » (`delete_decision`) rend l'équivalence photo ⇔ couverture sûre. Le module média n'est pas modifié.
- `migrations/` : trigger `fn_set_updated_at` sur `listings` (voir R3).
- `seed/seed_listings.sql` : les 6 biens ont une couverture, le feed de dev reste rempli.
- `bin/gen_openapi.rs` → `frontend/docs-frontend/openapi.json` : contrat consommé par le FE.

**Frontend**

- `shared/api/client.ts` : envoie déjà le bearer quand un jeton existe et gère le cycle 401 → refresh. Aucun changement.
- `features/listings/hooks/useListings.ts` : clé `["listing", id]`, politique de retry (404 exclu, 3 essais) sur laquelle repose le scénario d'expiration de session. Aucun changement.
- `features/auth` (`useAuth`, `status`) et `features/profile` (`useProfile`, clé `["profile", "me"]`, `UserDto.id`) : lus via leurs `index.ts` publics.
- `app/layout/RootLayout.tsx`, `app/layout/AdminLayout.tsx` : points d'appel de la purge au sign-out.
- `features/listings/components/ListingDetail.tsx` : l'état 404 existant sert aussi aux biens non publics.
- `shared/components` : `Badge`, `Alert` pour la barre propriétaire (variantes à vérifier au moment de l'implémentation).

---

## 5. Vérifications

**BE** : `cargo fmt`, `cargo check`, `cargo clippy`. `cargo test` seulement avec l'accord explicite de l'utilisateur, ciblé sur les modules touchés (`cargo test --lib listings extractors`). Dans Swagger (`/api/docs`) : `published_at` sur `ListingDetailDto`, réponse 401 sur `GET /listings/{id}`. Contrôle manuel avec `curl` :

- créer un bien : 201 avec `"published_at": null` ;
- absent du feed et de `?owner_id=<owner>` ; `total` cohérent ;
- détail en anonyme et avec un autre compte : 404, corps identique à celui d'un UUID inconnu (`diff` des deux réponses) ;
- détail avec le jeton du propriétaire : 200 ;
- détail avec `Authorization: Bearer abc` ou `Authorization: Basic x` : 401 ;
- biens du seed toujours visibles après migration et re-seed.

**FE** : `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Vérification manuelle avec le skill `/run` :

- owner après création : page de détail avec la barre « Draft » et son message ;
- autre compte et anonyme sur la même URL : état « not found » ;
- bien public vu par son owner : badge « Published », sans message ;
- sign-out puis bouton retour sur l'URL du brouillon : « not found », pas de vue propriétaire en cache ;
- session entièrement expirée (cookie de refresh supprimé, jeton expiré) sur un bien public : vue publique, pas d'écran d'erreur ;
- formulaire de création : nouveau sous-titre, redirection inchangée.

---

## 6. Risques et inconnues

| #  | Risque / inconnue                                                                                                                                                      | Traitement                                                                                                                                                                                                                                                                 |
| -- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R1 | Tant que MH-61 n'est pas livré, aucun bien créé via l'UI ne peut devenir public. Le parcours de l'epic (créer, voir dans le feed) est cassé entre MH-54 et MH-61. | Accepté. Le seed reste public. Éviter de déployer MH-54 seul en staging sans le savoir.                                                                                                                                                                                 |
| R2 | Biens existants sans photo : publiés par la migration, mais masqués (D2).                                                                                            | Accepté. À signaler dans la PR.                                                                                                                                                                                                                                          |
| R3 | `UPDATE listings SET published_at = created_at` déclenche `fn_set_updated_at` et remplace `updated_at` par `NOW()` sur toutes les lignes.                     | Sans impact fonctionnel connu (`updated_at` n'est lu nulle part dans les listings). Si on veut le garder intact, désactiver le trigger le temps de l'`UPDATE` dans la migration. À trancher à l'implémentation, après vérification de l'usage de `updated_at`. |
| R4 | EP-10 (recherche) et EP-11 (révélation du contact) devront réutiliser le prédicat. L'invariant du monolithe interdit d'importer le repository d'un autre module.   | `push_public_visibility` est `pub` dans le repository listings. Si la recherche vit dans un autre module, exposer la règle via un trait dans `shared/` au moment d'EP-10, pas maintenant.                                                                           |
| R5 | Pas d'index sur`published_at` ; le `EXISTS` sur la couverture s'appuie sur `idx_listing_media_one_cover` (unique partiel).                                      | Suffisant au volume MVP. À réévaluer avec le point R-07 (index sur le prix) si le feed ralentit.                                                                                                                                                                        |
| R6 | Les fichiers photo d'un brouillon sont servis par nginx sous`/media/` sans contrôle d'accès.                                                                       | Accepté par la décision « le backend ne proxifie jamais les lectures publiques » ; les clés sont des UUID générés côté serveur.                                                                                                                                  |
| R7 | Rôle non revalidé à chaque requête : un owner fraîchement approuvé garde un jeton`seeker`.                                                                     | Sans effet ici : le contrôle d'accès au détail compare`user_id` et `owner_id`, pas le rôle.                                                                                                                                                                        |
| R8 | Le scénario d'expiration de session dépend du délai de retry par défaut de TanStack Query (environ 1 s avant le premier retry).                                    | Vérifier manuellement. N'ajouter un traitement explicite que si l'enchaînement échoue.                                                                                                                                                                                  |
| R9 | La sécurité optionnelle dans utoipa (`security((), ("bearer" = []))`) : syntaxe exacte selon la version.                                                           | Vérifier via Context7 avant d'écrire l'annotation.                                                                                                                                                                                                                       |

---

## 7. Contrat qualité (à appliquer sur chaque branche, BE puis FE)

1. **Orientation** : `graphify query`, `graphify path` ou `graphify explain` avant tout grep ou lecture de fichier. Relire ce plan et le ticket GitHub.
2. **Context7** avant d'écrire le code qui utilise une librairie, pour les bonnes pratiques, la sécurité et l'idiome de la version installée :
   - BE : implémentation de `FromRequestParts` pour un extracteur optionnel dans axum 0.7 (et raison de ne pas utiliser `Option<T>`) ; nullabilité inférée par `query_as!` de sqlx 0.8 pour `to_char(NULL)` et `EXISTS`, surcharge `"col!"` ; `QueryBuilder::push` ; `security` optionnelle et `Option<String>` nullable dans utoipa 5 ; `sqlx migrate add`.
   - FE : `queryClient.removeQueries` et comportement de `retry` dans TanStack Query v5 ; `useQuery` avec `enabled: false` et données en cache.
3. **Code minimal, KISS** : pas d'abstraction qui ne serve pas ce ticket. Une fonction pure pour la décision, un prédicat SQL partagé, un extracteur, un helper de visibilité, un composant de barre.
4. **Implémentation fichier par fichier**, dans l'ordre des sections 2.1 et 2.3, en montrant chaque fichier avant de passer au suivant. Règles actives :
   - BE : `.claude/rules/rust.md`, `.claude/rules/database.md`, `.claude/rules/general-coding.md`.
   - FE : `.claude/rules/react-typecrypt.md`, `.claude/rules/general-coding.md`.
   - Commentaires : une ou deux lignes. 3 au plus, uniquement là où le code ne s'explique pas seul, sans citer de fichier de règles ou de doc comme justification.
5. **Tests unitaires sur la logique pure uniquement** (sections 2.2 et 2.4). Aucune dépendance à la DB, aucun mock de la DB.
6. **Vérifications** de la section 5.
7. **Première review, avant tout fix** :
   - BE : skill `code-review-backend` ; FE : skill `code-review-frontend` ;
   - reproduire chaque finding (debuggage) avant de le corriger, et comparer avec `HEAD`, car d'autres processus peuvent modifier l'arbre de travail ;
   - **appliquer les corrections confirmées avant de continuer**, puis relancer l'étape 6.
8. **Passe `humanizer:humanizer`** sur les commentaires, doc comments, messages d'erreur et textes d'UI ajoutés : ton humain, neutre, concis, fidèle au code.
9. **`graphify update .`** pour garder le graphe à jour.
10. **Mémoire** : noter la dette de tests (cas qui demandent une DB côté BE, absence de test de composant côté FE).
11. **Arrêt** : pas de commit ni de push. L'utilisateur relit, commit et ouvre la PR.

---

## 8. Hors périmètre

Action « Publish » (MH-61), écran photos et bouton « Next: add photos → » (MH-59), édition (MH-56), liste « Mes biens » et clé `["owner-listings"]` (MH-55), modération admin des biens (EP-12), recherche (EP-10), révélation du téléphone (EP-11), revalidation du rôle à chaque requête, index sur `published_at`.

---

## 9. Critères de fin

**MH-54-BE**

- [ ] Migration : `published_at` ajouté, rempli avec `created_at` pour l'existant ; seed mis à jour
- [ ] `POST /listings` : `published_at = NULL`, `"published_at": null` dans le 201
- [ ] Feed, total et filtre `owner_id` limités aux biens publics, via un prédicat unique
- [ ] Détail : 404 identique pour masqué et inexistant ; 200 pour le propriétaire dans tous les états
- [ ] Pas d'en-tête : anonyme ; en-tête invalide, expiré ou compte suspendu : 401
- [ ] `published_at` dans `ListingDetailDto` et dans le schéma OpenAPI, avec le 401 documenté
- [ ] TECHNICAL_SPEC à jour ; `openapi.json` régénéré ; `.sqlx` à jour
- [ ] Tests de 2.2 ; fmt, check et clippy propres

**MH-54-FE**

- [ ] Types régénérés avec `published_at`
- [ ] `listingVisibility` exporté et testé sur les 4 combinaisons
- [ ] Barre propriétaire visible pour le seul propriétaire (comparaison d'id), badge avec libellé texte et messages attendus
- [ ] Non-propriétaire sur un bien non public : état « not found »
- [ ] Sign-out puis retour sur un brouillon : « not found »
- [ ] Session expirée sur un bien public : vue publique
- [ ] Texte du formulaire de création corrigé, commentaire obsolète retiré, redirection inchangée
- [ ] lint, ts:check, test et prettier propres
