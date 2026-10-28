# MH-67 — Lien vers les autres annonces d'un owner : plan d'implémentation (FE)

- **Epic** : EP-10 — Search (pas d'issue d'epic)
- **Ticket** : MH-67 — Link a listing to its owner's other listings (#192). FE uniquement, pas de sous-ticket.
- **Branche** : `mh-67-link-a-listing-to-its-owners-other-listings` (en local et sur origin, à jour avec `develop` après le merge de MH-66). Aucun commit ni push automatique.
- **Déjà livré, et utilisé ici** : MH-64 (`owner_id` sur `GET /api/v1/search`, `SearchCriteria.ownerId` validé comme UUID, conservé et envoyé, retiré par « Clear all »), MH-62 (section « The owner » de la page détail), `useAuth` / `useProfile`.
- **CDC** : SRCH-03 (filtre par owner).

---

## 1. Contexte et objectif

L'API filtre déjà par owner et la page `/search` lit `owner_id` dans l'URL. Aucun chemin de l'interface n'y mène encore. Le ticket ajoute :

- un lien depuis la page détail vers `/search?owner_id={id}` ;
- sur `/search`, un chip amovible qui indique le filtre owner actif.

### Décisions

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Nom affiché | `formatFullName(first, last)` dans `shared/utils/format.ts` : prénom et nom trimés, parties vides retirées, joints par une espace ; `""` si rien. Utilisé par la page détail et par le chip. Les autres jointures existantes (`RootLayout`, `OwnerRequestForm`) ne sont pas touchées. |
| D2 | Lien de la page détail | Dans la section « The owner », sous le nom : « See other listings by {nom} », ou « See other listings by this owner » si le nom est vide. `Link to={{ pathname: "/search", search: "?owner_id=…" }}` (React Router 8), construit avec `URLSearchParams`. Pas d'import de la feature `search` : `search` importe déjà `listings`, l'inverse créerait un cycle. |
| D3 | Masquage pour l'owner | Le lien n'est affiché que quand on sait que le visiteur n'est pas l'owner : visiteur anonyme, ou connecté avec un profil chargé dont l'`id` diffère. Pendant le bootstrap de session ou le chargement du profil, il est masqué : pas de flash chez l'owner. |
| D4 | Libellé du chip | `ownerFilterLabel(ownerId, listings)` dans `features/search/ownerFilter.ts` (logique pure). Il prend le nom du premier résultat **dont l'owner est celui du filtre**, pour ne pas reprendre le nom d'un autre owner dans les résultats précédents affichés pendant un chargement (`keepPreviousData`). Sans résultat correspondant ou avec un nom vide, il renvoie « Listings by this owner ». |
| D5 | Chip | `OwnerFilterChip` dans `features/search/components/` (un seul usage, donc pas de composant partagé). Libellé, plus un bouton « × » avec `aria-label="Remove filter: {libellé}"`. Placé au-dessus du compteur de résultats, seulement si `criteria.ownerId` est présent. |
| D6 | Suppression | `changeFilters({ ownerId: "" })` : même chemin que les filtres de la sidebar (`replace`, retour page 1). Mot-clé et autres filtres conservés. |
| D7 | Combinaison | Déjà assurée par MH-64 (`toSearchRequest` envoie `owner_id` avec le reste). Rien à changer. |

---

## 2. Fichiers, dans l'ordre

1. `frontend/src/shared/utils/format.ts` : `formatFullName` ; cas ajoutés dans `format.test.ts`.
2. `frontend/src/features/listings/components/ListingDetail.tsx` : `ownerName` via `formatFullName`, le lien (D2, D3).
3. `frontend/src/features/search/ownerFilter.ts` (nouveau) et `ownerFilter.test.ts` (D4).
4. `frontend/src/features/search/components/OwnerFilterChip.tsx` (nouveau, D5).
5. `frontend/src/features/search/components/SearchPage.tsx` : rendu du chip et suppression (D6).

---

## 3. Tests unitaires (vitest, logique pure)

- `formatFullName` :
  - prénom et nom ;
  - un seul des deux ;
  - `null` ou `undefined` ;
  - espaces seulement → `""`.
- `ownerFilterLabel` :
  - nom pris dans un résultat du bon owner ;
  - résultats d'un autre owner ignorés ;
  - liste vide → « Listings by this owner » ;
  - nom vide → « Listings by this owner ».

Pas de test de composant pour le lien et le chip (aucun usage de React Testing Library dans le repo) : dette notée en mémoire.

---

## 4. Points d'intégration

| Zone | Existant | Changement |
| ---- | -------- | ---------- |
| `ListingDetail` section « The owner » | Avatar, nom, avertissement | Lien sous le nom. Le nom passe par `formatFullName` (affichage inchangé). |
| `SearchPage` | `criteria.ownerId`, `changeFilters`, `data.data` | Chip au-dessus des résultats. Aucune logique d'URL nouvelle. |
| `searchCriteria.ts` | `owner_id` validé comme UUID, transmis à l'API | Inchangé. |
| `OwnerSummaryDto` (`types.ts`) | `id`, `first_name`, `last_name` dans chaque résultat | Source du nom du chip, sans appel API en plus. |

---

## 5. Risques

- **R1** — Un `owner_id` valide mais inconnu ne donne aucun résultat. Le chip affiche alors « Listings by this owner » et l'état vide propose « Clear all » (comportement voulu par le ticket).
- **R2** — Pendant un changement d'owner, les résultats précédents restent affichés. Traité par D4.
- **R3** — Un owner qui consulte sa propre annonce sans être connecté voit le lien : il n'est pas identifiable. C'est voulu.

---

## 6. Vérifications

- `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`.
- Playwright sur l'app réelle :
  - le lien est visible pour un visiteur anonyme et mène à `/search?owner_id=…` ;
  - le libellé utilise le nom de l'owner ;
  - le chip affiche le nom ; avec un owner sans résultat, il affiche « this owner » ;
  - la suppression retire `owner_id` et garde `q`, les filtres et les prix ;
  - le filtre owner se combine avec un mot-clé ;
  - le lien est absent pour l'owner connecté. À vérifier si une session owner est disponible ; sinon, contrôle sur le code et dette notée.

---

## 7. Contrat qualité

- Context7 : `Link` avec un objet `to` (React Router 8), vérifié.
- KISS : deux fonctions pures, un composant, aucune librairie ajoutée, aucun changement de l'API.
- Tests unitaires sur la logique pure uniquement.
- Première review `code-review-frontend` avant tout fix, avec chaque finding reproduit.
- Passe `humanizer:humanizer` sur les commentaires et les textes visibles.
- `graphify update .` et note de dette de tests en mémoire.
- Ni commit ni push.
