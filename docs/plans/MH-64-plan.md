# MH-64 — Recherche par mots-clés et filtres : plan d'implémentation (BE + FE)

- **Epic** : EP-10 — Search (pas d'issue d'epic)
- **Ticket parent** : MH-64 — Search listings by keywords and filters (#185)
- **Sous-tickets** : MH-64-BE (#186) → MH-64-FE (#187)
- **Branches** : MH-64-BE sur `mh-64-be-add-the-get-search-endpoint` (existe en local et sur origin, ce plan y est écrit). MH-64-FE sur une nouvelle branche créée depuis `develop` après le merge du BE (elle n'existe pas encore : la demander avant de coder). Aucun commit ni push automatique.
- **Déjà livré, et utilisé ici** : MH-13 (feed, `ListingCard`, `Pagination`, `keepPreviousData`), MH-54 (`push_public_visibility`), MH-57 (`ListingStatus`, `AVAILABILITY_BADGE`), MH-63 (`place_filter`, `push_place_match`, `useDebouncedValue`, `AppQuery`), MH-20 (trigger `search_vector`, index GIN `idx_listings_search`).
- **CDC** : SRCH-01 (full-text titre, ville, quartier, owner, label du bien), SRCH-02 (filtres combinés), SRCH-03 (filtre `owner_id`), SRCH-04 (résultats paginés triés par pertinence). SRCH-05 (réindexation au changement de nom de l'owner) est déjà couvert par `tg_users_cascade_search`.

---

## 1. Contexte et objectif

Le feed (`GET /listings`) filtre déjà par `owner_id`, `city`, `neighborhood`, `type` et `status`, avec une règle de visibilité publique partagée par le comptage et la page. Il n'existe ni recherche par mots-clés, ni filtre prix, ni page de recherche. Le module `backend/src/modules/search/` est un fichier `mod.rs` vide, non déclaré dans `modules/mod.rs`. Côté FE, `features/search/` contient trois fichiers vides et l'onglet « Search » du header est un texte inerte.

La colonne `listings.search_vector` est déjà maintenue par le trigger `fn_update_listing_search_vector` : `to_tsvector('french', unaccent(...))`, poids A pour le titre, B pour ville, quartier, nom de l'owner et libellé français du type, C pour la description.

**Objectif** :

- **BE** : `GET /api/v1/search`, public, même enveloppe et même règle de visibilité que le feed, avec `q` (syntaxe web, accents et casse ignorés, racinisation française), les filtres du feed, `type` répétable, `price_min` / `price_max`, et un tri par pertinence. `GET /listings` accepte aussi un `type` répété.
- **FE** : page `/search` avec un champ mots-clés, une sidebar de filtres (cases à cocher), tout l'état dans l'URL, mise à jour sans rechargement.

### Décisions prises

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Branches | BE sur la branche actuelle. FE sur une nouvelle branche depuis `develop` après le merge du BE, pour régénérer les types depuis le vrai `openapi.json`. |
| D2 | Emplacement de l'endpoint | Handler `search` dans `listings/handler.rs`, route dans `listings/router.rs`, tag OpenAPI `listings`. Le dossier `modules/search/` est supprimé (aucun impact de compilation : il n'est pas déclaré). |
| D3 | Un seul chemin SQL pour le feed et la recherche | `ListingFilters` gagne `keywords`, `price_min`, `price_max` ; `listing_type: Option<_>` devient `listing_types: Vec<_>`. `count_listings` et `list_listings` restent les deux seules requêtes : le feed passe `keywords: None` et aucun prix. La règle « même visibilité, mêmes filtres » du ticket parent est garantie par construction, et `total` reste aligné avec la page. |
| D4 | Extracteur de query | `AppQuery` passe d'`axum::extract::Query` à `axum_extra::extract::Query` (feature `query` d'`axum-extra`, déjà en dépendance) pour collecter un `type` répété dans un `Vec`. Le rejet reste mappé sur `AppError::InvalidQueryParam`. `AppQuery` n'est utilisé que par `GET /listings` : rien d'autre ne change de comportement. |
| D5 | `type` répété | `#[serde(rename = "type", default)] listing_types: Vec<ListingType>` dans les deux DTO. SQL : `AND l.type = ANY($n)` avec un seul bind du `Vec`. Vide → pas de filtre. Une valeur inconnue → rejet de l'extracteur → 400 `INVALID_QUERY_PARAM`. |
| D6 | Mots-clés (logique pure) | `keyword_filter(raw: Option<String>) -> Result<Option<String>, AppError>` dans `shared/validation.rs`, à côté de `place_filter` : trim ; vide → `None` ; plus de `MAX_KEYWORD_CHARS` (100) caractères (comptés en `char`, pas en octets) → 400 ; NUL → 400 (même raison que `place_filter` : Postgres rejetterait la valeur liée). |
| D7 | Prédicat full-text | `l.search_vector @@ websearch_to_tsquery('french', unaccent($n))`, via un helper `push_keyword_query(qb, keywords)` qui écrit l'expression de la tsquery. Même configuration (`french`) et même fonction (`unaccent`, pas `immutable_unaccent`) que le trigger, dans le même ordre : sinon les accents ou les racines cessent de correspondre. Pas de préfixe `:*` : mots entiers uniquement. |
| D8 | Syntaxe malformée et mots vides | `websearch_to_tsquery` ne lève jamais d'erreur : guillemet non fermé, `OR` ou `-` isolé → 200. Une requête faite uniquement de mots vides (`le de`) produit une tsquery vide : `@@` est faux, la réponse est une liste vide avec `total: 0`. Aucun code spécifique. |
| D9 | Tri par pertinence (choix validé) | Avec `q` : palier explicite, puis `created_at DESC, id DESC`. `CASE WHEN ts_filter(l.search_vector, '{a}') @@ <tsquery> THEN 3 WHEN ts_filter(l.search_vector, '{a,b}') @@ <tsquery> THEN 2 ELSE 1 END DESC`. Titre > ville / quartier / owner / type > description seule, garanti même pour plusieurs mots. Sans `q` : ordre du feed, inchangé. |
| D10 | Prix (logique pure) | `price_min`, `price_max` : `Option<i64>` dans le DTO (un décimal, un texte ou un dépassement d'`i64` est rejeté par l'extracteur → 400). `price_bounds(min, max) -> Result<(Option<i64>, Option<i64>), AppError>` dans `listings/service.rs` : négatif → 400, `min > max` → 400, bornes incluses, chacune peut venir seule. SQL : `AND l.price >= $n` / `AND l.price <= $n` (comparaison `numeric` / `int8` native). Pas d'index (R-07). |
| D11 | `owner_id` et `status` | Déjà typés (`Uuid`, `ListingStatus`) : une valeur malformée est rejetée par l'extracteur → 400 `INVALID_QUERY_PARAM`, comme aujourd'hui sur le feed. |
| D12 | OpenAPI | `SearchListingsQuery` en `IntoParams` avec doc comments ; `type` déclaré comme tableau en `style = Form, explode` ; réponses 200 et 400 dans `#[utoipa::path]`. Description 400 du handler `list` mise à jour pour le `type` répété. |
| D13 | Documentation | Aucune modification de `TECHNICAL_SPEC` ni d'`ARCHITECTURE` dans MH-64 : le ticket range §1.1, §4.3, §4.4, §5.2/§5.3 et la nouvelle ADR dans la dette MH-68. Seul `openapi.json` est régénéré. |
| D14 | « Clear all » (choix validé) | Remet à zéro le mot-clé, tous les filtres **et** `owner_id` : retour à `/search` sans paramètre, donc tous les biens publics dans l'ordre du feed. |
| D15 | Libellés de type anglais (choix validé) | « apartment » ne trouve pas un bien par son type (le trigger indexe les libellés français). Limite connue, traitée par un ticket dédié : rien dans MH-64. |
| D16 | Requêtes FE | `searchListings(params)` dans `features/search/api.ts` ; `useSearch` avec la clé `["listings", "search", params]`. Le préfixe `["listings"]` fait profiter la recherche des invalidations déjà faites par les mutations owner (création, statut, publication, suppression, photos). `placeholderData: keepPreviousData`. |
| D17 | Paramètres répétés dans `apiGet` | `buildQueryString` accepte une valeur scalaire ou un tableau ; un tableau produit une clé répétée (`append`). Les appels existants ne changent pas. |
| D18 | État de recherche dans l'URL (logique pure) | `features/search/searchCriteria.ts` : `parseSearchCriteria(URLSearchParams)` ignore un type ou un statut inconnu, un `owner_id` qui n'est pas un UUID, une page invalide (→ 1) et un `q` de plus de 100 caractères ; `toUrlSearchParams(criteria)` omet les valeurs vides et la page 1, dans un ordre canonique (types dans l'ordre de `typeLabels`) pour garder une clé de cache stable ; `toSearchRequest(criteria)` produit les paramètres d'API. Clés d'URL identiques aux paramètres d'API (`q`, `type`, `status`, `city`, `neighborhood`, `owner_id`, `page`). |
| D19 | Statut à deux cases | L'URL garde les cases cochées (`status` répétable côté URL) pour qu'un rechargement les recoche ; l'API ne reçoit `status` que si une seule case est cochée. Aucune ou deux → pas de filtre. |
| D20 | Historique | `useSearchParams` (React Router) : filtre (case, ville, quartier, « Clear all ») → `replace: true` ; envoi du mot-clé et changement de page → nouvelle entrée. Tout changement autre que la page retire `page` de l'URL (retour page 1). |
| D21 | Délai des champs texte | Même délai que le feed (400 ms). `TEXT_FILTER_DEBOUNCE_MS` sort de `ListingFeed.tsx` vers `shared/hooks/useDebouncedValue.ts` ; `ListingFeed` ne change que par cet import (comportement identique). Un champ vidé s'applique tout de suite, comme sur le feed. |
| D22 | Case à cocher partagée | `shared/components/Checkbox.tsx` : `<input type="checkbox">` natif dans un `<label>` (clic sur le libellé, clavier et lecteur d'écran gratuits), carré, stylé avec Tailwind et les tokens de `DESIGN_TOKENS.md`. Réutilisée par MH-65. |
| D23 | Sections repliables | Section « Type » et panneau « Filters » mobile : un `<button aria-expanded aria-controls>` qui montre ou cache un bloc. Panneau mobile fermé par défaut. Un seul `FilterPanel` rendu (visible en permanence à partir de `md`, conditionné par l'état d'ouverture en dessous) pour ne pas dupliquer les champs. |

---

## 2. Découpage BE / FE et dépendances

```
MH-64-BE (#186)  ──►  merge dans develop  ──►  MH-64-FE (#187)
  Cargo.toml : axum-extra "query"          │     types.ts régénéré
  AppQuery → axum_extra::Query             │     client.ts : paramètres répétés
  keyword_filter (pure + tests)            │     search/api.ts, useSearch
  DTO : type Vec, SearchListingsQuery      │     searchCriteria.ts (pure + tests)
  ListingFilters + keywords/prix/types     │     Checkbox partagée
  list_listings : tri par paliers          │     SearchBar, FilterPanel, SearchPage
  service : price_bounds (pure + tests)    │     route /search, onglet Search
  handler + route /search, OpenAPI         │
  suppression modules/search/              │
```

Le FE dépend du BE pour l'endpoint, le `type` répété et les types générés. Le BE ne dépend d'aucun autre ticket. MH-65 (filtre prix FE) s'appuiera sur `price_min` / `price_max` et sur la `Checkbox` livrés ici.

### 2.1 MH-64-BE : fichiers, dans l'ordre (un fichier à la fois)

1. **`backend/Cargo.toml`** : `axum-extra = { version = "0.9", features = ["cookie", "query"] }` (D4). `Cargo.lock` mis à jour par `cargo check` (ajoute `serde_html_form`).
2. **`backend/src/shared/extractors.rs`** : `AppQuery` s'appuie sur `axum_extra::extract::Query` ; doc comment ajusté (clés répétées). Rejet toujours mappé sur `InvalidQueryParam`.
3. **`backend/src/shared/validation.rs`** : `MAX_KEYWORD_CHARS`, `keyword_filter` (D6) et ses tests.
4. **`backend/src/modules/listings/dto.rs`** :
   - `ListListingsQuery.listing_type` → `listing_types: Vec<ListingType>` (D5), doc comment « repeatable » ;
   - nouveau `SearchListingsQuery` (`q`, `owner_id`, `city`, `neighborhood`, `type`, `status`, `price_min`, `price_max`, `page`, `per_page`), doc comments repris par `IntoParams` (D12).
5. **`backend/src/modules/listings/repository.rs`** :
   - `ListingFilters` : `listing_types: Vec<ListingType>`, `keywords: Option<String>`, `price_min` / `price_max: Option<i64>` ; doc comment mis à jour (feed **et** recherche) ;
   - `push_keyword_query` (D7) ; `push_filters` : `= ANY`, prix, `@@` ;
   - `list_listings` : `ORDER BY` par paliers si `keywords` est présent (D9), sinon ordre actuel.
6. **`backend/src/modules/listings/service.rs`** :
   - `price_bounds` (D10) et ses tests ;
   - `search_listings(pool, SearchListingsQuery)` : `keyword_filter`, `place_filter` ×2, `price_bounds`, puis les mêmes `count_listings` / `list_listings` ;
   - le pas commun « compter, paginer, mapper » de `list_listings` et `search_listings` extrait dans une fonction privée ;
   - `list_listings` passe `listing_types` au lieu de `listing_type`.
7. **`backend/src/modules/listings/handler.rs`** : handler `search` (`AppQuery<SearchListingsQuery>`, `#[utoipa::path(get, path = "/search", …)]`, réponses 200 et 400) ; description 400 de `list` ajustée.
8. **`backend/src/modules/listings/router.rs`** : `.routes(routes!(handler::search)) // public`.
9. **`backend/src/modules/search/`** : supprimé.
10. **Artefacts** : `cargo run --bin gen_openapi` (`frontend/docs-frontend/openapi.json`). Pas de `cargo sqlx prepare` : les requêtes passent par `QueryBuilder`, hors cache `.sqlx/`.

### 2.2 MH-64-BE : tests unitaires (logique pure, sans DB)

`shared/validation.rs` (module `tests` existant), `keyword_filter` :

- `None`, `""`, espaces seulement → `Ok(None)` ;
- `"  villa douala  "` → `Ok(Some("villa douala"))` ;
- exactement 100 caractères, dont multi-octets (`"é"` ×100) → accepté ; 101 → `Err(InvalidQueryParam)` ;
- valeur avec `\0` → `Err(InvalidQueryParam)`.

`listings/service.rs` (module `tests` existant), `price_bounds` :

- aucune borne ; `min` seul ; `max` seul ; `min == max` → acceptés tels quels ;
- `min` ou `max` négatif → `Err(InvalidQueryParam)` ;
- `min > max` → `Err(InvalidQueryParam)`.

Pas de test sur le SQL (accents, racines, paliers, mots vides, combinaison, `total`) ni sur l'extracteur : ils demandent une vraie DB ou un serveur (dette pour l'epic de tests d'intégration).

### 2.3 MH-64-FE : fichiers, dans l'ordre (un fichier à la fois)

1. **`frontend/src/shared/api/types.ts`** : régénéré avec `npm run generate:types:ci`, jamais à la main.
2. **`frontend/src/shared/api/client.ts`** : `buildQueryString` accepte les tableaux (D17), exporté pour le test ; `client.test.ts` (nouveau).
3. **`frontend/src/shared/hooks/useDebouncedValue.ts`** : export de `TEXT_FILTER_DEBOUNCE_MS` ; **`ListingFeed.tsx`** l'importe au lieu de sa constante locale (D21).
4. **`frontend/src/shared/components/Checkbox.tsx`** (nouveau) et export dans `shared/components/index.ts` (D22).
5. **`frontend/src/features/search/api.ts`** : `SearchParams` (`q`, `ownerId`, `city`, `neighborhood`, `types`, `status`, `page`, `perPage`) et `searchListings` vers `/api/v1/search` (types `ListingType`, `ListingStatus`, `ListListingsResult` importés depuis `features/listings/index.ts`). Pas de prix : MH-65.
6. **`frontend/src/features/search/hooks/useSearch.ts`** : `useQuery` avec la clé de D16.
7. **`frontend/src/features/search/searchCriteria.ts`** (nouveau) et **`searchCriteria.test.ts`** (D18, D19).
8. **`frontend/src/features/search/components/SearchBar.tsx`** (nouveau) : formulaire, champ (`maxLength` 100, `aria-label`), bouton ; brouillon local resynchronisé quand `q` change dans l'URL (retour arrière) ; envoi sur Entrée ou bouton seulement.
9. **`frontend/src/features/search/components/FilterPanel.tsx`** (nouveau) : section « Type » repliable (D23) avec six `Checkbox` (`typeLabels`), deux `Checkbox` statut (`AVAILABILITY_BADGE`), champs ville et quartier debouncés (D21) resynchronisés depuis l'URL, bouton « Clear all ».
10. **`frontend/src/features/search/components/SearchPage.tsx`** (nouveau) : lit et écrit l'URL (D20), mise en page sidebar à partir de `md` / panneau « Filters » en dessous (D23), compteur « N properties », grille de `ListingCard`, `Pagination`, indicateur de chargement et opacité réduite pendant `isPlaceholderData`, squelettes au premier chargement, état vide avec « Clear all » si des critères sont actifs, `Alert` d'erreur avec un bouton « Try again » (`refetch`).
11. **`frontend/src/features/search/index.ts`** : exports publics (`SearchPage`, `useSearch`, `searchListings`).
12. **`frontend/src/app/router.tsx`** : `SearchPage` en `lazy`, `{ path: "search", element: withSuspense(SearchPage) }` sous `RootLayout`.
13. **`frontend/src/shared/components/SiteHeader.tsx`** : `PUBLIC_LINKS` devient `publicLinks(pathname)` (onglet « Search » → `/search`, `current` calculé ; « Listings » courant seulement sur `/`), utilisé par `PublicHeader` et `SeekerHeader` ; `ownerLinks` gagne l'onglet « Search ».

### 2.4 MH-64-FE : tests unitaires (vitest, logique pure)

`client.test.ts`, `buildQueryString` :

- paramètres absents ou tous `undefined` → `""` ;
- scalaires inchangés ; tableau → clé répétée dans l'ordre ; tableau vide → clé absente.

`searchCriteria.test.ts` :

- URL vide → critères par défaut ; aller-retour `parse(toUrl(c))` stable ;
- type ou statut inconnu, `owner_id` non UUID, `page=0` / `abc`, `q` de 101 caractères → ignorés ;
- types dédoublonnés et remis dans l'ordre canonique ; page 1 et valeurs vides absentes de l'URL ;
- `toSearchRequest` : statut envoyé seulement avec une case cochée ; aucune ou deux → absent ; textes trimés, vides retirés ; `owner_id` transmis.

`Checkbox`, `SearchBar`, `FilterPanel`, `SearchPage` : pas de test (aucun usage de React Testing Library dans le repo).

---

## 3. Ordre d'implémentation recommandé

1. **MH-64-BE**, sur la branche actuelle, dans l'ordre de 2.1 :
   1. MCP PostgreSQL (ou `psql` dans le conteneur) : relire les index de `listings` avant de toucher les requêtes ;
   2. `Cargo.toml` + `AppQuery`, `cargo check`, puis curl sur `GET /listings` pour confirmer que le feed répond comme avant ;
   3. `keyword_filter` et ses tests ;
   4. DTO, repository, service (`price_bounds` et tests), handler, router, suppression de `modules/search/` ;
   5. artefacts, vérifications (section 5), contrat qualité (section 7).
2. L'utilisateur commit et merge MH-64-BE dans `develop`.
3. **MH-64-FE**, nouvelle branche depuis `develop`, dans l'ordre de 2.3 : types et `client.ts` → constante de délai → `Checkbox` → `api.ts` / `useSearch` → `searchCriteria` et tests → `SearchBar` → `FilterPanel` → `SearchPage` → route → header.
4. L'utilisateur commit, merge et ferme #185.

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement MH-64 |
| ---- | -------- | ---------------- |
| `shared/extractors.rs::AppQuery` | Enveloppe `axum::Query`, rejet → `INVALID_QUERY_PARAM` ; seul usage : `GET /listings` | Enveloppe `axum_extra::Query` (clés répétées). Utilisé aussi par `GET /search`. |
| `shared/validation.rs::place_filter` | Normalise ville et quartier, NUL → 400 | Réutilisée telle quelle par la recherche ; `keyword_filter` est ajoutée à côté, sur le même modèle. |
| `listings/repository.rs::push_public_visibility`, `push_place_match` | Règle publique et correspondance de lieu du feed | Inchangées, partagées par la recherche via `push_filters`. |
| `listings/repository.rs::push_filters`, `count_listings`, `list_listings` | Filtres du feed, `total` aligné | Un `type` multiple, prix, mots-clés ; tri par paliers seulement avec mots-clés. Le feed garde son SQL effectif. |
| Trigger `fn_update_listing_search_vector`, `idx_listings_search` (GIN) | `french` + `unaccent`, poids A/B/C | Lus tels quels : la tsquery reprend la même chaîne de traitement (D7). Pas de migration. |
| `tg_users_cascade_search` | Recalcul à chaque changement de nom d'owner | Couvre SRCH-05, rien à faire. |
| `ListingType` / `ListingStatus` (`model.rs`) | `sqlx::Type` + serde `lowercase` | `Vec<ListingType>` lié en tableau (`listing_type[]`). |
| `shared/errors.rs::InvalidQueryParam`, `shared/pagination.rs` | 400 standard ; `PaginationMeta` (20 par défaut, 50 max) | Réutilisés sans changement. |
| `route.rs` / `api_doc.rs` / `gen_openapi` | Collecte via `routes!` | Rien à enregistrer à la main. |
| Rate limit global (`middleware/rate_limit.rs`) | Appliqué à toutes les routes | `/search` est couvert sans configuration. |
| FE `shared/api/client.ts::apiGet` | Paramètres scalaires | Tableaux acceptés ; appels existants inchangés. |
| FE invalidations `["listings"]` (création, statut, publication, suppression, photos) | Rafraîchissent le feed | Rafraîchissent aussi la recherche grâce à la clé `["listings", "search", …]`. |
| FE `ListingCard`, `Pagination`, `EmptyState`, `Alert`, `Skeleton`, `Spinner`, `Input`, `typeLabels`, `AVAILABILITY_BADGE` | Utilisés par le feed | Réutilisés via `features/listings/index.ts` et `shared/components`. |
| FE `ListingFeed` | État local, `Select` | Inchangé, sauf l'import de `TEXT_FILTER_DEBOUNCE_MS` (D21). |
| FE `SiteHeader` | « Listings » marqué courant en dur, « Search » inerte | Onglet actif calculé depuis le chemin ; « Search » relié pour public, seeker et owner. |

---

## 5. Vérifications

**BE** : `cargo fmt`, `cargo check`, `cargo clippy`. `cargo test` seulement avec l'accord explicite de l'utilisateur, ciblé sur `shared::validation` et `listings::service`. Swagger (`/api/docs`) : `GET /search` documenté, `type` en tableau, réponse 400. Curl (APP_ENV=development), avec des biens publiés : « Appartement … » à « Yaoundé / Bonapriso », un bien dont seul le texte de description contient « piscine », un brouillon et un bien sans photo :

- `q=yaounde` → bien de « Yaoundé » ; `q=appartements` → bien titré « Appartement » ; `q=Bonap` → `[]` ;
- `q="appartement meublé"`, `q=villa OR studio`, `q=appartement -meublé` → syntaxe appliquée ;
- `q="abc`, `q=OR`, `q=-` → 200 ; `q=le de` → 200, `[]`, `total: 0` ; `q=%20%20` → comme sans `q` ;
- `q` de 101 caractères → 400 `INVALID_QUERY_PARAM` ;
- ordre : un bien qui a le mot dans le titre avant un bien qui l'a dans la ville, avant un bien qui ne l'a que dans la description ; à palier égal, le plus récent d'abord ; `per_page=1` sur plusieurs pages → ni doublon ni trou ;
- `type=apartment&type=studio` sur `/search` **et** `/listings` ; `type=apartment` seul sur `/listings` inchangé ; `type=castle` → 400 ;
- `price_min=0`, `price_max` seul, bornes égales au prix exact → inclus ; `price_min=-1`, `price_min=12.5`, `price_min=abc`, `price_min=50&price_max=10` → 400 ;
- `owner_id=xyz`, `status=sold` → 400 ; chaque 400 dans l'enveloppe standard ;
- combinaison `q` + filtres : `total` et `total_pages` = ensemble filtré ; brouillon et bien sans photo jamais présents ;
- `EXPLAIN ANALYZE` (avec `SET enable_seqscan = off`) : `idx_listings_search` utilisé ; temps de la requête mesuré (`curl -w '%{time_total}'`) bien sous 500 ms.

**FE** : `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Skill `run` ou Playwright headless :

- `/search` sans paramètre → tous les biens publics, ordre du feed ; onglet « Search » courant (public, seeker, owner) ;
- frappe dans le champ mots-clés → aucune requête ; Entrée ou bouton → requête, nouvelle entrée d'historique, page 1 ; champ vidé puis envoyé → `q` retiré ;
- cases type et statut : application immédiate, `replace` dans l'historique ; deux statuts cochés → pas de `status` dans la requête mais cases recochées après rechargement ;
- ville et quartier : une requête par arrêt de frappe ;
- rechargement et URL partagée → mêmes résultats, mêmes cases ; URL avec `type=castle`, `status=sold`, `owner_id=xyz` → ignorés sans écran d'erreur ; `owner_id` valide conservé et envoyé ;
- ouvrir une annonce puis revenir → mêmes résultats, filtres et page ;
- « Clear all » (sidebar et état vide) → `/search` nu, `owner_id` compris ;
- requête lente (throttling) : résultats précédents visibles, indicateur de chargement, pas de saut de page ; erreur réseau → message et « Try again » ;
- clavier seul : cases, libellés cliquables, section « Type » et panneau « Filters » annoncent leur état ouvert ou fermé ; 360 px (panneau « Filters ») et ≥ 768 px (sidebar).

---

## 6. Risques et inconnues

| #  | Risque / inconnue | Traitement |
| -- | ----------------- | ---------- |
| R1 | Lier un `Vec<ListingType>` exige `PgHasArrayType` pour l'enum. | Vérifier via Context7 que le derive `sqlx::Type` (sqlx 0.8) le fournit pour un enum avec `type_name` ; sinon `#[sqlx(...)]` ou un bind par élément avec `separated`. |
| R2 | `serde_html_form` (sous `axum_extra::Query`) ne traite pas toujours une valeur vide comme `serde_urlencoded` (`page=`, `type=`). | Seul `GET /listings` est concerné. Curl avant/après sur ces cas ; un écart reste acceptable s'il ne produit pas de 500 (le FE n'envoie jamais de valeur vide). |
| R3 | API du rejet d'`axum_extra::extract::Query` en 0.9.6 (`body_text()` disponible ?). | Context7 ; sinon `to_string()`. Le code d'erreur reste `INVALID_QUERY_PARAM`. |
| R4 | La tsquery doit suivre exactement la chaîne du trigger (`unaccent` puis `french`). | D7 ; cas « yaounde » et « appartements » vérifiés en curl. Un changement futur du trigger devra modifier les deux côtés. |
| R5 | `q` fait de mots vides ou de syntaxe seule (`-`, `OR`) renvoie 0 résultat, et Postgres journalise un `NOTICE`. | Conforme au ticket (seul le 200 est exigé). Signalé seulement. |
| R6 | Le tri par paliers évalue `ts_filter` sur toutes les lignes trouvées avant `LIMIT`. | Négligeable au volume MVP ; mesure de la section 5. À revoir avec R-02 si le volume grandit. |
| R7 | Pas d'index sur `price` (R-07). | Ne pas en ajouter (ticket). Signalé si un plan d'exécution lent apparaît. |
| R8 | Mots-clés anglais sur le type (« apartment »). | Limite connue, ticket dédié (D15). |
| R9 | FE : les champs ville, quartier et mots-clés ont un état local qui doit suivre l'URL au retour arrière. | Resynchronisation pendant le rendu quand la valeur d'URL change (même motif que `usePagination`) ; vérifié par Playwright (retour arrière après filtre). |
| R10 | Position de défilement non restaurée au retour depuis une annonce (aucun `ScrollRestoration` dans l'app). | Hors critères du ticket (résultats, filtres, page). Signalé seulement. |
| R11 | Une URL forgée avec un NUL dans `city` ou `neighborhood` renvoie un 400 et affiche l'erreur. | Accepté : seuls type, statut et owner sont listés comme valeurs à ignorer ; l'erreur garde un « Try again ». |
| R12 | `@tanstack/react-query` 5, `react-router` 8 : signatures de `useSearchParams` / `setSearchParams(…, { replace })`. | Context7 avant d'écrire `SearchPage`. |
| R13 | MCP PostgreSQL, Sequential Thinking et plugin GitHub en échec pendant l'analyse. | `gh` utilisé ; schéma lu dans les migrations. Réessayer le MCP PostgreSQL au début de MH-64-BE, sinon `psql` dans le conteneur. |
| R14 | Monétisation non définie. | Aucune dépendance pour ce ticket. |

---

## 7. Contrat qualité (sur chaque branche, BE puis FE)

1. **Orientation** : `graphify query` / `path` / `explain` avant tout grep ou lecture. Relire ce plan et le ticket (`gh issue view 186` ou `187`).
2. **Context7**, avant d'écrire le code qui utilise une librairie, pour vérifier bonnes pratiques, sécurité et idiome de la version installée :
   - BE : `axum_extra::extract::Query` et son rejet (axum-extra 0.9), `serde_html_form` et `Vec` + `#[serde(default)]` ; `QueryBuilder::push_bind` d'un `Vec` d'enum et `PgHasArrayType` (sqlx 0.8) ; `websearch_to_tsquery`, `ts_filter`, `unaccent` (doc PostgreSQL) ; paramètres tableau `style = Form, explode` dans `IntoParams` (utoipa 5) ;
   - FE : `useSearchParams` / `setSearchParams` avec `replace` (React Router 8) ; `useQuery` + `keepPreviousData` et clés hiérarchiques (TanStack Query 5) ; `URLSearchParams.append` ; motif disclosure (`aria-expanded`, `aria-controls`) et case à cocher native accessible.
3. **Code minimal, KISS** :
   - BE : un chemin SQL partagé (pas de requête de recherche séparée), deux fonctions pures, un helper SQL, un DTO, un handler ;
   - FE : une fonction pure de critères, un composant partagé, trois composants de feature, aucune librairie ajoutée ;
   - rien d'autre que ce que le ticket demande (pas d'index, pas de migration, pas de doc spec : MH-68 ; pas de filtre prix FE : MH-65).
4. **Fichier par fichier**, dans l'ordre de 2.1 puis 2.3, en montrant chaque fichier avant de passer au suivant. Règles actives : `.claude/rules/rust.md`, `database.md`, `general-coding.md` (BE) ; `react-typecrypt.md`, `general-coding.md` (FE). Commentaires d'une ou deux lignes, seulement où le code ne s'explique pas seul, sans citer un fichier de règles ou de doc.
5. **Tests unitaires sur la logique pure uniquement** (2.2 et 2.4) : aucune dépendance à la DB, aucun mock de la DB.
6. **Vérifications** de la section 5.
7. **Première review, avant tout fix** :
   - review dédiée par domaine : BE avec le skill `code-review-backend`, FE avec `code-review-frontend` ;
   - reproduire chaque finding avant de le corriger, et comparer avec `HEAD` (d'autres processus peuvent modifier l'arbre de travail) ;
   - **appliquer les corrections confirmées avant de continuer**, puis refaire l'étape 6.
8. **Passe `humanizer:humanizer`** sur commentaires, doc comments, messages d'erreur, descriptions OpenAPI, libellés et textes de la page (état vide, erreur, filtres) : ton humain, neutre, concis et fidèle au code.
9. **`graphify update .`**
10. **Mémoire** : noter la dette de tests de 2.2 et 2.4 (cas SQL, extracteur, composants).
11. **Arrêt** : ni commit ni push. L'utilisateur relit, commit, merge et ouvre la PR.

### Outils vérifiés pendant l'analyse

- **Skills du repo** (`.claude/skills/`) : `code-review-backend`, `code-review-frontend`, `graphify`, `github-ticket`, `docker`, `readme`.
- **Règles du repo** (`.claude/rules/`) : `general-coding.md`, `rust.md`, `database.md`, `react-typecrypt.md`.
- **Skills de plugins utilisés** : `humanizer:humanizer`, `run`.
- **MCP disponibles** : Context7 (`mcp__context7__*`, `mcp__claude_ai_Context7__*`), Git MCP (lecture seule), Filesystem.
- **MCP en échec pendant l'analyse** : PostgreSQL, Sequential Thinking, plugin GitHub (remplacé par la CLI `gh`).

---

## 8. Hors périmètre

Filtre prix côté FE (MH-65) ; libellés de type anglais dans le `search_vector` (ticket dédié) ; mises à jour de TECHNICAL_SPEC et d'ARCHITECTURE, nouvelle ADR (MH-68) ; index sur `price` ; recherche partielle ou floue ; refonte de `ListingFeed` ; restauration du défilement ; tests d'intégration (epic dédiée).

---

## 9. Critères de fin

**MH-64-BE**

- [ ] `GET /api/v1/search` public, même enveloppe et même règle de visibilité que le feed, `total` aligné
- [ ] `q` : accents et casse ignorés, racinisation française, mots entiers, syntaxe web ; syntaxe malformée → 200 ; vide → absent ; > 100 caractères → 400 ; mots vides seuls → `[]`, `total: 0`
- [ ] Tri : paliers titre > ville / quartier / owner / type > description, puis `created_at DESC`, `id` ; sans `q`, ordre du feed
- [ ] `type` répétable sur `/search` et `/listings` ; `type` seul inchangé ; valeur inconnue → 400
- [ ] `price_min` / `price_max` : entiers ≥ 0, bornes incluses, seuls ou ensemble ; invalide ou `min > max` → 400
- [ ] `owner_id` malformé, `status` inconnu → 400 ; tous les 400 dans l'enveloppe standard ; filtres combinés en `AND`
- [ ] OpenAPI documente l'endpoint, le `type` répété et la réponse 400 ; `openapi.json` à jour
- [ ] `backend/src/modules/search/` supprimé
- [ ] Tests de 2.2 ; fmt, check et clippy propres

**MH-64-FE**

- [ ] `/search` public et chargé en `lazy` ; onglet « Search » courant pour public, seeker et owner
- [ ] Sidebar à partir de la tablette, panneau « Filters » repliable en dessous
- [ ] Mot-clé envoyé seulement sur Entrée ou bouton ; vide → retiré
- [ ] Six types et deux statuts en cases carrées partagées ; ville et quartier au délai du feed ; cases appliquées tout de suite
- [ ] Tout l'état dans l'URL ; valeurs invalides ignorées ; `owner_id` conservé et envoyé ; `replace` pour les filtres, nouvelle entrée pour mot-clé et page ; tout changement → page 1
- [ ] Retour depuis une annonce : mêmes résultats, filtres et page
- [ ] Compteur, cartes et pagination du feed ; résultats précédents visibles pendant le chargement ; état vide avec « Clear all » ; erreur avec « Try again »
- [ ] Clavier et lecteur d'écran : cases, libellés, sections repliables
- [ ] Tests de 2.4 ; lint, ts:check, test et prettier propres
