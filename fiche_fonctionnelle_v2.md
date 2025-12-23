# Fiche Fonctionnelle - Plateforme Location v2.0

## 1. Architecture du Système

```
                            Admin Système
                                 |
         -------------------------------------------------
         |                      |                        |
    Visiteur/            Bailleurs              Support & Modération
    Locataire         (Propriétaires)
         |                      |
         v                      v
  • Recherche filtrée    • Gestion des biens
  • Consultation         • Tableau de bord
  • Messagerie           • Statistiques
  • Favoris              • Réponses rapides
```

---

## 2. Informations Générales

| Élément | Détail |
|---------|--------|
| **Nom du projet** | Location - Plateforme Web de Location Immobilière |
| **Objectif** | Mettre en relation locataires et bailleurs de manière simple et moderne |
| **Public cible** | Particuliers cherchant à louer ou proposer un logement |
| **Plateformes** | Web responsive (mobile-first) + Desktop |
| **Stack Technique** | **Frontend**: React 18 + TypeScript + Tailwind CSS + Vite<br>**Backend**: Rust + Axum + SQLx + Tower<br>**BDD**: PostgreSQL 15<br>**Cache**: Redis (optionnel)<br>**Conteneurisation**: Docker + Docker Compose |
| **Sécurité** | JWT Authentication, HTTPS, CORS configuré, Rate Limiting, Validation stricte |
| **Performance** | Lighthouse Score > 90, Code Splitting, Lazy Loading |
| **SEO** | Meta tags optimisées, Sitemap.xml, Structure sémantique HTML5 |

---

## 3. Stack Technique Détaillée

### Backend - Rust + Axum
**Pourquoi Axum ?**
- Plus moderne et ergonomique qu'Actix
- Basé sur Tokio (async runtime performant)
- Type-safe routing
- Excellent support des middlewares
- Meilleure intégration avec Tower (middleware ecosystem)

**Dépendances principales:**
```toml
axum = "0.7"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "fs"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
jsonwebtoken = "9.0"
bcrypt = "0.15"
validator = "0.16"
chrono = "0.4"
dotenv = "0.15"
```

### Frontend - React + TypeScript
**Librairies:**
```json
{
  "react": "^18.2.0",
  "react-router-dom": "^6.20.0",
  "axios": "^1.6.0",
  "tailwindcss": "^3.3.0",
  "lucide-react": "^0.300.0",
  "react-hook-form": "^7.49.0",
  "zod": "^3.22.0",
  "zustand": "^4.4.0"
}
```

---

## 4. Fonctionnalités MVP (Minimum Viable Product)

### Phase 1 - Authentification & Comptes
| Priorité | Fonctionnalité | Description |
|----------|----------------|-------------|
| 🔴 HAUTE | Inscription Visiteur | Formulaire avec validation email |
| 🔴 HAUTE | Inscription Bailleur | Formulaire avec infos supplémentaires |
| 🔴 HAUTE | Connexion | JWT + Refresh Token |
| 🟡 MOYENNE | Mot de passe oublié | Email de réinitialisation |
| 🟡 MOYENNE | Profil utilisateur | Modification infos personnelles |

### Phase 2 - Gestion des Logements
| Priorité | Fonctionnalité | Description |
|----------|----------------|-------------|
| 🔴 HAUTE | Création annonce | Formulaire multi-étapes avec photos |
| 🔴 HAUTE | Liste des logements | Affichage avec pagination |
| 🔴 HAUTE | Détails logement | Page complète avec galerie photos |
| 🟡 MOYENNE | Modification annonce | Édition par le bailleur |
| 🟡 MOYENNE | Suppression annonce | Soft delete avec confirmation |

### Phase 3 - Recherche & Filtres
| Priorité | Fonctionnalité | Description |
|----------|----------------|-------------|
| 🔴 HAUTE | Recherche textuelle | Recherche par ville, quartier |
| 🔴 HAUTE | Filtres avancés | Type, prix min/max, chambres |
| 🟡 MOYENNE | Tri des résultats | Prix, date, pertinence |
| 🟢 BASSE | Sauvegarde recherche | Alertes email pour nouvelles annonces |

### Phase 4 - Messagerie & Contact
| Priorité | Fonctionnalité | Description |
|----------|----------------|-------------|
| 🟡 MOYENNE | Messagerie interne | Chat entre locataire et bailleur |
| 🟡 MOYENNE | Notifications | Badge + compteur de messages non lus |
| 🟢 BASSE | Historique conversations | Archivage automatique |

### Phase 5 - Tableau de Bord
| Priorité | Fonctionnalité | Description |
|----------|----------------|-------------|
| 🟡 MOYENNE | Dashboard bailleur | Vue d'ensemble annonces + stats |
| 🟡 MOYENNE | Dashboard locataire | Favoris + recherches sauvegardées |
| 🟢 BASSE | Statistiques | Vues, contacts, taux de réponse |

---

## 5. Design System

