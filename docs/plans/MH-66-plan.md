# MH-66 — Libellés de type anglais dans l'index de recherche : plan d'implémentation (BE)

- **Epic** : EP-10 — Search (pas d'issue d'epic)
- **Ticket** : MH-66 — Index English type labels for keyword search (#189). BE uniquement, pas de sous-ticket.
- **Branche** : `mh-66-index-english-type-labels-for-keyword-search` (en local et sur origin, à jour avec `develop` après le merge de MH-65). Aucun commit ni push automatique.
- **Déjà livré, et utilisé ici** : MH-20 (`fn_update_listing_search_vector`, `tg_listings_search_vector`, `tg_listings_updated_at`, index GIN `idx_listings_search`), MH-64-BE (`GET /api/v1/search`, `push_keyword_query`, paliers de pertinence).
- **CDC** : SRCH-01 (recherche full-text sur le libellé du bien).

---

## 1. Contexte et objectif

L'interface affiche les types en anglais (`typeLabels` dans `frontend/src/features/listings/labels.ts` : Apartment, Studio, House, Room, Villa, Other). Le `search_vector` ne contient que les libellés français, produits par le `CASE` de `fn_update_listing_search_vector` (migration `20260729044830_triggers_functions.sql`) : appartement, studio, maison, chambre, villa, autre. Un visiteur qui tape `apartment` ou `house` ne trouve donc rien.

**Objectif** : le `search_vector` de chaque annonce contient aussi le libellé anglais de son type, avec le même poids B que le libellé français. Les annonces existantes sont réindexées par la migration elle-même. Les autres champs et leurs poids ne changent pas. Le changement passe par une nouvelle migration, sans modifier les anciennes.

### Décisions prises

| #  | Sujet | Décision |
| -- | ----- | -------- |
| D1 | Où ajouter le libellé | Dans le `CASE`, chaque branche renvoie les deux libellés dans une seule chaîne : `appartement apartment`, `maison house`, `chambre room`, `autre other`. `studio` et `villa` ne changent pas (même mot dans les deux langues). Le libellé passe toujours par un seul `setweight(..., 'B')` : même poids, et les autres champs restent intacts. |
| D2 | Mise à jour de la fonction | `CREATE OR REPLACE FUNCTION fn_update_listing_search_vector()`, avec le corps repris à l'identique sauf le `CASE`. Le trigger reste lié à la fonction : pas de `DROP` / `CREATE TRIGGER`. |
| D3 | Réindexation (choix validé : préserver `updated_at`) | Dans la même migration : `ALTER TABLE listings DISABLE TRIGGER tg_listings_updated_at`, puis `UPDATE listings SET type = type` (le trigger BEFORE UPDATE recalcule le vecteur), puis `ENABLE TRIGGER`. sqlx exécute chaque migration dans une transaction (`no_tx = false` par défaut) : un échec annule tout, désactivation du trigger comprise. |
| D4 | Configuration texte | Les mots anglais passent par `french` + `unaccent`, comme le reste de l'index et la requête (`websearch_to_tsquery('french', unaccent(...))`). Vérifié en psql : chaque mot anglais et chaque mot français trouve le libellé bilingue (`apartment` reste `apartment`, `appartement` devient `appart`, `house` devient `hous`). Les pluriels anglais ne sont pas garantis (accepté par le ticket). |
| D5 | Code Rust | Aucun changement. La requête, les DTO et l'OpenAPI ne bougent pas, d'où pas de `cargo sqlx prepare`. |
| D6 | Documentation | TECHNICAL_SPEC §2.2 et le texte du trigger dans la DDL restent dans la dette MH-68. |

---

## 2. Fichiers, dans l'ordre (un fichier à la fois)

1. **`docs/plans/MH-66-plan.md`** : ce plan.
2. **`backend/migrations/<horodatage>_search_vector_english_type_labels.sql`** (nouveau, `sqlx migrate add`) :
   - `CREATE OR REPLACE FUNCTION` avec le `CASE` bilingue (D1, D2) ;
   - désactivation de `tg_listings_updated_at`, `UPDATE listings SET type = type`, réactivation (D3).

---

## 3. Tests unitaires

Aucune logique pure n'est ajoutée côté Rust, donc aucun test unitaire. Les comportements SQL (contenu du vecteur, réindexation, `updated_at` préservé, correspondance des mots-clés) demandent une vraie DB : ils sont vérifiés en psql et en curl (section 6) et notés comme dette pour l'epic des tests d'intégration.

---

## 4. Points d'intégration avec l'existant

| Zone | Existant | Changement MH-66 |
| ---- | -------- | ---------------- |
| `fn_update_listing_search_vector` | `CASE` qui donne le libellé français | `CASE` bilingue. Le reste du corps est identique. |
| `tg_listings_search_vector` (BEFORE INSERT OR UPDATE) | Appelle la fonction | Inchangé. Il appelle la nouvelle version. |
| `tg_listings_updated_at` (BEFORE UPDATE) | Met `updated_at` à `NOW()` | Désactivé seulement pendant la réindexation de la migration. |
| `tg_users_cascade_search` | `SET updated_at = NOW()` sur les annonces de l'owner renommé | Inchangé. Il recalcule toujours le vecteur, avec les deux libellés. |
| `idx_listings_search` (GIN) | Index sur `search_vector` | Inchangé, mis à jour par l'`UPDATE`. |
| `GET /api/v1/search`, `push_keyword_query`, paliers `ts_filter` | Le type est en poids B | Inchangés. Un mot de type anglais reste au palier B. |
| `backend/src/infra/db.rs` | `sqlx::migrate!()` au démarrage | La migration s'applique au prochain lancement. |

---

## 5. Risques et inconnues

| #  | Risque / inconnue | Traitement |
| -- | ----------------- | ---------- |
| R1 | Stemmer français appliqué à des mots anglais. | Vérifié avant de coder (D4) : les correspondances sont exactes pour les six types. |
| R2 | `DISABLE TRIGGER` : verrou `ACCESS EXCLUSIVE` sur `listings`, droits de propriétaire. | Négligeable au volume MVP. `myhouse` est propriétaire de la table et c'est l'utilisateur des migrations. |
| R3 | La réindexation recalcule tous les champs du vecteur. | Comparaison avant/après : seuls les lexèmes du type changent. |
| R4 | Le mot `other` trouve toutes les annonces de type `other`. | Voulu par le ticket. |
| R5 | Aucune annonce de type `other` en base de dev. | `INSERT` dans une transaction annulée pour vérifier ce type et le chemin INSERT du trigger. |
| R6 | MCP PostgreSQL en échec dans la session précédente. | `psql` via `docker exec mh-db`. |
| R7 | Monétisation non définie. | Aucune dépendance pour ce ticket. |

---

## 6. Vérifications

- Instantané avant migration : `id, type, updated_at, search_vector`.
- Migration appliquée au démarrage du backend :
  - `_sqlx_migrations` : nouvelle version, `success = true` ;
  - vecteurs : lexèmes français et anglais en poids B pour chaque type ;
  - autres lexèmes identiques à l'instantané ;
  - `updated_at` identique pour toutes les annonces ;
  - `tg_listings_updated_at` réactivé (`tgenabled = 'O'`), et un `UPDATE` de test fait toujours avancer `updated_at` (dans une transaction annulée).
- `INSERT` d'une annonce `other` dans une transaction annulée : son vecteur contient `autr` et `other`.
- Curl `GET /api/v1/search` : `apartment` / `appartement`, `house` / `maison`, `room` / `chambre`, `studio`, `villa` → mêmes annonces publiques pour les deux mots.
- `cargo fmt --check`, `cargo check`, `cargo clippy`.

---

## 7. Contrat qualité

1. **Orientation** : `graphify query`, `gh issue view 189`, branche vérifiée.
2. **Context7 / doc PostgreSQL 16 et sqlx 0.8** : `ALTER TABLE … DISABLE / ENABLE TRIGGER` (verrou, droits), transaction par migration, `CREATE OR REPLACE FUNCTION`.
3. **KISS** : une migration, un `CASE` modifié, trois instructions de réindexation, aucun changement Rust.
4. **Tests unitaires sur la logique pure uniquement** : aucun ici (section 3).
5. **Vérifications** de la section 6.
6. **Première review, avant tout fix** : skill `code-review-backend`. Reproduire chaque finding, comparer avec `HEAD`, appliquer les corrections confirmées avant de continuer, puis refaire la section 6.
7. **Passe `humanizer:humanizer`** sur les commentaires SQL.
8. **`graphify update .`**, note de dette de tests en mémoire.
9. **Arrêt** : ni commit ni push.

### Outils vérifiés

- **Skills du repo** : `code-review-backend` et `graphify` (utilisés) ; `code-review-frontend`, `github-ticket`, `docker`, `readme` (non utilisés ici).
- **Règles du repo** : `database.md`, `rust.md`, `general-coding.md`.
- **Skills de plugins** : `humanizer:humanizer`.
- **MCP** : Context7. PostgreSQL MCP remplacé par `psql` dans le conteneur.

---

## 8. Hors périmètre

Titres et descriptions en anglais ; index bilingue complet (configuration `english` en plus) ; pluriels anglais ; mise à jour de TECHNICAL_SPEC et d'ARCHITECTURE (MH-68) ; tests d'intégration (epic dédiée).

---

## 9. Critères de fin

- [ ] Le vecteur de chaque annonce contient le libellé anglais de son type, en poids B
- [ ] `apartment` / `appartement`, `house` / `maison`, `room` / `chambre`, `other` / `autre`, `studio`, `villa` trouvent les mêmes annonces
- [ ] Annonces existantes réindexées par la migration, sans étape manuelle ; `updated_at` préservé
- [ ] Autres champs et poids inchangés
- [ ] Nouvelle migration, anciennes migrations intactes
- [ ] fmt, check et clippy propres
