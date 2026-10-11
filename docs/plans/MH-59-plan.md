# MH-59 — Écran de gestion des photos d'un bien : plan d'implémentation (FE)

- **Epic** : EP-09 — Listings (#129)
- **Ticket** : MH-59 — Build the listing photo management screen (#166), workstream FE uniquement, aucun sous-ticket.
- **Branche** : `mh-59-build-the-listing-photo-management-screen` (branche courante, déjà poussée). Le code est écrit directement dessus (pas de découpage BE/FE), puis une PR unique part vers `develop`.
- **Déjà livré, et utilisé ici** : MH-50/51/52 (`POST /media/upload`, `DELETE /media/:id`, `PATCH /listings/:id/cover`, cover automatique au premier upload, verrou du bien, recontrôle du quota à l'insert), MH-54-FE (`OwnerBar`, `listingVisibility`), MH-56-FE (contrôle de propriété dans `EditListingPage`), MH-58-FE (`isAlreadyDeleted`, motif de modale destructive `DeleteListingAction`), profil (`AvatarUpload`, `avatarValidation.preCheck`, `FileDropzone`).
- **Suite** : MH-60 (réordonnancement) se branchera sur les tuiles de cet écran ; MH-61 (publication) sur la page de détail.
- **Exigences CDC** : LIST-04 « de 1 à 5 photos par bien (JPEG, PNG, WebP, 5 Mo max) », LIST-05 « cover automatique à la première photo, modifiable ensuite ».

L'implémentation se fait dans une session dédiée, selon le contrat qualité de la section 7.

---

## 1. Contexte et objectif

Un owner doit pouvoir ajouter, supprimer et choisir la cover des photos de son bien. Un bien sans photo ne peut pas devenir public. Le backend est complet ; ce ticket ne touche que le frontend.

Parcours owner (déjà décidé) : création du bien → page de détail (page de relecture) → « Next: add photos → » → cet écran → « Finish » → retour au détail, où le bien pourra être publié (MH-61). « Finish » reste disponible avec zéro photo.

Contrat backend utilisé :

| Appel | Succès | Erreurs gérées |
| ----- | ------ | -------------- |
| `POST /api/v1/media/upload` (multipart `listing_id` + `file`) | 201 `{ data: MediaDto }` (`id`, `url`, `is_cover`, `position`) | 409 `MEDIA_QUOTA_EXCEEDED`, 413 `PAYLOAD_TOO_LARGE`, 422 `INVALID_FILE`, 404 `LISTING_NOT_FOUND` |
| `DELETE /api/v1/media/:id` | 204 | 409 `COVER_PHOTO_REQUIRED`, 404 `MEDIA_NOT_FOUND` |
| `PATCH /api/v1/listings/:id/cover` `{ media_id }` | 200 `{ data: MediaDto }` (nouvelle cover) | 404 |
| `GET /api/v1/listings/:id` | 200 ; l'owner voit toujours son bien, brouillon ou sans photo compris | 404 pour tout autre appelant |

Les types (`MediaDto`, `MediaResponse`, `PromoteCoverRequest`, `ListingMediaDto`) sont déjà présents dans `shared/api/types.ts` : aucune régénération n'est nécessaire.

### Décisions prises

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Branche | Branche courante `mh-59-build-the-listing-photo-management-screen`, PR unique vers `develop`. |
| D2 | Filtre de `FileDropzone` | Le filtre silencieux `isAcceptedFile` est retiré : la dropzone transmet tous les fichiers et `accept` ne sert plus qu'au sélecteur natif. Chaque appelant valide lui-même. Avatar et owner-request, qui pré-contrôlent déjà, affichent alors leur message au lieu d'ignorer le fichier. |
| D3 | Pré-contrôle partagé | `shared/utils/imageFileCheck.ts` expose `imageFileProblem(file): "format" \| "size" \| null`. Constantes renommées `ACCEPTED_IMAGE_TYPES` / `MAX_IMAGE_SIZE_BYTES`. Chaque feature garde ses messages : l'avatar est inchangé à l'écran. |
| D4 | Suppression d'une photo | Modale de confirmation, motif de `DeleteListingAction` (MH-58) : non fermable pendant la requête, erreur affichée dans la modale. |
| D5 | Changement de cover | Pas d'optimisme : le marqueur « Cover » bouge sur 200 (`setQueryData` à partir de la réponse). Une erreur laisse l'ancienne cover, avec un toast d'erreur. |
| D6 | Une action photo à la fois | Pendant un changement de cover ou une suppression, tous les « Make cover » et « Delete » sont désactivés. Les uploads ne sont pas bloqués. Cela évite la course cover/suppression sur la même photo. |
| D7 | Source des tuiles | Photos serveur : `listing.media` du cache `["listing", id]` (même requête que la page de détail), dans l'ordre renvoyé par le backend (`ORDER BY position`). Uploads en cours : état local du composant. |
| D8 | Fin d'un upload | Sur 201, la photo est ajoutée au cache `["listing", id]` puis la tuile locale est retirée : pas de clignotement en attendant le refetch. |
| D9 | Invalidations | Dans le `onSettled` du hook (succès ou échec) : `["listing", id]`, `["listings"]`, `["owner-listings"]`, seulement quand la dernière mutation photo du bien se termine (`isMutating(...) === 1`, motif de `useUpdateListingStatus`). Un 409 de quota inattendu resynchronise ainsi le compte local. |
| D10 | 404 sur `DELETE /media/:id` | Traité comme un succès via `isAlreadyDeleted` (la photo n'existe plus, la tuile disparaît), comme MH-58. |
| D11 | Emplacement | Tout dans `features/listings/` (pas de nouvelle feature `media` côté FE) : l'écran, les hooks et la logique pure appartiennent au domaine du bien. |

---

## 2. Fichiers, dans l'ordre (un fichier à la fois)

### 2.1 Extraction du pré-contrôle et dropzone

1. **`frontend/src/shared/api/constants.ts`** : `MAX_AVATAR_SIZE_BYTES` → `MAX_IMAGE_SIZE_BYTES`, `ACCEPTED_AVATAR_TYPES` → `ACCEPTED_IMAGE_TYPES` (même valeur, commentaire « Mirrors `MAX_IMAGE_SIZE_BYTES` » conservé). Ajout de `MAX_PHOTOS_PER_LISTING = 5`, qui reflète `MAX_PHOTOS_PER_LISTING` de `backend/src/modules/media/service.rs`. Les constantes `ID_*` d'owner-request restent telles quelles.
2. **`frontend/src/shared/utils/imageFileCheck.ts`** (nouveau) + **`.test.ts`** : `export type ImageFileProblem = "format" | "size";` et `imageFileProblem(file: File): ImageFileProblem | null` (type hors liste → `"format"`, puis taille > max → `"size"`). Les quatre cas de `preCheck` de `avatarValidation.test.ts` y déménagent.
3. **`frontend/src/features/profile/avatarValidation.ts`** : `preCheck` délègue à `imageFileProblem` et mappe la raison sur `FORMAT_MESSAGE` / `SIZE_MESSAGE`. `serverMessage` inchangé. Le test garde deux cas de mapping (format, taille) ; les bornes sont testées dans `shared/`.
4. **`frontend/src/features/profile/components/AvatarUpload.tsx`** : import renommé (`ACCEPTED_IMAGE_TYPES`). Aucun autre changement.
5. **`frontend/src/shared/components/FileDropzone.tsx`** (D2) : suppression d'`isAcceptedFile` ; `handleFiles` transmet `Array.from(fileList)`. Doc comment d'une ligne sur `accept` : filtre du sélecteur natif seulement, la validation revient à l'appelant.

### 2.2 API et logique pure

6. **`frontend/src/features/listings/api.ts`** :
   - `export type ListingMedia = components["schemas"]["MediaDto"];`
   - `uploadListingPhoto(listingId, file): Promise<{ data: ListingMedia }>` → `FormData` (`listing_id`, puis `file`) → `apiUpload("/api/v1/media/upload", formData)` ;
   - `deleteListingPhoto(mediaId): Promise<void>` → `apiDelete(\`/api/v1/media/${encodeURIComponent(mediaId)}\`)` ;
   - `setListingCover(listingId, mediaId): Promise<{ data: ListingMedia }>` → `apiPatch(\`/api/v1/listings/${encodeURIComponent(listingId)}/cover\`, { media_id: mediaId })`.
7. **`frontend/src/features/listings/listingPhotos.ts`** (nouveau) + **`.test.ts`** : seule logique métier pure de l'écran.
   - `freePhotoSlots(photoCount, inFlightCount): number` → `Math.max(0, MAX_PHOTOS_PER_LISTING - photoCount - inFlightCount)`.
   - `planPhotoSelection(files: File[], freeSlots: number)` → `{ rejected: { file: File; problem: ImageFileProblem }[]; toUpload: File[]; skippedCount: number }`. Pré-contrôle d'abord ; les fichiers valides sont pris dans l'ordre de sélection jusqu'à `freeSlots`, le reste est compté dans `skippedCount`. Un fichier rejeté ne consomme pas de place.
   - `photoUploadErrorMessage(error: unknown): string` : `INVALID_FILE` → format ; **statut** 413 → taille (le 413 peut venir de nginx sans enveloppe JSON, R3) ; `MEDIA_QUOTA_EXCEEDED` → quota ; 404 → bien introuvable ; autre → message générique. Jamais le texte brut du serveur.
   - `isCoverRequiredError(error: unknown): boolean` → `ApiError` 409 `COVER_PHOTO_REQUIRED`.
   - Messages exportés en constantes (texte anglais, comme le reste de l'app).
8. **`frontend/src/features/listings/hooks/useListingPhotoMutations.ts`** (nouveau) : trois hooks, une préoccupation chacun, même clé `LISTING_PHOTOS_MUTATION_KEY = ["listing-photos"]` suivie de `listingId`, `retry: false`, logique de cache au niveau du hook (R1) :
   - `useUploadListingPhoto(listingId)` : `onSuccess` ajoute `response.data` à `listing.media` dans `["listing", listingId]` ;
   - `useDeleteListingPhoto(listingId)` : `mutationFn` avale le 404 (`isAlreadyDeleted`, D10) ; `onSuccess` retire la photo de `listing.media`. Si c'était la cover et la dernière photo, la liste devient vide ;
   - `useSetListingCover(listingId)` : `onSuccess` met `is_cover = (media.id === response.data.id)` sur chaque photo ;
   - `onSettled` commun (helper local au fichier) : invalidations D9.
   Les trois écritures de cache passent par un petit helper `updateListingMedia(queryClient, listingId, update)` local au fichier.

### 2.3 Composants et intégration

9. **`frontend/src/features/listings/components/PhotoTile.tsx`** (nouveau) : tuile en `<li>`, deux zones fixes pour accueillir MH-60 sans restructuration :
   - zone image (ratio fixe, `object-cover`, `alt` « Photo N of {title} ») ;
   - barre d'actions sous l'image, en `flex` : à gauche un emplacement libre (futurs contrôles d'ordre de MH-60, rien rendu ici) ; à droite le badge texte « Cover » (`Badge`) ou le bouton « Make cover » toujours visible, puis « Delete ».
   - Cover avec d'autres photos : « Delete » désactivé et indication « Choose another cover before deleting this photo. » reliée par `aria-describedby`.
   - Variante en cours d'upload : aperçu `blob:` (validé par `isDisplayableMediaUrl`), `Spinner` sans pourcentage, actions absentes.
   - Variante en échec : aperçu, `Alert` `error` avec le message de `photoUploadErrorMessage`, bouton « Remove » qui retire la tuile locale.
10. **`frontend/src/features/listings/components/DeletePhotoModal.tsx`** (nouveau, D4) : `Modal` partagé, « Delete this photo? », « This photo will be permanently deleted. », `Cancel` (`secondary`) et `Delete photo` (`danger`, `isLoading`). `handleClose` sans effet pendant la requête ; `mutation.reset()` à l'ouverture. Sur 409 `COVER_PHOTO_REQUIRED` (`isCoverRequiredError`), la modale affiche la même indication que la tuile ; autre erreur → message générique ; succès → modale fermée.
11. **`frontend/src/features/listings/components/ListingPhotosPage.tsx`** (nouveau) :
    - Chargement, 404 et contrôle de propriété copiés sur `EditListingPage` : `useListing(id)` + `useProfile()`, `profile.id !== listing.owner.id` → `ListingNotFound` (même si le bien est public).
    - État local : `pendingUploads: { key: string; file: File; previewUrl: string; error: string | null }[]` et `selectionNotice: { rejected: { name: string; message: string }[]; skippedCount: number } | null`.
    - `handleFilesSelected(files)` : `planPhotoSelection(files, freePhotoSlots(photos.length, inFlight))` ; message inline par fichier rejeté (nommé) ; message « N file(s) were skipped: a listing holds 5 photos at most. » si `skippedCount > 0` ; chaque fichier de `toUpload` reçoit sa tuile et part seul via `mutateAsync(file).then(retirer la tuile, ou la passer en erreur)` (R1).
    - `URL.revokeObjectURL` au retrait d'une tuile et au démontage.
    - Zone d'ajout : `FileDropzone multiple accept={ACCEPTED_IMAGE_TYPES.join(",")}` avec « JPEG, PNG or WebP · 5 MB max · 5 photos max ». Quand `photos + uploads en cours ≥ 5`, elle est remplacée par « You've reached the 5-photo limit. Delete a photo to add another. ».
    - Grille : photos serveur puis tuiles locales.
    - « Make cover » : `useSetListingCover`, désactivé selon D6 ; erreur → toast « Couldn't change the cover. Please try again. ».
    - « Finish » : `navigate(\`/listings/${id}\`)`, désactivé tant que `useIsMutating({ mutationKey: [...LISTING_PHOTOS_MUTATION_KEY, id] }) > 0`.
    - Lien retour « Back to listing » vers le détail.
12. **`frontend/src/features/listings/components/OwnerBar.tsx`** : second lien vers `/owner/listings/${listingId}/photos`, icône `Images` (lucide), libellé « Next: add photos → » si `!hasPhoto`, « Manage photos » sinon. Même style que « Edit listing ».
13. **`frontend/src/app/router.tsx`** : `ListingPhotosPage` en `lazy`, route `owner/listings/:id/photos` sous `RequireAuth` + `RequireOwner` (anonyme → `/login`, seeker/admin → `/`).
14. **`frontend/src/features/listings/index.ts`** : export de `ListingPhotosPage`, comme les autres pages.

---

## 3. Tests unitaires (vitest, logique pure, sans DB ni réseau)

- `imageFileCheck` : JPEG/PNG/WebP sous la limite → `null` ; GIF et type vide → `"format"` ; taille limite + 1 → `"size"` ; taille exactement à la limite → `null` ; mauvais format **et** trop gros → `"format"`.
- `avatarValidation` : `preCheck` mappe `"format"` et `"size"` sur les messages existants ; tests de `serverMessage` inchangés.
- `planPhotoSelection` : tout valide sous le quota ; plus de fichiers valides que de places (premiers gardés, `skippedCount` exact) ; `freeSlots = 0` ; rejetés et valides mêlés (un rejeté ne prend pas de place) ; sélection vide.
- `freePhotoSlots` : 0 photo, 3 + 2 en cours → 0, jamais négatif.
- `photoUploadErrorMessage` : `INVALID_FILE`, 413 avec et sans code, `MEDIA_QUOTA_EXCEEDED`, 404, erreur réseau (`TypeError`), code inconnu → générique.
- `isCoverRequiredError` : 409 `COVER_PHOTO_REQUIRED` → `true` ; 409 `MEDIA_QUOTA_EXCEEDED`, 404, non-`ApiError` → `false`.
- **Dette** (pas encore de React Testing Library) : états des tuiles, verrouillages D6, modale, « Finish », redirections de route, contrôle de propriété, mises à jour de cache. Vérifiés à la main (section 5) et notés en mémoire.

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement MH-59 |
| ---- | -------- | ---------------- |
| `shared/api/constants.ts` | `ACCEPTED_AVATAR_TYPES`, `MAX_AVATAR_SIZE_BYTES` | Renommés en génériques (D3) ; ajout de `MAX_PHOTOS_PER_LISTING`. |
| `features/profile/avatarValidation.ts` | `preCheck` renvoie un message | Délègue à `shared/utils/imageFileCheck` ; aucun import inter-features. |
| `shared/components/FileDropzone.tsx` | Filtre silencieux par `accept` ; `multiple` déjà supporté | Filtre retiré (D2). Impact visible sur avatar et owner-request (ils affichent leur message). |
| `features/owner-request` | `preCheckImage` / `preCheckPdf` propres, constantes `ID_*` | Pas de changement de code ; comportement de la dropzone à vérifier (R7). |
| `shared/api/client.ts` | `apiUpload` (FormData rejouable après refresh), `apiDelete` (204 → `undefined`), `apiPatch`, `ApiError` | Réutilisés tels quels. |
| `features/auth/AuthContext.tsx` | Refresh de session en single-flight | Plusieurs uploads qui reçoivent un 401 en même temps partagent un seul refresh. Rien à coder. |
| `shared/api/types.ts` | `MediaDto`, `MediaResponse`, `PromoteCoverRequest` déjà générés | Aucune régénération. |
| `useListing` / clé `["listing", id]` | Partagée par le détail et l'édition | Écrite par les trois hooks photo puis invalidée : le détail est à jour au retour. |
| Clés `["listings", …]`, `["owner-listings", …]` | Feed et « My properties » (`cover_photo_url`) | Invalidées par la dernière mutation photo (D9). |
| `useUpdateListingStatus` | Motif `isMutating(...) === 1` dans `onSettled` | Même motif, sur la clé `["listing-photos", id]`. |
| `listingDeletion.isAlreadyDeleted` | 404 = déjà supprimé | Réutilisé pour `DELETE /media/:id` (D10). |
| `EditListingPage` | Contrôle `profile.id === listing.owner.id` | Même contrôle dans `ListingPhotosPage`. |
| `RequireAuth` / `RequireOwner` | Redirections `/login` et `/` (rôle lu depuis le profil) | Réutilisés pour la nouvelle route. |
| `OwnerBar` | Badge de visibilité, lien « Edit listing », prop `hasPhoto` | Lien photos ajouté, libellé selon `hasPhoto`. |
| `ListingDetail` | Affiche `listing.media` filtré par `isRemoteMediaUrl` | Inchangé ; reçoit les photos via le cache. |
| `DeleteListingAction` / `Modal` | Modale destructive non fermable pendant la requête | Même motif pour `DeletePhotoModal` ; `Modal` inchangé. |
| `shared/utils/mediaUrl.ts` | `isRemoteMediaUrl`, `isDisplayableMediaUrl` (`blob:` accepté) | Photos serveur filtrées comme dans le détail ; aperçus locaux via `isDisplayableMediaUrl`. |
| `ListingForm` | Navigue vers `/listings/:id` après création | Inchangé : étape 2 du parcours déjà en place. |
| Wireframe `mh-14-listing-management.html` (écran 2) | Grille, ruban cover, tuile d'ajout, état d'upload | Suivi pour la structure ; la poignée de réordonnancement et la barre de progression sont exclues (MH-60, pas de pourcentage). |

---

## 5. Vérifications

**Commandes** : `npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`.

**À la main** (skill `run` ou Playwright headless, APP_ENV=development, backend local) :

- route : anonyme → `/login` ; seeker et admin → `/` ; owner sur le bien d'un autre owner, publié ou non → état « not found » ;
- `OwnerBar` : « Next: add photos → » sans photo, « Manage photos » avec photo ; lien vers l'écran ;
- sélection multiple de 3 images valides : 3 tuiles avec indicateur, puis 3 photos ; la première devient « Cover » ;
- sélection avec un GIF et une image > 5 Mo : deux messages nommant les fichiers, les fichiers valides partent quand même ;
- 4 photos présentes, sélection de 3 fichiers valides : 1 upload, message « 2 file(s) were skipped » ; à 5, zone d'ajout remplacée par le message de limite ;
- fichier `.jpg` contenant un PDF (magic bytes) : tuile en erreur « format », autres photos intactes ; « Remove » retire la tuile ;
- quota concurrent : deux onglets, 4 photos, un upload dans chaque → un 201 et un 409 affiché sur la tuile ;
- bien supprimé dans un autre onglet pendant l'upload → erreur « listing not found » sur la tuile ;
- « Make cover » : bouton désactivé pendant la requête (throttling), marqueur déplacé sur 200 ; erreur forcée (serveur coupé) → ancienne cover en place, toast ;
- suppression : photo non cover → modale, tuile retirée ; cover avec d'autres photos → « Delete » désactivé et indication ; cover seule photo → supprimée ; 409 forcé (photo ajoutée dans un autre onglet) → indication dans la modale ;
- « Finish » : désactivé pendant tout upload, changement de cover ou suppression ; actif avec zéro photo ; retour au détail ;
- détail, « My properties » et feed à jour sans rechargement (cover du feed et de la ligne, bien publié qui devient visible à la première photo ou caché à la dernière suppression) ;
- avatar et owner-request : un GIF déposé affiche maintenant le message de format (D2) ;
- largeur 360 px : grille et barre d'actions sans débordement ; navigation clavier sur la dropzone, « Make cover », « Delete », la modale.

---

## 6. Risques et inconnues

| #   | Risque / inconnue | Traitement |
| --- | ----------------- | ---------- |
| R1  | TanStack Query v5 : les callbacks passés à `mutate()` ne sont appelés que pour le dernier appel, et pas après démontage. Avec plusieurs uploads simultanés sur un même `useMutation`, l'état par tuile serait faux. | Cache dans les callbacks du hook (appelés pour chaque mutation) ; état des tuiles via la promesse de `mutateAsync` par fichier. `isPending` du hook n'est pas utilisé pour les uploads. À confirmer via Context7. |
| R2  | Course au quota : deux uploads acceptés localement peuvent produire un 409 `MEDIA_QUOTA_EXCEEDED` (recontrôle à l'insert). | Erreur normale sur la tuile (note technique du ticket) ; l'invalidation de D9 resynchronise le compte. |
| R3  | 413 renvoyé par nginx (`client_max_body_size 10M`) sans enveloppe JSON → `code = UNKNOWN_ERROR`. | `photoUploadErrorMessage` se base sur le statut 413, pas sur le code. Peu probable après le pré-contrôle à 5 Mo. |
| R4  | `file.type` vide ou non standard (HEIC sur iOS, extensions inconnues). | Rejeté comme « format » localement ; le serveur reste l'autorité (magic bytes). Accepté pour le MVP. |
| R5  | Navigation hors de l'écran pendant un upload (header, bouton retour du navigateur) : seul « Finish » est verrouillé. | Les uploads continuent ; le cache est tenu à jour par les callbacks du hook ; les tuiles locales et les object URLs disparaissent au démontage. Pas de `useBlocker` (hors ticket). |
| R6  | Positions non fiables (doublons après suppression, MH-60) : un refetch peut changer l'ordre affiché. | Ordre serveur affiché tel quel ; corrigé par MH-60. La barre d'actions garde un emplacement libre pour ses contrôles. |
| R7  | D2 change le comportement d'avatar et owner-request (fichier refusé affiché au lieu d'ignoré). | Les deux pré-contrôlent déjà ; vérification manuelle (section 5). Pas de modification de leur code. |
| R8  | Focus après suppression : la tuile et le déclencheur de la modale sont démontés, le focus tombe sur `body`. | Accepté pour le MVP comme MH-58 R3 ; à noter si la review FE le relève. |
| R9  | Annonce des erreurs et fins d'upload aux lecteurs d'écran. | `Alert` (rôle d'alerte) sur la tuile en erreur et pour les fichiers rejetés ; pas de région live supplémentaire (KISS). |
| R10 | `MAX_PHOTOS_PER_LISTING` dupliqué FE/BE. | Commentaire « Mirrors » comme les autres constantes ; le serveur reste l'autorité (409). |
| R11 | MCP en échec pendant l'analyse : PostgreSQL, Sequential Thinking, plugin GitHub (remplacé par `gh`). | Aucun besoin pour ce ticket FE. |
| R12 | `CLAUDE.md` cite `docs/ARCHITECTURE.md` / `TECHNICAL_SPEC_MVP.md` alors que les fichiers sont en `_v1.2`. | Hors périmètre, à signaler seulement. |
| R13 | Monétisation non définie. | Aucune dépendance pour ce ticket. |

---

## 7. Contrat qualité

1. **Orientation** : `graphify query`, `path` ou `explain` avant tout grep ou lecture. Relire ce plan et le ticket (`gh issue view 166`). Vérifier qu'on est sur `mh-59-build-the-listing-photo-management-screen` à jour (`git fetch`).
2. **Context7**, avant d'écrire le code qui utilise une librairie, pour vérifier bonnes pratiques, sécurité et idiome de la version installée :
   - TanStack Query v5 : `useMutation` (callbacks du hook vs de `mutate`, `mutateAsync` concurrents, R1), `setQueryData`, `invalidateQueries`, `isMutating` / `useIsMutating` avec `mutationKey` partielle ;
   - React Router v8 : `useParams`, `useNavigate`, routes `lazy` ;
   - React 19 : nettoyage des object URLs dans `useEffect`, état local sur promesse résolue après démontage ;
   - API `File` / `FormData` / `DataTransfer` côté navigateur (multipart, `accept`, `multiple`).
3. **Code minimal, KISS** : trois fonctions d'API, un module pur, trois hooks, trois composants, un lien et une route. Pas de nouvelle feature, pas de changement de `Modal`, pas d'abstraction d'upload générique.
4. **Fichier par fichier**, dans l'ordre de la section 2, en montrant chaque fichier avant de passer au suivant. Règles actives : `.claude/rules/react-typecrypt.md`, `general-coding.md`. Commentaires d'une ou deux lignes, seulement là où le code ne s'explique pas seul, sans citer un fichier de règles ou de doc comme justification.
5. **Tests unitaires sur la logique pure uniquement** (section 3) : aucune dépendance à la DB ni au réseau, aucun mock de la DB.
6. **Vérifications** de la section 5.
7. **Première review, avant tout fix** :
   - review dédiée par domaine : skill `code-review-frontend` (aucun changement backend, donc pas de `code-review-backend`) ;
   - reproduire chaque finding avant de le corriger, et comparer avec `HEAD` ;
   - **appliquer les corrections confirmées avant de continuer**, puis refaire l'étape 6.
8. **Passe `humanizer:humanizer`** sur les commentaires, doc comments, messages d'erreur, textes des tuiles, de la modale et de la zone d'ajout : ton humain, neutre, concis et fidèle au code.
9. **`graphify update .`**
10. **Mémoire** : noter la dette de tests de la section 3.
11. **Arrêt** : ni commit ni push. L'utilisateur relit, commit et ouvre la PR.

### Outils vérifiés pendant l'analyse

- **Skills du repo** (`.claude/skills/`) : `code-review-backend`, `code-review-frontend`, `graphify`, `github-ticket`, `docker`, `readme`.
- **Skills de plugins utilisés** : `humanizer:humanizer`, `run`.
- **MCP disponibles** : Context7 (`mcp__context7__*`, `mcp__claude_ai_Context7__*`), Git MCP (lecture seule).
- **MCP en échec pendant l'analyse** : PostgreSQL, plugin GitHub (remplacé par la CLI `gh`), Sequential Thinking.

---

## 8. Hors périmètre

Réordonnancement et renumérotation des positions (MH-60) ; publication depuis le détail (MH-61) ; barre de progression en pourcentage ; annulation d'un upload en cours ; blocage de la navigation pendant un upload ; recadrage ou compression côté client ; refactor du pré-contrôle d'owner-request ; changement du backend ou de l'OpenAPI ; tests d'intégration et de composants (epic dédiée).

---

## 9. Critères de fin

- [ ] Route `/owner/listings/:id/photos` : `/login` pour l'anonyme, `/` pour seeker et admin, « not found » pour un autre owner, même sur un bien public
- [ ] `OwnerBar` : « Next: add photos → » sans photo, « Manage photos » sinon
- [ ] Tuiles : « Cover » en texte sur la cover, « Make cover » toujours visible sur les autres
- [ ] Ajout par dropzone ou sélecteur, sélection multiple ; fichiers invalides rejetés avant upload avec leur nom ; fichiers valides envoyés quand même
- [ ] Un upload par fichier, tuile avec indicateur sans pourcentage ; échec sur la tuile (format, taille, quota, bien introuvable), autres photos intactes
- [ ] Jamais plus de 5 photos + uploads en cours ; fichiers en trop ignorés avec leur nombre ; message de limite à 5
- [ ] « Make cover » désactivé pendant la requête ; erreur → ancienne cover en place
- [ ] Suppression avec confirmation ; cover non supprimable tant que d'autres photos existent (indication, y compris sur 409) ; cover seule photo supprimable
- [ ] « Finish » désactivé pendant toute mutation photo, actif sinon même sans photo ; retour au détail
- [ ] Détail, « My properties » et feed à jour sans rechargement
- [ ] Pré-contrôle déplacé dans `shared/`, avatar inchangé à l'écran ; `FileDropzone` sans filtre silencieux
- [ ] Tests de la section 3 ; lint, ts:check, test et prettier propres
