# MH-63 — Filtres du feed (quartier, statut, lieux insensibles aux accents) : plan d'implémentation (BE + FE)

- **Epic** : EP-09 — Listings (#129)
- **Ticket parent** : MH-63 — Feed filters: neighborhood, status, accent-insensitive places (#174)
- **Sous-tickets** : MH-63-BE (#175) → MH-63-FE (#176)
- **Branches** : MH-63-BE sur `mh-63-be-add-the-neighborhood-and-status-filters-to-get-listings` (existe en local et sur origin, ce plan y est écrit). MH-63-FE sur une nouvelle branche créée depuis `develop` après le merge du BE. Aucun commit ni push automatique.
- **Déjà livré, et utilisé ici** : MH-13 (feed, filtre ville debouncé, select type, `keepPreviousData`), MH-53/56 (`normalize_place_name` à l'écriture), MH-54 (`push_public_visibility`), MH-57 (`ListingStatus`, `AVAILABILITY_BADGE`).
- **CDC** (FEED-05, Must) : « Le feed peut être filtré par owner_id, ville, quartier, par type ». Le filtre statut vient du ticket : le feed montre par défaut les biens disponibles et indisponibles.

---

## 1. Contexte et objectif

`GET /listings` accepte aujourd'hui `owner_id`, `city`, `type`, `page`, `per_page`. Le filtre ville s'écrit `lower(l.city) = lower($1)` dans `listings/repository.rs::push_filters` :

- il ignore la casse mais pas les accents : « Yaounde » ne trouve pas « Yaoundé » ;
- il n'utilise pas l'index `idx_listings_city_normalized`, défini sur `lower(immutable_unaccent(city))` ;
- la valeur n'est pas normalisée, alors que la ville stockée l'est (trim et espaces internes réduits à un seul).

**Objectif** : deux paramètres `neighborhood` et `status` ; `city` et `neighborhood` comparés sur la valeur entière, sans casse ni accents, après la même normalisation qu'à l'enregistrement. Côté FE, la barre de filtres gagne un champ quartier debouncé et un select statut. EP-10 réutilisera ces règles : elles sont définies une seule fois ici.

### Décisions prises

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Branches | BE sur la branche actuelle. FE sur une nouvelle branche depuis `develop` après le merge du BE (types régénérés depuis le vrai `openapi.json`). |
| D2 | Normalisation des filtres (logique pure) | `place_filter(field, raw: Option<String>) -> Result<Option<String>, AppError>` dans `shared/validation.rs`, à côté de `normalize_place_name` qu'elle appelle. Vide après normalisation → `None` (filtre ignoré). `pub` pour EP-10. |
| D3 | Caractère NUL | Une valeur qui contient `\0` → `400 INVALID_QUERY_PARAM` (variante existante). Aujourd'hui Postgres rejette la valeur liée et le feed répond 500. Une valeur stockée ne contient jamais de NUL, donc un tel filtre ne peut correspondre à rien. |
| D4 | Prédicat SQL | `lower(immutable_unaccent(<colonne>)) = lower(immutable_unaccent($n))`, extrait dans `push_place_match(qb, column, value)`. Le côté colonne reprend exactement l'expression de l'index ville. `column` est une constante du code, jamais une entrée client ; la valeur passe par `push_bind`. `pub` pour EP-10, comme `push_public_visibility`. |
| D5 | `status` invalide | `status: Option<ListingStatus>` dans `ListListingsQuery`. Une valeur inconnue est rejetée par l'extracteur `Query` d'axum (400), comme `type` aujourd'hui, ce que demande le ticket. Pas d'alignement sur le motif `InvalidQueryParam` d'`owner_requests` : hors périmètre. |
| D6 | Pas de correspondance partielle | Égalité stricte sur la valeur normalisée : « Bonap » ne trouve pas « Bonapriso ». Pas de `LIKE`, pas de longueur maximale côté filtre (une valeur trop longue ne correspond simplement à rien). |
| D7 | Index | Aucune migration. Pas d'index sur `neighborhood` (acceptable au volume MVP, dit le ticket). À ajouter à côté de R-07 dans ARCHITECTURE seulement si un problème est mesuré. |
| D8 | OpenAPI | Doc comments sur les champs de `ListListingsQuery`, repris comme descriptions par `IntoParams`. Réponse 400 ajoutée à `#[utoipa::path]` du handler `list`. |
| D9 | TECHNICAL_SPEC | Deux lignes dans la table « `GET /listings` — Query params » (§4.3) et une phrase pour la règle de correspondance. |
| D10 | Debounce FE | Hook `useDebouncedValue(value, delayMs)` dans `shared/hooks/`, qui remplace l'effet inline du champ ville et sert au champ quartier. Même délai (`CITY_FILTER_DEBOUNCE_MS` renommé `TEXT_FILTER_DEBOUNCE_MS`, 400 ms). |
| D11 | Filtres FE (logique pure) | `toFeedFilters({ city, neighborhood, type, status })` dans `features/listings/feedFilters.ts` : trim, retire les valeurs vides, renvoie l'objet passé à `useListings` et à `usePagination`. `hasActiveFilters` = au moins une clé présente. Testé avec vitest. |
| D12 | Options du select statut | « All statuses » (valeur `""`, défaut), puis les libellés de `AVAILABILITY_BADGE` (« Available », « Unavailable ») pour ne pas dupliquer les textes. |

---

## 2. Découpage BE / FE et dépendances

```
MH-63-BE (#175)  ──►  merge dans develop  ──►  MH-63-FE (#176)
  place_filter (pure + tests)              │     types.ts régénéré
  ListListingsQuery + neighborhood/status  │     api.ts : neighborhood, status
  ListingFilters + push_place_match        │     useDebouncedValue
  service::list_listings                   │     toFeedFilters (pure + tests)
  handler : OpenAPI 400                    │     ListingFeed : champ + select
  spec §4.3, openapi.json                  │
```

Le FE dépend du BE pour les deux paramètres et pour les types générés depuis `openapi.json`. Le BE ne dépend d'aucun autre ticket.

### 2.1 MH-63-BE : fichiers, dans l'ordre (un fichier à la fois)

1. **`backend/src/shared/validation.rs`** : `place_filter` (D2, D3) et ses tests. Message d'erreur du type « {field} must not contain a NUL character. ».
2. **`backend/src/modules/listings/dto.rs`** : `ListListingsQuery` gagne `neighborhood: Option<String>` et `status: Option<ListingStatus>` ; doc comments sur `city`, `neighborhood`, `status` (D8).
3. **`backend/src/modules/listings/repository.rs`** : `ListingFilters` gagne `neighborhood: Option<String>` et `status: Option<ListingStatus>` ; `push_place_match` (D4) ; `push_filters` l'utilise pour `city` et `neighborhood` et ajoute `AND l.status = $n`. Mettre à jour le doc comment de `ListingFilters`. `count_listings` et `list_listings` partagent déjà `push_filters` : `total` reste aligné sans autre code.
4. **`backend/src/modules/listings/service.rs`** : `list_listings` construit les filtres avec `place_filter("city", …)?`, `place_filter("neighborhood", …)?` et `status: query.status`.
5. **`backend/src/modules/listings/handler.rs`** : `(status = 400, …)` dans `#[utoipa::path]` de `list` (valeur `type`/`status` inconnue, filtre avec NUL).
6. **`docs/TECHNICAL_SPEC_MVP_v1.2.md`** : table §4.3 et règle de correspondance (D9).
7. **Artefacts** : `cargo run --bin gen_openapi` (`frontend/docs-frontend/openapi.json`). Pas de `cargo sqlx prepare` : les requêtes du feed passent par `QueryBuilder`, hors cache `.sqlx/`.

### 2.2 MH-63-BE : tests unitaires (logique pure, sans DB)

Dans `shared/validation.rs`, module `tests` existant :

- `None` → `Ok(None)` ; `""` → `Ok(None)` ;
- espaces seulement (`" \t\u{00A0} "`) → `Ok(None)` ;
- espaces mélangés (`" Bonapriso \t\n Nord "`) → `Ok(Some("Bonapriso Nord"))` ;
- valeur avec `\0` → `Err(AppError::InvalidQueryParam(_))`.

Pas de test sur le SQL, la combinaison des filtres, les accents ou `total` : ils demandent une vraie DB (dette pour l'epic de tests d'intégration).

### 2.3 MH-63-FE : fichiers, dans l'ordre (un fichier à la fois)

1. **`frontend/src/shared/api/types.ts`** : régénéré avec `npm run generate:types:ci`, jamais à la main.
2. **`frontend/src/features/listings/api.ts`** : `ListListingsParams` gagne `neighborhood?: string` et `status?: ListingStatus` ; `listListings` les transmet.
3. **`frontend/src/shared/hooks/useDebouncedValue.ts`** (nouveau) : `useDebouncedValue<T>(value, delayMs)` (D10).
4. **`frontend/src/features/listings/feedFilters.ts`** (nouveau) et **`feedFilters.test.ts`** : `toFeedFilters` (D11).
5. **`frontend/src/features/listings/components/ListingFeed.tsx`** :
   - états `neighborhoodInput` et `status` ; `city` et `neighborhood` via `useDebouncedValue` ;
   - champ « Neighborhood » (`aria-label="Filter by neighborhood"`) et select statut (D12) dans la barre ;
   - `filters = useMemo(() => toFeedFilters(…))`, clé de `usePagination` inchangée dans son principe : tout changement de filtre revient en page 1 ;
   - « Clear filters » remet les quatre filtres à zéro ; description de l'état vide réécrite pour couvrir quatre filtres.

### 2.4 MH-63-FE : tests unitaires (vitest, logique pure)

`feedFilters.test.ts` :

- tout vide ou espaces seulement → `{}` ;
- valeurs trimées et conservées ; `type` et `status` conservés ;
- les quatre filtres combinés ;
- aucun filtre actif / un seul filtre actif (quartier, statut) pour `hasActiveFilters`.

`useDebouncedValue` et le composant : pas de test (aucun usage de React Testing Library dans le repo).

---

## 3. Ordre d'implémentation recommandé

1. **MH-63-BE**, sur la branche actuelle, dans l'ordre de 2.1 :
   1. `place_filter` et ses tests, `cargo check` ;
   2. DTO, repository, service, handler ;
   3. spec, artefacts, vérifications (section 5), contrat qualité (section 7).
2. L'utilisateur commit et merge MH-63-BE dans `develop`.
3. **MH-63-FE**, nouvelle branche depuis `develop`, dans l'ordre de 2.3 : types et `api.ts` → hook → `toFeedFilters` et tests → `ListingFeed`.
4. L'utilisateur commit, merge et ferme #174.

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement MH-63 |
| ---- | -------- | ---------------- |
| `shared/validation.rs::normalize_place_name` | Utilisée à l'écriture par `required_place_name` (city, neighborhood) | Réutilisée par `place_filter` : filtre et valeur stockée passent par la même fonction. |
| `listings/repository.rs::push_filters` | `owner_id`, `city` (`lower = lower`), `type` | `city` et `neighborhood` via `push_place_match`, + `status`. |
| `listings/repository.rs::push_public_visibility` | Règle publique partagée par count et list | Inchangée ; les nouveaux filtres s'y ajoutent en `AND`. |
| `count_listings` / `list_listings` | Partagent `push_filters` | `pagination.total` et `total_pages` comptent exactement l'ensemble filtré. |
| Migration baseline : `immutable_unaccent`, `idx_listings_city_normalized`, `idx_listings_status` | Fonction IMMUTABLE/STRICT, index ville normalisé, index statut | Utilisés tels quels. `STRICT` : un `neighborhood` NULL ne correspond à aucun filtre quartier. Pas de migration. |
| `ListingStatus` (model.rs) | `sqlx::Type` + serde `lowercase` | Désérialisé directement depuis la query string, lié directement en SQL. |
| `shared/errors.rs::InvalidQueryParam` | 400 `INVALID_QUERY_PARAM` (owner_requests) | Réutilisé pour le NUL (D3). |
| `route.rs` / `api_doc.rs` / `gen_openapi` | Collecte automatique via `routes!` | Rien à enregistrer à la main. |
| FE `ListingFeed`, `usePagination(JSON.stringify(filters))`, `useListings` (`keepPreviousData`) | Retour page 1 sur changement de filtre, grille conservée pendant le chargement | Mêmes mécanismes ; seuls les filtres changent. |
| FE `AVAILABILITY_BADGE`, `typeLabels`, `Input`, `Select`, `EmptyState` | `labels.ts`, `shared/components` | Réutilisés. |
| FE `["listings", params]` | Clé partagée feed / owner bar (invalidation) | Les nouveaux paramètres entrent dans la clé ; les invalidations existantes de `["listings"]` restent valables. |

---

## 5. Vérifications

**BE** : `cargo fmt`, `cargo check`, `cargo clippy`. `cargo test` seulement avec l'accord explicite de l'utilisateur, ciblé sur `shared::validation`. Swagger (`/api/docs`) : `neighborhood` et `status` documentés sur `GET /listings`, réponse 400. Curl (APP_ENV=development), avec des biens publiés à « Yaoundé / Bonapriso » (available) et « Douala / Akwa » (unavailable) :

- `city=yaounde`, `city=YAOUNDÉ`, `city=%20Yaoundé%20%20` → les biens de Yaoundé ;
- `neighborhood=bonapriso` → trouvé ; `neighborhood=Bonap` → `[]`, `total: 0` ;
- `status=available` / `status=unavailable` → un seul statut ; sans `status` → les deux ;
- `status=sold` → 400 (même réponse que `type=castle`) ;
- `city=%20%20` ou `neighborhood=` → filtre ignoré ;
- `city=Akwa%00` → 400 `INVALID_QUERY_PARAM` (et non plus 500) ;
- combinaisons `city` + `neighborhood` + `type` + `status` + `owner_id` : `total` et `total_pages` = taille de l'ensemble filtré (`per_page=1` pour le vérifier) ; un brouillon ou un bien sans photo n'apparaît jamais ;
- index : `EXPLAIN` de la requête ville avec `SET enable_seqscan = off` → `idx_listings_city_normalized` utilisé.

**FE** : `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Skill `run` ou Playwright headless :

- taper « yaounde » puis « bonapriso » : une seule requête par arrêt de frappe, page 1, compteur et indicateur de page à jour ;
- select statut : « All statuses » par défaut, filtrage immédiat ;
- depuis la page 2, changer un filtre → page 1 ;
- aucun résultat avec seulement un filtre quartier ou statut actif → état vide « filtres » ; « Clear filters » vide les quatre champs ;
- requête lente (throttling) : grille actuelle visible à opacité réduite, barre utilisable ;
- clavier seul et 360 px OK (la barre passe à la ligne).

---

## 6. Risques et inconnues

| #  | Risque / inconnue | Traitement |
| -- | ----------------- | ---------- |
| R1 | Changement de comportement du filtre ville (accents ignorés, valeur normalisée). | Voulu par le ticket. Le FE envoie déjà une valeur trimée ; aucun autre appelant. |
| R2 | L'index ville n'est utilisé que si l'expression côté colonne est identique. | D4 + contrôle `EXPLAIN` (section 5). Au volume dev, le planner peut préférer un seq scan : d'où `enable_seqscan = off` pour le contrôle. |
| R3 | `unaccent` ne couvre que son dictionnaire (ligatures, diacritiques latins). | Suffisant pour les noms de lieux FR/Afrique francophone. Signalé seulement. |
| R4 | Deux formats d'erreur 400 : `type`/`status` invalide → rejet texte d'axum ; NUL → JSON `INVALID_QUERY_PARAM`. | Assumé (D3, D5). Le FE n'envoie jamais de valeur invalide. Aligner tous les rejets de query sur `AppError` serait un ticket à part. |
| R5 | Pas d'index sur `neighborhood`. | Accepté (D7). À noter près de R-07 seulement si mesuré. |
| R6 | Le doc comment de `normalize_place_name` cite « MH-59 » pour les filtres. | Ne pas corriger au passage (code non lié) ; à signaler à l'utilisateur si la review le relève. |
| R7 | `QueryBuilder::push_bind` exige une valeur possédée ou de durée `'args`. | Garder le `clone()` actuel ; vérifier l'idiome sqlx 0.8 via Context7. |
| R8 | Après « Clear filters », les champs texte se vident tout de suite mais la requête part après le délai de debounce. | Comportement actuel du champ ville, conservé. |
| R9 | MCP PostgreSQL, Sequential Thinking et plugin GitHub en échec pendant l'analyse. | `gh` utilisé ; schéma lu dans les migrations. Réessayer le MCP PostgreSQL au début de MH-63-BE (règle : consulter les index avant de toucher une requête `listings`), sinon `psql` dans le conteneur. |
| R10 | Monétisation non définie. | Aucune dépendance pour ce ticket. |

---

## 7. Contrat qualité (sur chaque branche, BE puis FE)

1. **Orientation** : `graphify query` / `path` / `explain` avant tout grep ou lecture. Relire ce plan et le ticket (`gh issue view 175` ou `176`).
2. **Context7**, avant d'écrire le code qui utilise une librairie, pour vérifier bonnes pratiques, sécurité et idiome de la version installée :
   - BE : `QueryBuilder::push` / `push_bind` (sqlx 0.8), descriptions de `IntoParams` par doc comments et réponses multiples dans `#[utoipa::path]` (utoipa 5), rejet de `Query` sur un enum inconnu (axum 0.8) ;
   - FE : `useQuery` + `placeholderData: keepPreviousData` et clés de requête (TanStack Query v5) ; nettoyage du timer dans `useEffect` (React) ; `<select>` et champ de filtre accessibles (ARIA).
3. **Code minimal, KISS** :
   - BE : une fonction pure, un helper SQL, deux champs DTO, deux champs de filtre, une ligne OpenAPI ;
   - FE : deux paramètres d'API, un hook, une fonction pure, un composant modifié ;
   - rien d'autre que ce que le ticket demande (pas d'index, pas de refonte des rejets de query).
4. **Fichier par fichier**, dans l'ordre de 2.1 puis 2.3, en montrant chaque fichier avant de passer au suivant. Règles actives : `.claude/rules/rust.md`, `database.md`, `general-coding.md` (BE) ; `react-typecrypt.md`, `general-coding.md` (FE). Commentaires d'une ou deux lignes, seulement où le code ne s'explique pas seul, sans citer un fichier de règles ou de doc.
5. **Tests unitaires sur la logique pure uniquement** (2.2 et 2.4) : aucune dépendance à la DB, aucun mock de la DB.
6. **Vérifications** de la section 5.
7. **Première review, avant tout fix** :
   - review dédiée par domaine : BE avec le skill `code-review-backend`, FE avec `code-review-frontend` ;
   - reproduire chaque finding avant de le corriger, et comparer avec `HEAD` ;
   - **appliquer les corrections confirmées avant de continuer**, puis refaire l'étape 6.
8. **Passe `humanizer:humanizer`** sur commentaires, doc comments, messages d'erreur, descriptions OpenAPI, textes de l'état vide et des filtres, amendement de la spec : ton humain, neutre, concis et fidèle au code.
9. **`graphify update .`**
10. **Mémoire** : noter la dette de tests de 2.2 et 2.4.
11. **Arrêt** : ni commit ni push. L'utilisateur relit, commit, merge et ouvre la PR.

### Outils vérifiés pendant l'analyse

- **Skills du repo** (`.claude/skills/`) : `code-review-backend`, `code-review-frontend`, `graphify`, `github-ticket`, `docker`, `readme`.
- **Règles du repo** (`.claude/rules/`) : `general-coding.md`, `rust.md`, `database.md`, `react-typecrypt.md`.
- **Skills de plugins utilisés** : `humanizer:humanizer`, `run`.
- **MCP disponibles** : Context7 (`mcp__context7__*`, `mcp__claude_ai_Context7__*`), Git MCP (lecture seule), Filesystem.
- **MCP en échec pendant l'analyse** : PostgreSQL, Sequential Thinking, plugin GitHub (remplacé par la CLI `gh`).

---

## 8. Hors périmètre

Recherche plein texte (EP-10) ; liste de référence des villes et quartiers ; correspondance partielle ou floue ; index sur `neighborhood` ou `price` ; alignement des rejets de query string sur `AppError` ; filtre statut dans « My properties » ; tests d'intégration (epic dédiée).

---

## 9. Critères de fin

**MH-63-BE**

- [ ] `neighborhood` et `status` filtrent `GET /listings` ; sans `status`, les deux statuts sont renvoyés
- [ ] `city` et `neighborhood` : valeur entière, casse et accents ignorés, pas de correspondance partielle
- [ ] Valeur normalisée avant comparaison ; vide après normalisation → ignorée ; NUL → 400 `INVALID_QUERY_PARAM`
- [ ] Combinaison avec `type`, `owner_id` et la règle de visibilité ; `total` et `total_pages` exacts
- [ ] `status` invalide rejeté comme `type` invalide
- [ ] OpenAPI documente les deux paramètres et la réponse 400 ; `openapi.json` à jour ; spec §4.3
- [ ] Tests de 2.2 ; fmt, check et clippy propres

**MH-63-FE**

- [ ] Types régénérés ; `listListings` transmet `neighborhood` et `status`
- [ ] Champ quartier debouncé comme la ville ; select statut « All statuses » / « Available » / « Unavailable »
- [ ] Tout changement de filtre → page 1 ; compteur et indicateur de page reflètent l'ensemble filtré
- [ ] État vide « filtres » dès qu'un des quatre filtres est actif ; « Clear filters » les remet tous à zéro
- [ ] Grille conservée et barre utilisable pendant le chargement
- [ ] Tests de 2.4 ; lint, ts:check, test et prettier propres
