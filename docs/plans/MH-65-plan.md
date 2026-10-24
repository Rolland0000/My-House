# MH-65 — Filtre prix à deux curseurs dans la recherche : plan d'implémentation (FE)

- **Epic** : EP-10 — Search (pas d'issue d'epic). Le ticket indique EP-10, pas EP-09.
- **Ticket** : MH-65 — Filter search results with a price range slider (#188). FE uniquement, pas de sous-ticket.
- **Branche** : `mh-65-filter-search-results-with-a-price-range-slider` (en local et sur origin, créée depuis `develop` après le merge de MH-64-FE). Aucun commit ni push automatique.
- **Déjà livré, et utilisé ici** : MH-64-BE (`price_min` / `price_max` sur `GET /api/v1/search`, déjà dans `types.ts`), MH-64-FE (`SearchPage`, `FilterPanel`, `searchCriteria.ts`, `useUrlDraft`, `draftResetKey`), MH-53 (`PRICE_MAX`), `formatPrice` / `PRICE_CURRENCY` (`shared/utils/format.ts`).
- **CDC** : SRCH-02 (filtres combinés : ville, type, prix min/max).

---

## 1. Contexte et objectif

La page `/search` (MH-64-FE) filtre par mots-clés, type, statut, ville et quartier. Tout l'état est dans l'URL. Le backend accepte déjà `price_min` et `price_max` : entiers ≥ 0, bornes incluses, `min > max` → 400, valeur non entière → 400. Le FE ne les envoie pas encore.

**Objectif** : une section « Monthly rent (XAF) » dans la sidebar, avec :

- un slider à deux poignées de 0 à 1 000 000 XAF, par pas de 5 000 ;
- deux champs « Min » / « Max » synchronisés avec les poignées ;
- les bornes dans l'URL ;
- une recherche lancée au relâchement d'une poignée, à la sortie d'un champ ou à son envoi (Entrée), jamais pendant un glissement.

Pas de librairie de slider.

### Décisions prises

| #   | Sujet | Décision |
| --- | ----- | -------- |
| D1  | Bornes du slider | Une seule constante `PRICE_SLIDER = { min: 0, max: 1_000_000, step: 5_000, pageStep: 50_000 }` dans `features/search/priceRange.ts`. C'est le seul endroit à modifier quand il y aura de vraies données de prix. |
| D2  | État de la recherche | `SearchCriteria` gagne `priceMin: number \| null` et `priceMax: number \| null`. `null` = pas de borne. `EMPTY_CRITERIA` les met à `null`. Clés d'URL identiques aux paramètres d'API (`price_min`, `price_max`). |
| D3  | Bornes « absentes » (choix validé) | Min à 0 → `priceMin: null`. Max égal à 1 000 000, butée haute comprise → `priceMax: null`, libellé « 1 000 000+ XAF ». Un max saisi **au-dessus** de 1 000 000 met la poignée en butée et reste envoyé. La même règle s'applique au champ, à la poignée et à l'URL, donc un rechargement redonne le même état. |
| D4  | Plafond (choix validé) | `PRICE_MAX` des annonces (9 999 999 999), réexporté par `features/listings/index.ts`. Une saisie au-dessus est ramenée au plafond. Dans l'URL, une valeur au-dessus est ignorée comme une valeur invalide. Aucune valeur ne peut dépasser l'`i64` du backend ni la précision des nombres JS. |
| D5  | Lecture de l'URL | `price_min` / `price_max` sont acceptés seulement s'ils sont faits uniquement de chiffres (`/^\d+$/`) et ≤ `PRICE_MAX`. Le reste est ignoré : texte, négatif, décimal, vide. Si les deux sont valides et `min > max`, les deux sont ignorés. `price_min=0` et `price_max=1000000` sont lus comme absents (D3). |
| D6  | Champs (choix validé) | `<input type="number" min=0 step=1 inputMode="numeric">` avec des chiffres bruts. Le brouillon reste une chaîne pendant la frappe. Au-dessus du slider, une ligne de lecture formatée avec `formatPrice` + `PRICE_CURRENCY` affiche par exemple « 50 000 XAF – 1 000 000+ XAF ». C'est le « format existant » du ticket (fr-FR). |
| D7  | Saisie libre | Tout montant entier est accepté. La poignée se place sur le pas le plus proche (arrondi au multiple de 5 000, borné à [0, 1 000 000]) et suit la frappe en direct. Le montant exact est envoyé. |
| D8  | Sortie d'un champ (choix validé) | Min vide → 0 (pas de borne). Max vide → butée (pas de borne). Un décimal est arrondi à l'entier le plus proche. Un texte illisible reprend la valeur par défaut de sa borne. Ensuite, un min au-dessus du max devient le max, et un max sous le min devient le min. Pas de borne max = pas de croisement possible. |
| D9  | Non-croisement des poignées | Pendant un glissement ou au clavier, la valeur de chaque poignée est bornée par l'autre : `min ≤ max`, égalité permise. Pendant la frappe, l'affichage des poignées est aussi borné. La correction de la valeur saisie n'a lieu qu'à la sortie du champ (D8). |
| D10 | Moment de la recherche | Le `onChange` React d'un range suit l'événement `input`. Il ne met à jour que le brouillon (poignées, champs, lecture). La recherche part sur l'événement natif `change` (relâchement du pointeur, ou pas clavier), écouté via une `ref`. Elle part aussi sur `blur` d'un champ et sur Entrée dans un champ (`onKeyDown`) : avec deux champs, un `<form>` sans bouton d'envoi ne se soumet pas implicitement. |
| D11 | Clavier | Flèches et Home / End : comportement natif du range (un pas, butées). Page Up / Page Down : `onKeyDown` avec `preventDefault`, ±50 000 bornés par D9, puis recherche explicite (une valeur posée par le code ne déclenche pas `change`). Le pas natif de ces touches dépend du navigateur. |
| D12 | Accessibilité | Ranges avec `aria-label="Minimum rent"` / `"Maximum rent"` et `aria-valuetext` formaté (« 50 000 XAF », « 1 000 000+ XAF » en butée sans borne, sinon le montant envoyé). Section en `<fieldset>` avec la légende « Monthly rent (XAF) ». Champs avec les libellés visibles « Min » / « Max ». |
| D13 | Rendu du slider | Deux `<input type="range">` natifs superposés sur une piste. Les inputs sont transparents et en `pointer-events-none`. Les pouces (`::-webkit-slider-thumb`, `::-moz-range-thumb`) sont en `pointer-events-auto`, via les variantes arbitraires de Tailwind 4. La portion entre les poignées est remplie en `ink-900`, comme les cases cochées. Quand les poignées se chevauchent, celle du min passe au-dessus si elles sont en butée haute, pour qu'elle reste attrapable. |
| D14 | Historique et pagination | Même chemin que les autres filtres : `changeFilters` (`replace: true`, retour page 1). `FilterPatch` inclut déjà les nouveaux champs. |
| D15 | « Clear all » | `EMPTY_CRITERIA` remet les bornes à `null`. Le composant prix est placé dans le `Fragment` à `draftResetKey`, donc son brouillon non envoyé est abandonné. |
| D16 | Position dans la sidebar | Type → Monthly rent → Status → City → Neighborhood. Section non repliable (le ticket ne le demande pas). |

---

## 2. Fichiers, dans l'ordre (un fichier à la fois)

1. **`frontend/src/features/listings/index.ts`** : réexporter `PRICE_MAX` depuis `listingFormValidation.ts` (D4). Une feature n'importe pas les fichiers internes d'une autre.
2. **`frontend/src/features/search/priceRange.ts`** (nouveau) : `PRICE_SLIDER` (D1) et les fonctions pures :
   - `toHandlePosition(amount)` (D7) ;
   - `parsePriceInput(raw, bound)` (D8 : vide, décimal, illisible, plafond) ;
   - `resolveCrossing(min, max, edited)` (D8) ;
   - `shiftHandle(value, delta, limit)` (D9, D11) ;
   - `toPriceBounds(min, max)` (D3) ;
   - `formatPriceBound(amount, isTop)` (D6, D12).
3. **`frontend/src/features/search/priceRange.test.ts`** (nouveau) : section 3.
4. **`frontend/src/features/search/api.ts`** : `SearchParams` gagne `priceMin` / `priceMax`, envoyés en `price_min` / `price_max`.
5. **`frontend/src/features/search/searchCriteria.ts`** : `priceMin` / `priceMax` dans `SearchCriteria` et `EMPTY_CRITERIA` (D2). `parsePrice` et la règle « croisés → les deux ignorés » (D5). Écriture dans un ordre fixe dans `toUrlSearchParams`, transmission dans `toSearchRequest`. `hasSearchCriteria` couvre le prix sans changement.
6. **`frontend/src/features/search/searchCriteria.test.ts`** : section 3.
7. **`frontend/src/features/search/components/PriceRangeFilter.tsx`** (nouveau) :
   - props `priceMin`, `priceMax`, `onCommit({ priceMin, priceMax })` ;
   - brouillon local resynchronisé depuis l'URL (motif `useUrlDraft`) ;
   - lecture formatée, piste, deux ranges, deux champs (D6 à D13).
8. **`frontend/src/features/search/components/FilterPanel.tsx`** : insérer `PriceRangeFilter` après la section Type (D16), dans le `Fragment` à `draftResetKey` (D15), relié à `onChange`.
9. **`SearchPage.tsx`** : pas de changement attendu. `changeFilters` et `clearAll` couvrent déjà le prix (D14, D15). À confirmer à la relecture.

---

## 3. Tests unitaires (vitest, logique pure)

`priceRange.test.ts` :

- `toHandlePosition` : 0 → 0 ; 7 499 → 5 000 ; 7 500 → 10 000 ; 1 000 000 et 2 500 000 → butée haute ;
- `parsePriceInput` : `""` → borne par défaut (0 pour min, pas de borne pour max) ; `"12.5"` → 13 ; `"abc"` ou `"-5"` → borne par défaut ; au-dessus de `PRICE_MAX` → `PRICE_MAX` ;
- `resolveCrossing` : min modifié au-dessus du max → min = max ; max modifié sous le min → max = min ; max absent → inchangé ; égalité gardée ;
- `shiftHandle` : ±50 000 bornés par 0, 1 000 000 et l'autre poignée ;
- `toPriceBounds` : min 0 → `null` ; max 1 000 000 → `null` ; max 1 500 000 et max 0 → gardés ;
- `formatPriceBound` : butée sans borne → « 1 000 000+ XAF ». Comparer avec la sortie de `formatPrice`, pas avec une chaîne écrite à la main : fr-FR utilise une espace fine insécable.

`searchCriteria.test.ts` (cas ajoutés) :

- aller-retour avec les deux bornes ; `FULL_CRITERIA` complété ;
- `price_min=abc`, `-5`, `1.5`, vide, au-dessus de `PRICE_MAX` → ignorés ;
- `price_min > price_max` → les deux ignorés ; égaux → gardés ; une seule borne → gardée ;
- `price_min=0` et `price_max=1000000` → absents ; `price_max=1500000` → gardé ;
- `toUrlSearchParams` omet les `null` ; `toSearchRequest` transmet `priceMin` / `priceMax` ; `hasSearchCriteria` est vrai avec un prix seul.

`PriceRangeFilter`, `FilterPanel` : pas de test de composant (aucun usage de React Testing Library dans le repo). La dette est notée en mémoire.

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement MH-65 |
| ---- | -------- | ---------------- |
| `GET /api/v1/search` (MH-64-BE) | `price_min` / `price_max` : entiers ≥ 0, bornes incluses ; `min > max`, négatif ou décimal → 400 | Aucun changement. Le FE n'envoie que des entiers valides et ordonnés (D4, D5, D8), donc jamais de 400 venant du prix. |
| `shared/api/types.ts` | Contient déjà `price_min` / `price_max` | Pas de régénération. |
| `features/search/searchCriteria.ts` | `parse` / `toUrl` / `toSearchRequest` / `hasSearchCriteria` | Deux champs en plus, mêmes règles : une valeur invalide est ignorée, ordre fixe dans l'URL. |
| `features/search/api.ts`, `useSearch` | Clé `["listings", "search", params]`, `keepPreviousData` | Deux paramètres en plus. La clé couvre le prix sans changement. |
| `FilterPanel` / `SearchPage` | `changeFilters` (`replace`, page 1), `clearAll` + `draftResetKey`, panneau « Filters » mobile | Réutilisés tels quels. Une section en plus. |
| `useUrlDraft` | Brouillon de texte resynchronisé quand la valeur d'URL change | Même motif pour le brouillon `{ min, max }`, ou réutilisation sur une clé dérivée. |
| `shared/utils/format.ts` | `formatPrice` (fr-FR), `PRICE_CURRENCY` | Réutilisés pour la lecture et `aria-valuetext`. |
| `features/listings/listingFormValidation.ts::PRICE_MAX` | Plafond du formulaire d'annonce | Réexporté via `features/listings/index.ts`, utilisé comme plafond de recherche (D4). |
| `shared/components/Input` | Champ stylé | Utilisé pour Min / Max. |
| Tokens (`DESIGN_TOKENS.md`) | `ink-900`, `border-strong`, `focus` | Piste, remplissage, pouces et anneau de focus. Pas de nouveau token. |

---

## 5. Vérifications

`npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Ensuite le skill `run`, ou Playwright headless, avec l'onglet réseau observé :

- glissement d'une poignée → aucune requête pendant le glissement, une seule au relâchement ; champs et lecture suivent en direct ;
- flèches, Page Up / Page Down (±50 000), Home / End → valeurs attendues ; une requête par pas ; jamais de croisement ;
- `7499` saisi → poignée à 5 000, `price_min=7499` envoyé à la sortie du champ ; `1500000` en Max → poignée en butée, `price_max=1500000` envoyé ; `1000000` en Max → pas de `price_max` ;
- sortie de Min au-dessus du Max → Min ramené au Max (et l'inverse) ; champ vidé → borne retirée ; Entrée dans un champ → requête ;
- chaque changement → page 1, `replace` dans l'historique ; rechargement → poignées, champs et lecture restaurés ;
- URL avec `price_min=abc`, `-5`, `1.5`, `price_min=500000&price_max=100000` → ignorés, pas d'écran d'erreur ;
- « Clear all » (sidebar et état vide) → section remise à zéro, brouillon non envoyé compris ;
- arbre d'accessibilité : noms « Minimum rent » / « Maximum rent », valeurs annoncées en XAF ;
- 360 px en émulation tactile (`hasTouch`) dans le panneau « Filters » : glissement des deux poignées, y compris quand elles se chevauchent ; ≥ 768 px en sidebar. Si possible, contrôle manuel sur un vrai téléphone (iOS Safari, Android Chrome).

---

## 6. Risques et inconnues

| #   | Risque / inconnue | Traitement |
| --- | ----------------- | ---------- |
| R1  | React mappe `onChange` d'un range sur `input`. L'événement natif `change`, qui marque le relâchement, n'a pas de prop dédiée. | Écoute via `ref` + `addEventListener("change")`. Vérifier dans Context7 (React 19) qu'il n'existe pas d'alternative intégrée. Repli : `onPointerUp` + `onKeyUp`. |
| R2  | Pas natif de Page Up / Page Down variable selon le navigateur. | Gestion explicite (D11), avec `preventDefault`. |
| R3  | Deux ranges superposés : le pouce du dessus capte les clics quand les poignées se chevauchent. | Ordre d'empilement selon la position (D13). Cas « deux poignées en butée haute » et « deux poignées à 0 » vérifiés à la souris et au toucher. |
| R4  | Toucher sur les pouces avec une piste en `pointer-events-none` (iOS Safari). L'émulation Playwright ne remplace pas un vrai appareil. | Contrôle manuel conseillé. Ajuster `touch-action` sur les pouces si le défilement de la page vole le geste. |
| R5  | Pseudo-éléments des pouces différents entre WebKit/Blink et Firefox ; syntaxe des variantes arbitraires de Tailwind 4. | Context7 (Tailwind 4), et contrôle dans Chromium et Firefox. |
| R6  | Une flèche maintenue envoie une requête par pas. | Accepté : `keepPreviousData` garde les résultats affichés, et la dernière réponse l'emporte via la clé de cache. Signalé seulement. |
| R7  | `type="number"` accepte `e`, `+`, `-`. La molette peut changer la valeur d'un champ qui a le focus (Chromium). | `parsePriceInput` normalise tout à la sortie du champ (D8). Pour la molette, rien de plus, sauf si le cas apparaît en test. |
| R8  | Min saisi au-dessus de 1 000 000 : les deux poignées sont en butée et celle du max ne peut plus descendre. | Cas limite accepté. La correction passe par le champ. |
| R9  | Bornes 0–1 000 000 arbitraires, faute de données réelles. | Constante unique (D1). |
| R10 | Pas d'index sur `listings.price` (R-07). | Hors périmètre FE. Signalé seulement. |
| R11 | Écarts avec la demande : EP-10 au lieu d'EP-09 ; plan demandé sous `MH-58-plan.md`, qui existe déjà. | Plan écrit sous `MH-65-plan.md`. Le plan MH-58 n'est pas touché. |
| R12 | MCP PostgreSQL, Sequential Thinking et plugin GitHub en échec pendant l'analyse. Le serveur `plugin:context7` demande une authentification. | Inutiles pour ce ticket FE (`gh` utilisé à la place). Context7 joignable via `mcp__context7__*` ; à vérifier au début de l'implémentation. |
| R13 | Monétisation non définie. | Aucune dépendance pour ce ticket. |

---

## 7. Contrat qualité

1. **Orientation** : `graphify query` / `explain` avant tout grep ou lecture. Relire ce plan et `gh issue view 188`. Vérifier qu'on est sur la branche MH-65 (`git fetch`, `git status`).
2. **Context7**, avant d'écrire le code qui s'appuie sur une API, pour vérifier bonnes pratiques, sécurité et idiome de la version installée :
   - React 19 : événements d'un `<input type="range">`, `onChange` / `input` / `change`, écouteur natif via `ref` et nettoyage ;
   - Tailwind 4 : variantes arbitraires sur `::-webkit-slider-thumb` / `::-moz-range-thumb` ;
   - MDN : `input type="range"`, sémantique de `change`, clavier ; `input type="number"` et `valueAsNumber` ;
   - WAI-ARIA APG : motif « Multi-Thumb Slider », `aria-valuetext` ;
   - React Router 8 : `setSearchParams` (déjà utilisé, pas de nouvel usage prévu).
3. **Code minimal, KISS** :
   - une constante de bornes et des fonctions pures dans `priceRange.ts` ;
   - un composant, deux champs de critères ;
   - aucune librairie ajoutée, aucun changement du backend, de `types.ts` ou de `SearchPage` (sauf nécessité constatée) ;
   - rien d'autre que ce que le ticket demande.
4. **Fichier par fichier**, dans l'ordre de la section 2, en montrant chaque fichier avant de passer au suivant. Règles actives : `.claude/rules/react-typecrypt.md`, `general-coding.md`. Commentaires d'une ou deux lignes, seulement là où le code ne s'explique pas seul, sans citer un fichier de règles ou de doc.
5. **Tests unitaires sur la logique pure uniquement** (section 3) : aucune dépendance à la DB ni au réseau, aucun mock de la DB.
6. **Vérifications** de la section 5.
7. **Première review, avant tout fix** :
   - review frontend avec le skill `code-review-frontend`. Pas de review backend : le ticket ne touche pas au backend ;
   - reproduire chaque finding avant de le corriger, et comparer avec `HEAD` (d'autres processus peuvent modifier l'arbre de travail) ;
   - **appliquer les corrections confirmées avant de continuer**, puis refaire l'étape 6.
8. **Passe `humanizer:humanizer`** sur les commentaires, doc comments et textes visibles (légende, libellés, `aria-label`, `aria-valuetext`, lecture formatée). Ton humain, neutre, concis et fidèle au code.
9. **`graphify update .`**
10. **Mémoire** : noter la dette de tests de composants (glissement, clavier, toucher, synchronisation champs ↔ poignées, Clear all).
11. **Arrêt** : ni commit ni push. L'utilisateur relit, commit, merge et ouvre la PR.

### Outils vérifiés pendant l'analyse

- **Skills du repo** (`.claude/skills/`) : `code-review-frontend` et `graphify` (utilisés), `code-review-backend`, `github-ticket`, `docker`, `readme` (non utilisés ici).
- **Règles du repo** (`.claude/rules/`) : `general-coding.md` et `react-typecrypt.md` (actives), `rust.md` et `database.md` (non concernées).
- **Skills de plugins utilisés** : `humanizer:humanizer`, `run`.
- **MCP** : Context7 (`mcp__context7__*`), Git MCP (lecture seule), Filesystem. En échec : PostgreSQL, Sequential Thinking, plugin GitHub (remplacé par la CLI `gh`).

---

## 8. Hors périmètre

Backend et OpenAPI (déjà livrés par MH-64-BE) ; index sur `price` (R-07) ; filtre prix sur le feed `/` ; histogramme des prix ; bornes calculées depuis les données ; mise à jour de TECHNICAL_SPEC et d'ARCHITECTURE (MH-68) ; tests de composants et d'intégration (epic dédiée).

---

## 9. Critères de fin

- [ ] Section « Monthly rent (XAF) », slider 0 – 1 000 000 par pas de 5 000, champs Min / Max synchronisés
- [ ] Min à 0 et max en butée → paramètre absent ; libellé « 1 000 000+ XAF » ; max saisi au-dessus → poignée en butée, montant envoyé
- [ ] Montants au format fr-FR existant suivis de « XAF »
- [ ] Poignées jamais croisées ; à la sortie d'un champ, une borne croisée prend la valeur de l'autre
- [ ] Recherche au relâchement, à la sortie d'un champ ou à l'Entrée, jamais pendant un glissement ; retour page 1
- [ ] `price_min` / `price_max` dans l'URL, restaurés au rechargement ; valeurs invalides ou croisées ignorées
- [ ] « Clear all » remet la section à zéro
- [ ] Clavier : flèches, Page Up / Page Down (±50 000), Home / End ; noms accessibles et valeurs annoncées en XAF
- [ ] Glissement tactile dans le panneau « Filters » mobile
- [ ] Tests de la section 3 ; lint, ts:check, test et prettier propres