### Palette de Couleurs
```css
/* Couleurs principales */
--primary: #2563EB;        /* Bleu moderne */
--primary-dark: #1E40AF;
--accent: #A86A3D;         /* Terre/Bois */
--accent-light: #D4A574;

/* Neutres */
--background: #FAFAFA;     /* Blanc cassé */
--surface: #FFFFFF;
--text-primary: #1F2937;   /* Noir doux */
--text-secondary: #6B7280; /* Gris */
--border: #E5E7EB;

/* États */
--success: #10B981;
--warning: #F59E0B;
--error: #EF4444;
```

### Typographie
- **Titres**: Playfair Display (serif élégant)
- **Corps**: Inter (sans-serif moderne et lisible)
- **Code/Données**: JetBrains Mono

### Composants UI
- Boutons avec états hover/active/disabled
- Cards avec ombre subtile
- Inputs avec validation visuelle
- Modals/Dialogs accessibles
- Toast notifications
- Skeleton loaders

---

## 6. Structure du Projet

```
location-platform/
├── backend/                    # API Rust + Axum
│   ├── src/
│   │   ├── main.rs            # Point d'entrée
│   │   ├── config.rs          # Configuration (DB, JWT, etc.)
│   │   ├── models/            # Structures de données
│   │   │   ├── user.rs
│   │   │   ├── property.rs
│   │   │   └── message.rs
│   │   ├── handlers/          # Handlers HTTP (controllers)
│   │   │   ├── auth.rs
│   │   │   ├── properties.rs
│   │   │   └── messages.rs
│   │   ├── services/          # Logique métier
│   │   ├── middleware/        # Auth, CORS, Rate limiting
│   │   ├── utils/             # Helpers (JWT, validation, etc.)
│   │   └── database/          # Connexion DB + migrations
│   ├── migrations/            # SQL migrations
│   ├── Cargo.toml
│   └── Dockerfile
│
├── frontend/                   # Application React
│   ├── src/
│   │   ├── main.tsx           # Point d'entrée
│   │   ├── App.tsx            # Composant racine
│   │   ├── pages/             # Pages de l'application
│   │   │   ├── Home.tsx
│   │   │   ├── Login.tsx
│   │   │   ├── Register.tsx
│   │   │   ├── PropertyList.tsx
│   │   │   ├── PropertyDetail.tsx
│   │   │   └── Dashboard.tsx
│   │   ├── components/        # Composants réutilisables
│   │   │   ├── Navbar.tsx
│   │   │   ├── PropertyCard.tsx
│   │   │   ├── SearchBar.tsx
│   │   │   └── Footer.tsx
│   │   ├── hooks/             # Custom React hooks
│   │   ├── services/          # API calls (axios)
│   │   ├── store/             # State management (Zustand)
│   │   ├── types/             # TypeScript types
│   │   └── utils/             # Helpers
│   ├── public/
│   ├── package.json
│   ├── tailwind.config.js
│   ├── vite.config.ts
│   └── Dockerfile
│
├── docker-compose.yml          # Orchestration complète
├── .env.example                # Variables d'environnement
├── README.md                   # Documentation
└── DEPLOYMENT.md               # Guide de déploiement
```

---

## 7. Base de Données PostgreSQL

### Tables Principales

```sql
-- Users (Locataires & Bailleurs)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone VARCHAR(20),
    role VARCHAR(20) NOT NULL CHECK (role IN ('tenant', 'landlord', 'admin')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Properties (Logements)
CREATE TABLE properties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    landlord_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    property_type VARCHAR(50) NOT NULL, -- appartement, studio, chambre, maison
    address TEXT NOT NULL,
    city VARCHAR(100) NOT NULL,
    postal_code VARCHAR(10),
    price_per_month DECIMAL(10, 2) NOT NULL,
    surface_area DECIMAL(8, 2), -- en m²
    rooms_count INTEGER,
    bedrooms_count INTEGER,
    bathrooms_count INTEGER,
    available_from DATE,
    status VARCHAR(20) DEFAULT 'available', -- available, reserved, rented
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Property Images
CREATE TABLE property_images (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    property_id UUID REFERENCES properties(id) ON DELETE CASCADE,
    image_url TEXT NOT NULL,
    is_primary BOOLEAN DEFAULT FALSE,
    display_order INTEGER DEFAULT 0,
    uploaded_at TIMESTAMPTZ DEFAULT NOW()
);

-- Messages
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    property_id UUID REFERENCES properties(id) ON DELETE CASCADE,
    sender_id UUID REFERENCES users(id) ON DELETE CASCADE,
    recipient_id UUID REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    sent_at TIMESTAMPTZ DEFAULT NOW()
);

-- Favoris
CREATE TABLE favorites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    property_id UUID REFERENCES properties(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, property_id)
);
```

---

## 8. Plan d'Exécution par Phases

### ✅ Phase 1 - Préparation (Aujourd'hui)
1. Révision de la fiche fonctionnelle
2. Installation des outils nécessaires
3. Création de l'arborescence projet
4. Configuration initiale Docker

