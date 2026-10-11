# MH-62 — Page détail alignée sur DET-01 / DET-02 : plan d'implémentation (FE)

- **Epic** : EP-09 — Listings (#129)
- **Ticket** : MH-62 — Bring the listing detail page in line with DET-01 and DET-02 (#173). Workstream FE uniquement, pas de sous-ticket BE.
- **Branche** : `mh-62-bring-the-listing-detail-page-in-line-with-det-01-and-det-02` (existe en local et sur origin). Aucun commit ni push automatique.
- **Déjà livré, et utilisé ici** : `GET /listings/:id` renvoie déjà `owner.avatar_url` (`OwnerDetailDto`, `listings/dto.rs`). Les helpers `formatPrice` / `formatInitials` (`shared/utils/format.ts`) et `isRemoteMediaUrl` (`shared/utils/mediaUrl.ts`) existent, ainsi que le composant `Alert` (variante `warning`, prop `title`).
- **CDC v2.1 §4.2.3** :
  - DET-01 : la page détail affiche « un message d'avertissement concernant tout échange et transaction avec un propriétaire ».
  - DET-02 : elle affiche les informations publiques du propriétaire (prénom, nom, photo de profil), sans le numéro.

---

## 1. Contexte et objectif

La page détail publique (`features/listings/components/ListingDetail.tsx`) s'écarte des exigences sur cinq points :

| Écart | Aujourd'hui | Cible |
| ----- | ----------- | ----- |
| DET-01 | Aucun avertissement | `Alert` warning « Before you contact this owner » dans la section owner |
| DET-02 | Initiales seules, `owner.avatar_url` ignoré | Photo si l'URL est définie et se charge. Sinon, initiales |
| Signalement | Bouton « Report this listing » qui ouvre une modale sans envoi | Bouton et modale retirés (le signalement passe dans EP-12) |
| Devise | « FCFA / month » sur la carte du feed, la page détail et « My properties ». Le formulaire dit « XAF » | « {amount} XAF / month » partout, produit en un seul point. Footer : « All prices in XAF » |
| Not-found | « … or its status changed to “unavailable” » | N'évoque plus l'indisponibilité (un bien indisponible reste public) |

**Objectif** : corriger ces cinq écarts côté FE uniquement, sans changer l'API, et ajouter un test vitest sur le libellé prix.

### Décisions prises

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Libellé prix | Le helper pur `formatMonthlyRent(price): { amount: string; unit: string }` vit dans `shared/utils/format.ts`. `amount` vient de `formatPrice` (format inchangé) et `unit` vaut `"XAF / month"`. Le petit composant `ListingPrice` rend `amount` et `unit` en deux tons, comme aujourd'hui. Les trois vues ne passent que des classes de taille. Pas d'`aria-label`, ignoré sur un `<span>` générique : le texte rendu se lit déjà « 1 200 XAF / month ». |
| D2 | Constante devise | `PRICE_CURRENCY = "XAF"` dans `shared/utils/format.ts`, utilisé par `formatMonthlyRent` et par `SiteFooter`. `shared/` ne peut pas importer depuis `features/`, donc la constante reste dans `shared`. |
| D3 | Emplacement de `ListingPrice` | `features/listings/components/ListingPrice.tsx`. Les trois consommateurs (`ListingCard`, `ListingDetail`, `OwnerListingRow`) sont tous dans le feature `listings`. Composant interne, non exporté par `listings/index.ts`. |
| D4 | Avatar owner | Nouveau composant `features/listings/components/OwnerAvatar.tsx` (props `avatarUrl`, `firstName`, `lastName`). L'`<img>` n'est rendue que si `avatarUrl` passe `isRemoteMediaUrl`. Sur `onError`, l'URL est mémorisée comme cassée (`failedUrl`, même schéma que `brokenUrl` dans `AvatarUpload`), ce qui ramène aux initiales. Un nouvel `avatarUrl` repart donc d'un état propre. Les initiales viennent de `formatInitials(first, last) \|\| "?"` et remplacent le calcul inline actuel. |
| D5 | Accessibilité de l'avatar | Le nom de l'owner est affiché juste à côté : `alt=""` sur l'image et `aria-hidden` sur les initiales, pour éviter la double lecture. |
| D6 | Avertissement | `<Alert variant="warning" title="Before you contact this owner">Never send money before visiting the property. My House does not hold deposits and is not a party to your agreement.</Alert>`, placée sous la ligne owner, dans la section « The owner ». Taille standard d'`Alert` (`text-sm`, identique à l'alerte « unavailable » déjà sur la page), pas de texte de bas de page. `role="note"` sur cette instance (voir R4). |
| D7 | Signalement | Bouton, état `isReportOpen`, import et rendu de `ReportListingModal` retirés de `ListingDetail`. Le fichier `ReportListingModal.tsx` est supprimé : il n'a pas d'autre consommateur et EP-12 le reconstruira contre une vraie API. L'entrée texte « Report a listing » du footer reste en place (hors périmètre). |
| D8 | Not-found | Titre « Listing not found ». Description « This listing doesn't exist, was removed by its owner, or isn't public. » CTA « Back to listings » inchangé. |
| D9 | Commentaire de `formatPrice` | Réécrit en une ligne neutre (format fr-FR, montant seul). Plus de renvoi à la question de devise ni au blocker de monétisation. |

---

## 2. Points d'intégration avec l'existant

### 2.1 Fichiers, dans l'ordre (un fichier à la fois)

1. **`frontend/src/shared/utils/format.ts`**
   - `export const PRICE_CURRENCY = "XAF";`
   - `formatMonthlyRent(price)` → `{ amount: formatPrice(price), unit: \`${PRICE_CURRENCY} / month\` }` ;
   - commentaire de `formatPrice` réécrit (D9) ;
   - `formatInitials` inchangé, mais son commentaire cite déjà « detail » et devient exact.
2. **`frontend/src/shared/utils/format.test.ts`** : bloc `describe("formatMonthlyRent")`.
   - Séparateur de milliers : `formatMonthlyRent(150000)` donne `amount === "150 000"` (U+202F, comme le test existant) et `unit === "XAF / month"`.
   - Montant entier sans décimale : `formatMonthlyRent(950).amount === "950"` (pas de `",00"`).
3. **`frontend/src/features/listings/components/ListingPrice.tsx`**
   - Props : `price: number`, `className?` (montant), `unitClassName?` (unité) ;
   - rendu : `<span className={className}>{amount}{" "}<span className={unitClassName}>{unit}</span></span>`.
4. **`frontend/src/features/listings/components/ListingCard.tsx`** : remplace `formatPrice(...) + "FCFA / month"` par `ListingPrice`, avec les classes actuelles (`text-[11.5px] font-medium text-ink-500` sur l'unité). L'import `formatPrice` est retiré.
5. **`frontend/src/features/listings/components/OwnerListingRow.tsx`** : même remplacement (`text-xs font-medium text-ink-500`).
6. **`frontend/src/features/listings/components/OwnerAvatar.tsx`** : composant D4/D5 (carré `size-11 bg-ink-900`, `<img className="size-full object-cover">`).
7. **`frontend/src/features/listings/components/ListingDetail.tsx`**, en un seul passage :
   - prix via `ListingPrice` (`text-2xl font-bold text-ink-900` / `text-sm font-medium text-ink-500`) ;
   - `OwnerAvatar` à la place du `<span>` d'initiales inline ;
   - `Alert` DET-01 sous la ligne owner (D6) ;
   - suppression du bouton report, de `useState`, de `isReportOpen` et de `ReportListingModal` ; le `justify-between` de la ligne owner n'a plus de raison d'être ;
   - imports nettoyés (`useState`, `formatPrice`).
8. **`frontend/src/features/listings/components/ReportListingModal.tsx`** : supprimé (D7).
9. **`frontend/src/features/listings/components/ListingNotFound.tsx`** : nouveau texte (D8).
10. **`frontend/src/shared/components/SiteFooter.tsx`** : `© 2026 My House. All prices in {PRICE_CURRENCY}.`

### 2.2 Ce qui ne change pas

- **Backend / API / `types.ts`** : aucun changement. `OwnerDetailDto.avatar_url` est déjà généré. Les cartes (`OwnerSummaryDto`) n'ont pas d'avatar, ce qui est voulu.
- **`ListingForm.tsx`** : le libellé « Monthly rent (XAF) » et le suffixe de validation `" XAF"` restent tels quels (ce ne sont pas des affichages de prix d'annonce).
- **Docs** : ni TECHNICAL_SPEC ni ARCHITECTURE ne mentionnent de devise, rien à mettre à jour. Le wireframe `mh-13-feed-detail.html` (FCFA, texte d'avertissement en français) est un artefact historique et n'est pas modifié.
- **EP-11** : le reveal du téléphone déplacera l'avertissement sous le numéro (note technique du ticket). D6 le place dans la section owner pour que ce déplacement reste local.

### 2.3 Vérification

- `npm run lint`, `npx tsc -b` et `npx vitest run src/shared/utils/format.test.ts` dans `frontend/`.
- `grep -rn "FCFA" frontend/src` doit être vide. `grep -rn "ReportListingModal" frontend/src` aussi.
- Vérification manuelle (Playwright headless, comme MH-54/56/57-FE) sur la page détail :
  - owner avec avatar ;
  - owner sans avatar ;
  - avatar en 404 (retour aux initiales) ;
  - présence de l'alerte ;
  - absence du bouton report ;
  - libellé prix sur le feed, la page détail et « My properties » ;
  - footer ;
  - not-found sur un id inconnu.
- `graphify update .` après les modifications.

---

## 3. Risques / inconnues identifiés

| #  | Risque | Traitement |
| -- | ------ | ---------- |
| R1 | `listings.price` est `NUMERIC(12,2)`. Un prix non entier inséré hors formulaire (seed, SQL direct) s'afficherait « 1 200,5 ». | Le formulaire impose un entier et le ticket demande un format « as today ». Pas d'arrondi ajouté. Le test fige le cas entier. |
| R2 | `ListingNotFound` est aussi rendu par `EditListingPage` et `ListingPhotosPage` (côté owner). Le nouveau texte « isn't public » y est un peu décalé. | Accepté : un 404 côté owner veut dire « supprimé ou pas à vous », et le texte reste vrai. Pas de prop de variante (KISS). |
| R3 | URL d'avatar pointant vers un fichier supprimé (avatar remplacé entre-temps) : l'image casse. | Couvert par le retour aux initiales sur `onError` (D4). |
| R4 | `Alert` impose `role="alert"`, une sémantique « urgente » pour un contenu statique. | Tranché via Context7 (ARIA APG) : une alerte rendue dynamiquement est annoncée, et la page détail se rend après le spinner. `role="note"` est donc passé via `...rest` sur cette seule instance. L'alerte « unavailable » n'est pas modifiée (hors périmètre). |
| R5 | Aucune infrastructure de test de composant (pas de React Testing Library dans le repo). | Tests vitest sur la logique pure seulement (`formatMonthlyRent`). Le rendu (avatar et repli, alerte, absence du bouton) est vérifié manuellement. Dette à consigner en mémoire, comme pour les tickets FE précédents. |
| R6 | Texte de l'avertissement : le wireframe parlait d'un texte « à valider par le produit/légal ». | Le ticket fixe le texte exact, qui est repris mot pour mot. |
| R7 | Monétisation non définie (blocker CLAUDE.md). | Aucune dépendance : le ticket tranche que tout prix est un loyer mensuel en XAF. |

---

## 4. Contrat qualité (méthodologie d'implémentation)

1. **Context7 avant d'écrire** : React 19 (`<img onError>`, remise à zéro d'un état dérivé d'une prop), React Router (inchangé ici), et la sémantique `role="alert"` vs `note` pour R4. Pas d'appui sur la mémoire d'entraînement pour ces API.
2. **Code minimal, KISS** : un helper pur et deux petits composants, aucune abstraction supplémentaire. Pas de variante de `ListingNotFound`, pas de composant `Avatar` partagé : le seul consommateur est la page détail. Les styles existants sont réutilisés tels quels.
3. **Tests unitaires sur la logique pure seulement** : `formatMonthlyRent` dans `format.test.ts`. Aucun test qui demanderait une DB, un mock réseau ou un rendu de composant.
4. **Une première review avant tout fix** : une fois tous les fichiers de 2.1 écrits, une review complète. Chaque finding est vérifié en le reproduisant, sur un diff comparé à `HEAD`. Les fixes sont appliqués et revérifiés avant de continuer.
5. **Review par domaine** : frontend via le skill `code-review-frontend` (présent dans `.claude/skills/`). Pas de review backend : le ticket ne touche aucun fichier `backend/`.
6. **Passe `humanizer:humanizer`** sur les commentaires ajoutés ou modifiés (`format.ts`, `ListingPrice.tsx`, `OwnerAvatar.tsx`). Ton humain, neutre et concis, fidèle au code, pas de référence à un document externe.
7. **Fin de ticket** : lint, `tsc` et vitest passent, `graphify update .` est lancé, et une note de dette de couverture est ajoutée en mémoire (rendu de l'avatar et repli, alerte, absence du bouton report, libellé dans les trois vues : vérifiés manuellement seulement). Le commit reste à la charge de l'utilisateur.
