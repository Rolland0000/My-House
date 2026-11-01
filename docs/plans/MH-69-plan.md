# MH-69 — Révélation des numéros de l'owner : plan d'implémentation (BE + FE)

- **Epic** : EP-11 — Contact (pas d'issue d'epic)
- **Tickets** : MH-69 (#198), MH-69-BE (#199), MH-69-FE (#200, dépend de MH-69-BE)
- **Branches** :
  - BE : `mh-69-be-add-the-get-listingsidcontact-endpoint` (origin, à jour avec `develop`)
  - FE : pas encore créée. À récupérer ou demander avant de coder.
  - Ni commit ni push automatique.
- **CDC** :
  - DET-02 : pas de téléphone dans les données publiques.
  - DET-03 : numéro révélé seulement aux utilisateurs connectés, via « Contacter ».
- **Documentation** (spec §1.1/§1.2/§4.6/§8, ARCHITECTURE §6.3, CDC DET-03) : traitée par MH-71, rien n'est modifié ici.

---

## 1. Contexte et objectif

La page détail montre l'owner (nom, avatar) mais aucun moyen de le joindre. Le téléphone est le seul canal de contact du MVP. Le ticket ajoute :

- **BE** : `GET /api/v1/listings/{id}/contact`, dans un nouveau module `contact`. Il renvoie `phone` (profil de l'owner) et `secondary_phone` (dernière demande owner approuvée) à un seeker ou un owner connecté, seulement pour un bien public (disponible ou non) dont l'owner est actif.
- **FE** : bouton « Contact the owner » dans la section « The owner », avec révélation des numéros et actions Call, Copy et WhatsApp, plus un indice sur le formulaire de demande owner.

### Décisions

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Accès aux données | Une seule requête `QueryBuilder` dans `contact/repository.rs` : `listings l JOIN users u` + `LEFT JOIN LATERAL` sur la dernière demande `approved` (`ORDER BY reviewed_at DESC NULLS LAST, created_at DESC LIMIT 1`). Elle réutilise `listings::repository::push_public_visibility` comme le prévoit son commentaire. Elle donne un instantané cohérent en un aller-retour. |
| D2 | Conditions de révélation | `l.id = $1 AND u.is_active AND u.phone IS NOT NULL`, plus `push_public_visibility`. Toute condition non remplie donne `AppError::ListingNotFound`, identique à un id inconnu. Le statut `available` / `unavailable` n'entre pas en jeu (décision du 2026-11-01, contraire à l'AC initial de MH-69-BE) : un bien `unavailable` reste contactable. |
| D3 | Rôles | `AuthUser` (401, et 401 `ACCOUNT_SUSPENDED` pour un compte suspendu) puis `require_role(&[Seeker, Owner])` (403 pour un admin), en tête de handler, comme `OWNER_ROLES` dans `listings/handler.rs`. L'owner de l'annonce passe (AC BE). |
| D4 | Limite par utilisateur (choix validé, hors AC initiaux du BE) | Compteur à fenêtre fixe par `user_id`, sur le modèle de `middleware/rate_limit.rs` : `Arc<AtomicU32>` dans un `AppCacheProvider<Uuid, _>` moka dont le TTL vaut la fenêtre. Chaque appel d'un seeker ou d'un owner compte, 404 compris, pour freiner le balayage d'ids. Le dépassement renvoie `AppError::RateLimited` (429 `RATE_LIMITED`) avec un `tracing::warn!` sur `user_id`. Configuration : `CONTACT_REVEAL_MAX_PER_WINDOW=20` et `CONTACT_REVEAL_WINDOW_SECONDS=3600` (valeurs à confirmer), obligatoires et > 0 comme `RATE_LIMIT_*`. |
| D5 | Id malformé | Même `Path<Uuid>` que `GET /listings/{id}`, donc même rejet Axum (400 en texte brut). Rien à ajouter. |
| D6 | Log | Après un succès : `tracing::info!(user_id = %…, listing_id = %…, "owner contact revealed")`. Jamais de numéro dans les logs. |
| D7 | Cache FE | `useQuery(["listing-contact", id])` avec `enabled` piloté par le clic, `staleTime: Infinity`, `gcTime: Infinity` et `retry: false`. Une deuxième révélation dans la même session ne refait pas d'appel. Un rechargement vide la mémoire. Une fonction exportée par `features/contact` purge la clé à la déconnexion (appelée dans `RootLayout.signOutTo`, à côté de `removeOwnerScopedQueries`). |
| D8 | Affichage du bouton | Rien n'est affiché pendant `bootstrapping` ni pendant le chargement du profil (pas de flash, comme D3 de MH-67). Un visiteur anonyme voit « Sign in to contact », un lien vers `/login` (le retour vers l'annonce sera traité par un ticket dédié). Un connecté qui n'est ni admin (`profile.role`) ni l'owner (`profile.id`) voit « Contact the owner ». Le bouton reste visible et actif quel que soit le statut du bien (décision du 2026-11-01, contraire à l'AC initial de MH-69-FE qui le désactivait sur un bien `unavailable`). L'`Alert` « This property is currently unavailable. » existante suffit à signaler le statut. |
| D9 | Erreurs FE | `contactErrorKind(error)` dans un fichier de logique pure, sur le modèle de `admin/reviewErrorMessage.ts` : `not-found` (404), `rate-limited` (429, le bouton reste désactivé jusqu'au remount), `unauthorized` (401 après échec du refresh : `client.ts` a déjà appelé `clearSession`, le statut passe à `anonymous` et le bouton « Sign in to contact » revient de lui-même), sinon `other` (message et Retry). |
| D10 | Liens | `telHref(phone)` donne `tel:` suivi du numéro sans espaces. `whatsappHref(phone)` donne `null` si le numéro ne commence pas par `+`, sinon `https://wa.me/{chiffres seuls}`, ouvert avec `target="_blank" rel="noopener noreferrer"`. Ces fonctions pures vivent dans `features/contact/phoneLinks.ts`. |
| D11 | Copie | `navigator.clipboard.writeText` dans un try/catch. En cas de succès, le libellé passe à « Copied » pendant `COPIED_FEEDBACK_MS`. En cas d'échec ou d'API absente, un message le signale et le numéro reste du texte sélectionnable. |
| D12 | Accessibilité | Le bloc révélé est un `role="status"` qui annonce les numéros. Le focus passe sur ce bloc (`tabIndex={-1}`) puisque le bouton disparaît. Les `aria-label` portent le numéro : « Call {n} », « Copy {n} », « Open WhatsApp chat with {n} ». |
| D13 | Message de sécurité | Il est déplacé de `ListingDetail.tsx` vers `ContactReveal`, juste sous le bouton ou sous les numéros, avec le texte et le `role="note"` actuels. Il reste visible pour l'owner, l'admin et pendant le bootstrap : il est rendu hors des conditions du bouton. |
| D14 | Indice du formulaire owner | Téléphone principal : « Signed-in users can see these numbers on your listings. ». Second numéro : « Optional — only sent if filled in. Signed-in users can see these numbers on your listings. ». |

---

## 2. Découpage BE / FE et dépendances

- **MH-69-BE** n'a pas de dépendance. Il livre le contrat OpenAPI.
- **MH-69-FE** dépend du BE : `shared/api/types.ts` est régénéré à partir de `frontend/docs-frontend/openapi.json` produit par le BE. Le FE peut démarrer sur la logique pure (D9, D10) avant le merge, mais l'intégration attend le BE.
- **Contrat** : `200 { "data": { "phone": string, "secondary_phone": string | null } }`, ainsi que 401, 403, 404 `LISTING_NOT_FOUND` et 429 `RATE_LIMITED`.

---

## 3. Ordre d'implémentation

### MH-69-BE

1. `shared/errors.rs` : aucun changement (`ListingNotFound`, `Forbidden`, `RateLimited` existent).
2. Config :
   - `config/mod.rs` : `contact_reveal_max_per_window: u32` et `contact_reveal_window_seconds: u64`, validés > 0 comme `rate_limit_*`.
   - `backend/.env.example` : les deux variables.
   - Toute autre source d'environnement qui déclare `RATE_LIMIT_*` (docker-compose, CI) est alignée.
3. `infra/cache/mod.rs` : `build_contact_reveal_rate_limit_cache(window)` avec un plafond d'entrées en constante, un champ et un accesseur sur `AppCache`, un paramètre de `AppCache::new`. Puis `main.rs` et les tests qui construisent `AppCache`.
4. `modules/contact/` :
   - `mod.rs` déclare `dto`, `handler`, `repository`, `router` et `service`.
   - `dto.rs` : `ContactDto { phone, secondary_phone: Option<String> }` et `ContactResponse { data }`, en `ToSchema`.
   - `repository.rs` : `find_owner_contact(pool, listing_id) -> Result<Option<ContactRow>, AppError>` (D1, D2).
   - `service.rs` :
     - `allow_reveal(counters, user_id, max) -> bool` (D4) ;
     - `reveal_owner_contact(pool, listing_id)`, qui transforme `None` en `ListingNotFound`.
   - `handler.rs` : `#[utoipa::path(get, path = "/listings/{id}/contact", tag = "contact", …)]` avec les réponses 200, 401, 403, 404 et 429. Ordre : extracteur, rôle, limite, service, log.
   - `router.rs` : `pub fn router() -> OpenApiRouter<AppState>`.
5. `modules/mod.rs` déclare `contact`. `route.rs` ajoute `.merge(contact::router::router())`. `api_doc.rs` ajoute le tag `contact`.
6. `cargo run --bin gen_openapi -- ../frontend/docs-frontend/openapi.json`. Pas de `cargo sqlx prepare`, puisque `QueryBuilder` n'utilise pas le cache offline.

### MH-69-FE

1. `npm run generate:types` (ou `:ci`) pour obtenir le chemin `/api/v1/listings/{id}/contact` et `ContactDto`.
2. `features/contact/api.ts` : `getListingContact(id)` via `apiGet`, plus la clé de requête et la fonction de purge (D7).
3. `features/contact/phoneLinks.ts` et `contactError.ts` (D9, D10), avec leurs `.test.ts`.
4. `features/contact/hooks/useListingContact.ts` (D7).
5. `features/contact/components/ContactReveal.tsx` : props `listingId`, `ownerId` et `status`. Il porte D8, D9, D11, D12 et D13.
6. `features/contact/index.ts` exporte `ContactReveal` et la purge.
7. `features/listings/components/ListingDetail.tsx` : `<ContactReveal …/>` remplace le bloc du message de sécurité.
8. `app/layout/RootLayout.tsx` : appel de la purge à la déconnexion.
9. `features/owner-request/components/OwnerRequestForm.tsx` : les indices de D14.

---

## 4. Points d'intégration

| Zone | Existant | Changement |
| ---- | -------- | ---------- |
| `listings/repository.rs` | `push_public_visibility` (`pub`, alias `l`, préfixe `WHERE`) | Réutilisé tel quel par `contact` |
| `owner_requests` | `secondary_phone` nullable, `reviewed_at` posé à l'approbation | Lu par la requête LATERAL de `contact`. Le module `owner_requests` n'est pas modifié. |
| `shared/extractors.rs` | `AuthUser` (contrôle `is_active` avec un cache moka de 8 s) | Utilisé tel quel |
| `shared/rbac.rs` | `require_role` → `Forbidden` | Utilisé tel quel |
| `middleware/rate_limit.rs` | Compteur à fenêtre fixe par IP | Reste en place globalement ; son modèle est repris par utilisateur (D4) |
| `infra/cache`, `config`, `main.rs` | Caches et variables `RATE_LIMIT_*` | Nouveau cache et deux variables |
| `route.rs` / `api_doc.rs` | Fusion des routers sous `/api/v1`, tags | Router et tag `contact` |
| `ListingDetail.tsx` | Section « The owner » et `Alert role="note"` | `ContactReveal` remplace l'`Alert` |
| `shared/api/client.ts` | `ApiError`, refresh sur 401 puis `clearSession` | Utilisé tel quel |
| `RootLayout.signOutTo` | `removeOwnerScopedQueries` | Ajout de la purge du contact |
| `OwnerRequestForm.tsx` | `FormField hint` | Indices modifiés |

---

## 5. Risques et inconnues

- **R1** : deux décisions s'écartent des tickets. Il faut mettre à jour les issues et ajouter le §8 de la spec à la dette MH-71.
  - La limite par utilisateur (D4) dépasse les AC de MH-69-BE : ajouter à #199 l'AC du 429 et les variables d'environnement. Les valeurs 20 / 3600 s restent à confirmer.
  - Le contact sur un bien `unavailable` (D2, D8) : retirer la condition `available` de #199 et le bouton désactivé de #200.
- **R2** : `users_profile_complete_for_non_admin` est `NOT VALID`, donc des comptes anciens peuvent avoir `phone` NULL. Ils sont exclus par D2 (404) plutôt que de casser le contrat `phone: string`.
- **R3** : `owner_requests` n'a pas d'index sur `user_id` hors la contrainte partielle `pending`. La LATERAL lit une table petite au MVP. Un `EXPLAIN` est à faire via `psql` (le MCP PostgreSQL ne s'est pas connecté dans cette session) sans ajouter d'index au passage.
- **R4** : l'architecture veut que les modules passent par les services. La requête unique (D1) lit `users` et `owner_requests` directement. L'écart est assumé pour la cohérence et le nombre d'allers-retours, et sera signalé dans MH-71.
- **R5** : le compteur est en mémoire, par instance, et remis à zéro au redémarrage. C'est acceptable au MVP (une instance, pas de Redis).
- **R6** : l'état « rate-limited » du FE vit dans le composant. Une navigation aller-retour le réactive, alors que l'AC dit « jusqu'au rechargement ». Le BE renverra simplement un nouveau 429.
- **R7** : le retour vers l'annonce après connexion est absent. Un ticket dédié le traitera.
- **R8** : le FE n'a ni React Testing Library ni jsdom, donc aucun test de composant. La dette sera notée en mémoire.

---

## 6. Vérifications

**BE**
- `cargo fmt`, `cargo check` et `cargo clippy`.
- Tests unitaires :
  - `allow_reveal` : N appels passent, l'appel N+1 est refusé, deux utilisateurs restent indépendants ;
  - garde de rôle : seeker et owner passent, admin donne `Forbidden`.
- Curl scripté sur la stack locale :
  - 401 sans jeton, 401 avec un jeton expiré, 401 `ACCOUNT_SUSPENDED` ;
  - 403 pour l'admin ;
  - 200 pour un seeker, avec et sans `secondary_phone` ;
  - 200 pour l'owner sur son propre bien ;
  - 200 pour un bien `unavailable` publié avec photo ;
  - 404 pour un brouillon, un bien sans photo, un owner inactif et un id inconnu, avec des corps identiques ;
  - id malformé identique à `GET /listings/{id}` ;
  - 429 après N appels ;
  - log sans numéro.
- `docs/openapi.json` sans dérive.

**FE**
- `npm run lint`, `ts:check`, `test` et `prettier:check`.
- Tests vitest : `telHref`, `whatsappHref` (avec `+`, sans `+`, espaces) et `contactErrorKind` (404, 429, 401, autre).
- Playwright sur l'app réelle (connexion OTP via Mailhog) :
  - anonyme, seeker, owner de l'annonce, admin ;
  - bien `unavailable` : bouton visible et révélation fonctionnelle ;
  - révélation, cache sans second appel, Copy, liens tel et wa.me ;
  - 404 et 429 ;
  - déconnexion puis reconnexion sans numéro en cache.

---

## 7. Contrat qualité (sur chaque branche)

- **Context7** pour vérifier :
  - BE : `QueryBuilder` de sqlx, `utoipa::path` et `OpenApiRouter`, moka ;
  - FE : options `useQuery` v5, Clipboard API.
- **KISS** : code minimal, aucune librairie ajoutée.
- **Tests** : unitaires sur la logique pure uniquement, sans dépendance à la DB.
- **Première review** avant tout fix (`code-review-backend` ou `code-review-frontend`, skills du repo). Chaque finding est reproduit, et le fix est appliqué avant de continuer.
- **Review dédiée par domaine**, puis une passe `humanizer:humanizer` sur les commentaires et les textes visibles : ton humain, neutre, concis et fidèle au code.
- **Après le code** : `graphify update .` et note de dette de tests en mémoire.
- Ni commit ni push.
