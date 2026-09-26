# MH-53 — Créer un bien : plan d'implémentation (BE + FE)

- **Epic** : EP-09 — Listings (#129)
- **Ticket parent** : MH-53 — Create a listing (#130)
- **Sous-tickets** : MH-53-BE (#131) → MH-53-FE (#132). Le FE dépend du contrat livré par le BE.
- **Dépendances déjà livrées** : MH-32 (`AuthUser`, contrôle de rôle), MH-50 (upload photo, écran photo prévu en MH-58)

Ce plan couvre les deux sous-tickets. Chaque sous-ticket est traité dans sa propre session, sur sa propre branche, en suivant le déroulé de la section 5.

---

## 1. Décisions tranchées

| #  | Sujet                      | Décision                                                                                                                                                           | Raison                                                                                                                                                                                                                                                 |
| -- | -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| D1 | Format du 422              | Nouveau code`VALIDATION_FAILED`. L'enveloppe d'erreur reçoit un champ optionnel `fields: [{ field, message }]`, omis pour toutes les autres erreurs.           | Rétrocompatible avec l'enveloppe actuelle, ordre stable, se mappe directement sur`setError` côté front.                                                                                                                                           |
| D2 | Type du prix               | Entier XAF (`i64`), borné à `1..=9_999_999_999`. Écriture en SQL via `$n::numeric`. La lecture reste en `float8`, sans changement.                       | Le XAF n'a pas de centimes en usage courant. Aucun arrondi de float.`NUMERIC(12,2)` accepte au plus 9 999 999 999,99.                                                                                                                                |
| D3 | Validation                 | Écrite à la main, sans le crate`validator` ni macro. Un petit accumulateur `FieldErrors` dans `shared/validation.rs`, avec une méthode par type de règle. | Aucune dépendance ajoutée, cohérent avec`shared/validation.rs`. Une ligne par champ, chaque règle testable comme fonction pure. Une proc-macro imposerait un crate séparé et `syn`/`quote` ; une `macro_rules!` deviendrait un mini-DSL. |
| D4 | Redirection après succès | Vers`/listings/:id` (détail du bien) pour l'instant. MH-58 remplacera la cible par l'écran photos.                                                              | L'écran photos n'existe pas encore.                                                                                                                                                                                                                   |

### Décisions de conception prises pendant l'analyse (sans question)

- **Champ manquant = 422, pas 400.** Tous les champs du body sont `Option<_>` dans le DTO. Un champ absent donne une entrée `"<field> is required."` dans `fields`. Seul un JSON mal formé ou un type JSON primitif faux (`"price": "abc"`) reste un 400 via `AppJson`.
- **`type` invalide = 422.** `type` est désérialisé en `Option<String>`, puis converti en `ListingType`. Une valeur hors des six labels est une violation de règle, pas un JSON mal formé. Le schéma utoipa reste typé `ListingType` (`#[schema(value_type = ...)]`).
- **Bornes ville et quartier** : de 1 à 100 caractères après normalisation. Le ticket ne les donne pas, mais `VARCHAR(100)` renverrait sinon une erreur 500.
- **Le quartier est requis**, comme le dit le ticket. Le wireframe mh-14 le marque « facultatif » : le ticket, plus récent, l'emporte. La colonne reste nullable à cause des lignes du seed.
- **Libellé XAF** : le wireframe écrit « FCFA », le ticket et l'epic disent XAF. On retient **XAF**.
- **`owner_id` du body** : le DTO n'a pas de champ `owner_id` et n'utilise pas `deny_unknown_fields`, donc la valeur est ignorée silencieusement. Le propriétaire vient toujours de `AuthUser.user_id`.
- **Réponse 201** : `{ "data": ListingDetailDto }`, obtenue en réutilisant `service::get_listing_detail` après l'insertion. C'est le même format que `GET /listings/:id`, avec `media: []`. Le front réutilise le type existant.
- **Garde front** : on crée un nouveau `RequireOwner`, calqué sur `RequireAdmin`. `RequireAdmin` n'est pas refactoré en garde générique, car ce n'est pas demandé.
- **Texte de l'UI en anglais**, comme le reste de l'application.

---

## 2. Existant sur lequel on s'appuie

**Backend** (`backend/src/`)

- `modules/listings/` : lecture seule. `router.rs` contient encore `// TODO EP-03: owner create, update and delete.`. `model.rs` a `ListingType` et `ListingStatus` (`sqlx::Type`). Le prix est lu en `float8` parce que sqlx n'a ni `rust_decimal` ni `bigdecimal`.
- `shared/extractors.rs` : `AuthUser { user_id, role }` et `require_role(&[Role])` ; `AppJson<T>` transforme un rejet JSON en 400.
- `shared/rbac.rs` : rôles pairs, sans hiérarchie. Une route `owner` refuse un `admin`.
- `shared/errors.rs` : enveloppe `{ error: { code, message, status } }` ; les 5xx sont masqués.
- `shared/validation.rs` : fonctions `required_name` et assimilées, qui renvoient `AppError::BadRequest`.
- `modules/media/handler.rs` : modèle à suivre pour une route owner (`user.require_role(&[Role::Owner])?`, 201, doc utoipa avec 401/403).
- `api_doc.rs` : les chemins et schémas sont collectés automatiquement par `utoipa-axum`. Il faut seulement mettre à jour la description du tag `listings` (« Public read-only listings feed »).
- `.sqlx/` : cache offline sqlx utilisé. Toute nouvelle `query!` exige `cargo sqlx prepare`.

**Frontend** (`frontend/src/`)

- `shared/api/client.ts` : `ApiError { status, code, message }` ne lit pas encore de détail par champ.
- `app/RequireAdmin.tsx` : garde basée sur `useProfile().role`, qui redirige vers `/`. `RequireAuth` redirige vers `/login`.
- `features/listings/` : `api.ts` (`listListings`, `getListing`), `labels.ts` (`typeLabels`), `hooks/useListings.ts` (clés de cache `["listings", params]` et `["listing", id]`).
- `features/owner-request/components/OwnerRequestForm.tsx` : formulaire de référence (react-hook-form, `FormField`, `Input`, `Select`, `Alert`, `Button isLoading`).
- `shared/components/` : `FormField`, `Input`, `TextArea`, `Select`, `Alert`, `Card`, `DimensionRule`.
- Types générés : `cargo run --bin gen_openapi` écrit `frontend/docs-frontend/openapi.json`, puis `npm run generate:types:ci` produit `shared/api/types.ts`. Ce fichier ne se modifie jamais à la main.

**Dette connue à garder en tête** : `AuthUser` ne revalide pas le rôle à chaque requête. Un owner tout juste approuvé garde un jeton `seeker` jusqu'au prochain refresh. Pendant ce temps, la garde front, qui lit le rôle en base via le profil, le laisse passer, mais `POST /listings` renvoie 403. Le formulaire doit afficher un message clair pour ce cas (voir 4.4).

---

## 3. MH-53-BE — Création d'un bien

**Branche** : `mh-53-be-implement-listing-creation`, créée depuis `develop`.

### 3.1 Contrat

```
POST /api/v1/listings           (JWT, rôle owner uniquement)

Request
{
  "title": "Studio meublé Plateau",        // requis, 5..=120 caractères (après trim)
  "description": "…",                      // requis, 20..=2000 caractères (après trim)
  "type": "studio",                        // requis, l'un des 6 labels ListingType
  "price": 150000,                         // requis, entier XAF, 1..=9_999_999_999
  "city": "  Dakar ",                      // requis, normalisé, 1..=100 caractères
  "neighborhood": "Plateau   Nord",        // requis, normalisé, 1..=100 caractères
  "surface_m2": 35,                        // optionnel, 1..=100_000
  "rooms": 1                               // optionnel, 0..=100
}

201 → { "data": ListingDetailDto }        // status = "available", media = []
400 → JSON mal formé / type JSON primitif faux (BAD_REQUEST)
401 → jeton absent ou invalide
403 → rôle différent de owner (FORBIDDEN)
422 → { "error": { "code": "VALIDATION_FAILED", "message": "…", "status": 422,
                   "fields": [ { "field": "title", "message": "…" }, … ] } }
```

Normalisation de la ville et du quartier : `split_whitespace().collect::<Vec<_>>().join(" ")`. Cette expression supprime les espaces en début et en fin, et réduit toute suite d'espaces Unicode (tabulations, retours à la ligne, espaces insécables) à un seul espace.

### 3.2 Fichiers, dans l'ordre (un fichier à la fois)

1. **`shared/errors.rs`**

   - `pub struct FieldError { pub field: &'static str, pub message: String }` (`Serialize`, `Debug`).
   - Variante `Validation(Vec<FieldError>)`, avec la ligne `(StatusCode::UNPROCESSABLE_ENTITY, "VALIDATION_FAILED")` dans le bloc 422. Le message annonce le nombre de champs invalides.
   - `ErrorBody` reçoit `#[serde(skip_serializing_if = "Option::is_none")] fields: Option<Vec<FieldError>>`, rempli seulement par `Validation`.
   - Tests : un 422 porte `code`, `status` et un tableau `fields` avec une entrée par champ ; une autre erreur (par exemple `ListingNotFound`) n'a pas de clé `fields`.
2. **`shared/validation.rs`** (ajouts, sans toucher aux fonctions existantes)

   - `pub fn normalize_place_name(raw: &str) -> String`.
   - `pub struct FieldErrors(Vec<FieldError>)` avec :
     - `required_text(field, Option<String>, RangeInclusive<usize>) -> Option<String>` : trim, puis compte en `chars()` ;
     - `required_place_name(field, Option<String>, max) -> Option<String>` : normalise, puis rejette une valeur vide ou trop longue ;
     - `required_int(field, Option<i64>, RangeInclusive<i64>) -> Option<i64>` et `optional_int(...) -> Option<Option<i64>>`, ou une signature équivalente plus simple trouvée à l'implémentation ;
     - `push(field, message)` pour les règles propres à un module (le `type`) ;
     - `finish(self) -> Result<(), AppError>`, qui renvoie `Err(AppError::Validation(..))` si au moins une erreur a été accumulée.
   - Les commentaires de module sont mis à jour : le fichier ne sert plus seulement au profil.
   - Tests : trim, réduction des espaces (tabulation, retour à la ligne, U+00A0), valeur vide après normalisation, bornes de chaque méthode (limite, limite ±1), `finish` sans erreur, `finish` avec N erreurs.
3. **`modules/listings/model.rs`**

   - `impl FromStr for ListingType`, ou une fonction `ListingType::from_label` qui fait un `match` sur les 6 labels en minuscules.
   - `pub struct NewListing { title, description, listing_type, price: i64, city, neighborhood: String, surface_m2: Option<i32>, rooms: Option<i32> }`. Cette structure n'a **pas** de champ `owner_id`.
4. **`modules/listings/dto.rs`**

   - `CreateListingRequest` (`Deserialize`, `ToSchema`) : tous les champs en `Option`, `#[serde(rename = "type")]` sur `listing_type: Option<String>`, et `#[schema(required = true, value_type = ...)]` sur les champs requis pour que les types générés côté front restent justes. Pas de `owner_id`, pas de `deny_unknown_fields`.
   - La réponse réutilise `ListingDetailResponse`.
5. **`modules/listings/repository.rs`**

   - `insert_listing(pool, owner_id: Uuid, listing: &NewListing) -> Result<Uuid, AppError>` avec `sqlx::query_scalar!` : `INSERT … VALUES (…, $n::numeric, …) RETURNING id`. Le statut vient du `DEFAULT 'available'` de la colonne.
   - Consulter le MCP PostgreSQL avant d'écrire la requête, pour vérifier le schéma et les triggers `search_vector` et `updated_at`. Aucun nouvel index n'est ajouté.
6. **`modules/listings/service.rs`**

   - Constantes nommées pour les bornes (`TITLE_LENGTH`, `DESCRIPTION_LENGTH`, `PRICE_RANGE`, `SURFACE_RANGE`, `ROOMS_RANGE`, `PLACE_NAME_MAX_LENGTH`).
   - `pub fn validate_new_listing(request: CreateListingRequest) -> Result<NewListing, AppError>` : fonction pure, qui accumule toutes les erreurs avant de répondre. MH-55 la réutilisera pour la modification.
   - `pub async fn create_listing(pool, owner_id, request) -> Result<ListingDetailDto, AppError>` : valide, insère, puis appelle `get_listing_detail(pool, id)`.
7. **`modules/listings/handler.rs`**

   - `const LISTING_WRITE_ROLES: &[Role] = &[Role::Owner];`
   - `create(State, user: AuthUser, AppJson(payload)) -> Result<(StatusCode, Json<ListingDetailResponse>), AppError>` : `user.require_role(LISTING_WRITE_ROLES)?`, puis appel au service, puis réponse 201.
   - `#[utoipa::path(post, path = "/listings", tag = "listings", request_body = CreateListingRequest, responses(201, 400, 401, 403, 422))]`.
8. **`modules/listings/router.rs`**

   - `.routes(routes!(handler::list, handler::create))` : même chemin, deux méthodes, dans un seul `routes!`.
   - Le TODO est remplacé par ce qui reste réellement à faire (modification et suppression).
9. **`api_doc.rs`** : la description du tag `listings` devient « Listings feed, detail and owner management ».
10. **`.sqlx/`** : `cargo sqlx prepare` (base de dev lancée), pour que la nouvelle `query_scalar!` compile hors ligne.

### 3.3 Tests unitaires (logique pure, sans DB)

| Critère d'acceptation          | Test                                                                                                                                                                                         |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bornes de chaque champ          | `validate_new_listing` : title 4/5/120/121, description 19/20/2000/2001, price 0/1/max/max+1, surface 0/1/100 000/100 001, rooms −1/0/100/101, type inconnu, chaque champ requis absent   |
| Normalisation ville et quartier | `normalize_place_name` et `validate_new_listing` renvoient `"Plateau Nord"` ; une valeur faite uniquement d'espaces est rejetée                                                       |
| Une entrée par champ           | un body avec 3 champs invalides donne exactement 3`FieldError`, avec les bons noms de champ                                                                                                |
| Refus du rôle                  | `require_role(Role::Seeker, LISTING_WRITE_ROLES)` et `require_role(Role::Admin, …)` renvoient `Forbidden`                                                                             |
| `owner_id` ignoré            | un JSON contenant`owner_id` se désérialise en `CreateListingRequest` et passe la validation. `NewListing` n'a aucun champ owner : le propriétaire ne peut venir que de `AuthUser` |
| Format du 422                   | test d'enveloppe dans`errors.rs` (voir 3.2, point 1)                                                                                                                                       |

Hors unitaires (ils demandent une vraie DB) : insertion réelle, `status = available` en base, 201 de bout en bout. Ces cas sont vérifiés à la main via Swagger ou curl, puis notés dans la mémoire de dette de tests pour l'epic des tests d'intégration.

### 3.4 Vérifications BE

`cargo fmt`, `cargo check`, `cargo clippy` (règle rust.md). `cargo test` n'est lancé qu'avec l'accord explicite de l'utilisateur, ciblé sur les modules touchés (`cargo test --lib listings validation errors`). Vérifier dans Swagger (`/api/docs`) que `POST /listings` apparaît avec ses réponses 201, 403 et 422.

---

## 4. MH-53-FE — Formulaire « Publish a listing »

**Branche** : `mh-53-fe-build-listing-creation-form`, créée depuis `develop` une fois MH-53-BE mergé.

### 4.1 Préalable

Régénérer les types : `cargo run --bin gen_openapi` (depuis `backend/`), puis `npm run generate:types:ci`. `CreateListingRequest` doit apparaître dans `types.ts`.

### 4.2 Fichiers, dans l'ordre (un fichier à la fois)

1. **`shared/api/client.ts`**

   - `ErrorEnvelope.error.fields?: { field: string; message: string }[]`.
   - `ApiError.fieldErrors: readonly FieldError[]` (vide par défaut), rempli dans `sendWithRetry`. Les autres appelants ne changent pas.
2. **`features/listings/api.ts`**

   - `export type CreateListingRequest = components["schemas"]["CreateListingRequest"]`.
   - `createListing(body) => apiPost<{ data: ListingDetail }>("/api/v1/listings", body)`.
3. **`features/listings/listingFormValidation.ts`** (pur, testé)

   - Constantes des bornes, qui reprennent celles du backend (`TITLE_MIN_LENGTH`, …, `PRICE_MAX`).
   - `normalizePlaceName(raw)` : même transformation que le backend (`trim` puis `/\s+/g → " "`).
   - `toCreateListingPayload(values)` : trim des textes, normalisation de la ville et du quartier, et un champ numérique optionnel vide ou `NaN` est omis.
   - `requestErrorMessage(error)` : message du bandeau par code (`VALIDATION_FAILED`, `FORBIDDEN` avec le message du cas « rôle pas encore actif dans cette session, déconnecte-toi puis reconnecte-toi », avec repli générique). Le texte brut du serveur n'est jamais affiché dans le bandeau.
4. **`features/listings/hooks/useCreateListing.ts`**

   - `useMutation({ mutationFn: createListing, onSuccess: invalidate ["listings"] })`, pour que le nouveau bien apparaisse dans le feed.
5. **`features/listings/components/CreateListingForm.tsx`**

   - react-hook-form, avec les règles `required`, `minLength`, `maxLength`, `min`, `max` et `validate` (entier) reprises des constantes.
   - Champs : titre, description (`TextArea`), type (`Select` avec `typeLabels`), prix (libellé **« Price (XAF) »**, entier), surface (m², optionnel), pièces (optionnel), ville, quartier.
   - **Verrouillage pendant l'envoi** : tous les champs sont dans un `<fieldset disabled={mutation.isPending}>`, et le bouton a `isLoading`.
   - **Erreur 422** : pour chaque `error.fieldErrors`, `setError(field, { type: "server", message })`. Le bandeau `Alert` en haut du formulaire résume les erreurs. Un champ serveur inconnu du formulaire n'apparaît que dans le bandeau.
   - **Succès** : `navigate(`/listings/${data.id}`)`. Un commentaire d'une ligne indique que MH-58 remplacera cette cible par l'écran photos.
   - Mise en page d'après le wireframe mh-14 « Publier un bien », écran ①. L'indicateur d'étape « ② Photos » verrouillé est optionnel ; s'il est ajouté, il reste statique.
6. **`app/RequireOwner.tsx`** : copie du modèle `RequireAdmin`, avec `profile?.role !== "owner"` et une redirection vers `/`.
7. **`app/router.tsx`** : import lazy de `CreateListingForm`, route `owner/listings/new` sous `RootLayout` : `<RequireAuth><RequireOwner>{withSuspense(CreateListingForm)}</RequireOwner></RequireAuth>`. Un visiteur non connecté va vers `/login`, un seeker ou un admin vers `/`.
8. **`features/listings/index.ts`** : export de `CreateListingForm`, `createListing` et `useCreateListing`, selon ce qui est consommé hors de la feature.

### 4.3 Tests unitaires (vitest, logique pure)

`listingFormValidation.test.ts` :

- `normalizePlaceName` renvoie le même résultat que le backend sur les cas testés côté BE ;
- `toCreateListingPayload` : trim, champs optionnels vides ou `NaN` omis, aucun `owner_id` émis ;
- `requestErrorMessage` : `VALIDATION_FAILED`, `FORBIDDEN`, code inconnu, erreur autre qu'une `ApiError`.

Pas de test de composant : RTL n'est utilisé nulle part dans le projet. Cette dette est notée en mémoire, comme pour MH-49.

### 4.4 Vérifications FE

`npm run lint`, `npm run ts:check`, `npm run test`, `npm run prettier:check`. Vérification manuelle dans l'app avec le skill `/run` : owner (succès puis détail), 422 forcé (erreurs sous les champs et bandeau), seeker (redirection), anonyme (vers `/login`).

---

## 5. Déroulé imposé pour chaque sous-ticket

À suivre dans l'ordre, dans chaque session (BE puis FE).

1. **Branche** : créer la branche du sous-ticket depuis `mh-53-create-a-listing`.
2. **Orientation** : `graphify query "<question>"`, `graphify path` ou `graphify explain` avant tout grep de fichiers. Relire ce plan et le ticket GitHub.
3. **Documentation des librairies (Context7)** avant d'écrire le code qui les utilise :
   - BE : ordre des extracteurs dans axum 0.7 (le body en dernier) ; `query_scalar!` de sqlx 0.8 avec `RETURNING` et le cast `numeric` ; `#[schema(required, value_type)]` de utoipa 5 ; `routes!` de utoipa-axum avec plusieurs méthodes.
   - FE : `setError` et `valueAsNumber` de react-hook-form 7 ; `useMutation` et `invalidateQueries` de TanStack Query v5 ; `Navigate` et `useNavigate` de react-router v8.
4. **Implémentation fichier par fichier**, dans l'ordre des sections 3.2 et 4.2, en montrant chaque fichier avant de passer au suivant. Règles actives :
   - BE : `.claude/rules/rust.md`, `.claude/rules/database.md`, `.claude/rules/general-coding.md`.
   - FE : `.claude/rules/react-typecrypt.md`, `.claude/rules/general-coding.md`.
   - Commentaires : une ou deux lignes au plus, uniquement là où le code ne s'explique pas seul, sans citer de fichier de règles ou de doc comme justification.
5. **Tests unitaires** : la logique pure (sections 3.3 et 4.3). Pas de mock de DB.
6. **Vérifications** : sections 3.4 et 4.4.
7. **`/simplify`** sur le diff : réutilisation, simplicité, KISS. Appliquer ses corrections.
8. **`/humanizer:humanizer`** sur les commentaires, doc comments et messages d'erreur ajoutés : ton neutre, concis, fidèle au code, sans formule vague ni verbeuse.
9. **Première review** :
   - BE : skill `code-review-backend` ; FE : skill `code-review-frontend` ;
   - `/humanizer:humanizer` en mode review sur les mêmes commentaires ;
   - reproduire chaque finding avant de le corriger, et comparer avec `HEAD`, car d'autres processus peuvent modifier l'arbre de travail ;
   - **corriger les findings confirmés**, puis relancer l'étape 6.
10. **`graphify update .`** pour garder le graphe à jour.
11. **Mémoire** : noter la dette de tests (cas qui demandent une DB, absence de test de composant côté FE).
12. **Arrêt** : pas de commit ni de push. L'utilisateur relit, commit et ouvre la PR.

---

## 6. Hors périmètre

Modification et suppression d'un bien (MH-55 et suivants), écran photos (MH-58), liste « Mes biens », bascule de statut, filtres du feed par quartier et disponibilité (MH-59), lien d'accès au formulaire dans le header, affichage de la devise XAF dans le feed et le détail, revalidation du rôle à chaque requête.

---

## 7. Critères de fin

**MH-53-BE**

- [ ] `POST /listings` réservé au rôle owner ; `owner_id` jamais lu dans le body
- [ ] Bornes de tous les champs appliquées ; ville et quartier normalisés, valeur vide rejetée
- [ ] Un 422 avec une entrée par champ invalide
- [ ] Le bien est créé avec le statut `available` ; réponse 201 avec l'`id`
- [ ] Endpoint dans le schéma utoipa, avec 403 et 422
- [ ] Tests unitaires du tableau 3.3 écrits ; fmt, check et clippy propres ; `.sqlx` à jour

**MH-53-FE**

- [ ] Tous les champs présents, avec les bornes vérifiées côté client
- [ ] Prix saisi en XAF et libellé en XAF
- [ ] Champs verrouillés pendant la requête
- [ ] 422 : erreur sous chaque champ et bandeau récapitulatif en haut
- [ ] Succès : redirection vers le détail du nouveau bien (cible provisoire jusqu'à MH-58)
- [ ] Route owner protégée : seeker et admin renvoyés vers `/`, anonyme vers `/login`
- [ ] Tests vitest de 4.3 ; lint, ts:check, test et prettier propres