### 🔧 Phase 2 - Backend Foundation (Jours 1-2)
5. Initialisation projet Rust + Axum
6. Configuration base de données PostgreSQL
7. Création des modèles de données
8. Mise en place de l'authentification JWT
9. Routes API de base (CRUD users)

### 🎨 Phase 3 - Frontend Foundation (Jours 3-4)
10. Initialisation React + TypeScript + Vite
11. Configuration Tailwind CSS + Design System
12. Création des pages principales (routing)
13. Composants UI de base (Navbar, Footer, Cards)
14. Intégration API (Axios + interceptors)

### 🏠 Phase 4 - Fonctionnalités Logements (Jours 5-7)
15. Backend: CRUD complet des propriétés
16. Backend: Upload et gestion d'images
17. Frontend: Formulaire de création d'annonce
18. Frontend: Liste et recherche de logements
19. Frontend: Page détail d'un logement

### 💬 Phase 5 - Messagerie (Jours 8-9)
20. Backend: API messagerie
21. Frontend: Interface de chat
22. Notifications en temps réel (optionnel: WebSocket)

### 📊 Phase 6 - Tableaux de Bord (Jours 10-11)
23. Dashboard bailleur avec statistiques
24. Dashboard locataire avec favoris
25. Filtres avancés et tri

### 🚀 Phase 7 - Finalisation (Jours 12-14)
26. Tests manuels complets
27. Optimisations performances
28. Documentation technique
29. Guide de déploiement
30. Préparation du package final (.zip)

---

## 9. Améliorations Futures (Post-MVP)

### Court terme
- ⭐ Système d'avis et de notation
- 📍 Carte interactive (Google Maps / Leaflet)
- 📧 Notifications email automatiques
- 🔔 Système d'alertes pour nouvelles annonces

### Moyen terme
- 💳 Paiement en ligne (Stripe)
- 📅 Calendrier de visites
- 📄 Génération de contrats PDF
- 🤖 Chatbot d'assistance

### Long terme
- 📱 Applications mobiles natives (React Native)
- 🌐 Multi-langue (i18n)
- 🎯 Recommandations basées sur IA
- 📊 Analytics avancés pour bailleurs

---

## 10. Critères de Qualité

### Performance
- ✅ Lighthouse Score > 90
- ✅ Time to First Byte < 200ms
- ✅ First Contentful Paint < 1.5s
- ✅ Code Splitting et Lazy Loading

### Sécurité
- ✅ HTTPS obligatoire en production
- ✅ Headers de sécurité (CSP, HSTS, etc.)
- ✅ Rate limiting sur API
- ✅ Validation stricte des inputs
- ✅ Protection CSRF

### SEO
- ✅ Meta tags optimisées
- ✅ Structure HTML sémantique
- ✅ Sitemap.xml généré
- ✅ Schema.org markup
- ✅ URLs propres et descriptives

### Accessibilité
- ✅ ARIA labels
- ✅ Navigation au clavier
- ✅ Contraste suffisant (WCAG AA)
- ✅ Alt text sur images

---

## 11. Environnement de Développement

### Prérequis Windows 11
```bash
# Node.js 18+ (LTS)
https://nodejs.org/

# Rust (via rustup)
https://rustup.rs/

# PostgreSQL 15
https://www.postgresql.org/download/windows/

# Docker Desktop
https://www.docker.com/products/docker-desktop/

# VS Code + Extensions
- rust-analyzer
- Tailwind CSS IntelliSense
- ES7+ React/Redux/React-Native snippets
- Prettier
- ESLint
- Docker
```

---

## 12. Variables d'Environnement

### Backend (.env)
```env
DATABASE_URL=postgres://user:password@localhost:5432/location_db
JWT_SECRET=your-super-secret-key-change-me
JWT_EXPIRATION=3600
RUST_LOG=debug
PORT=8000
CORS_ORIGIN=http://localhost:5173
```

### Frontend (.env)
```env
VITE_API_URL=http://localhost:8000/api
VITE_APP_NAME=Location
```

---

## 📝 Notes Importantes

1. **Axum vs Actix**: Axum a été choisi pour sa modernité, sa simplicité et son excellent système de types. Il est plus facile à apprendre et maintenir.

2. **TypeScript**: Ajouté au frontend pour une meilleure maintenabilité et moins d'erreurs en production.

3. **Vite**: Remplace Create React App pour des temps de build ultra-rapides et une meilleure expérience développeur.

4. **Zustand**: State management léger et simple, parfait pour ce projet (alternative à Redux).

5. **Docker**: Permet un déploiement uniforme sur n'importe quel environnement.

---

## 🎯 Prochaine Étape

**Voulez-vous que je commence par :**
1. ✅ La configuration complète de l'environnement ?
2. 🏗️ La création de l'arborescence avec fichiers de base ?
3. 🚀 Directement le backend Axum avec la DB ?

**Dites-moi et on démarre ! 💪**