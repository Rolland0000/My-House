# 🏠 Location - Plateforme de Location Immobilière

Plateforme web moderne de mise en relation entre locataires et bailleurs. Développée avec **Rust (Axum)** et  **React (TypeScript)** .

---

## 📋 Table des Matières

* [Fonctionnalités](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#fonctionnalit%C3%A9s)
* [Stack Technique](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#stack-technique)
* [Prérequis](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#pr%C3%A9requis)
* [Installation](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#installation)
* [Démarrage](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#d%C3%A9marrage)
* [Structure du Projet](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#structure-du-projet)
* [API Endpoints](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#api-endpoints)
* [Développement](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#d%C3%A9veloppement)
* [Déploiement](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#d%C3%A9ploiement)

---

## ✨ Fonctionnalités

### Pour les Locataires

* ✅ Recherche de logements avec filtres avancés (ville, type, prix, nombre de pièces)
* ✅ Visualisation détaillée des annonces avec galerie photos
* ✅ Messagerie intégrée pour contacter les bailleurs
* ✅ Système de favoris
* ✅ Tableau de bord personnalisé

### Pour les Bailleurs

* ✅ Création et gestion d'annonces
* ✅ Upload de photos pour les biens
* ✅ Tableau de bord avec statistiques
* ✅ Gestion des demandes des locataires

### Fonctionnalités Générales

* ✅ Authentification sécurisée (JWT)
* ✅ Design responsive (mobile-first)
* ✅ Interface moderne et intuitive
* ✅ Performance optimisée (Lighthouse > 90)

---

## 🛠 Stack Technique

### Backend

* **Framework** : Axum 0.7 (Rust)
* **Base de données** : PostgreSQL 15
* **ORM** : SQLx
* **Authentification** : JWT (jsonwebtoken)
* **Sécurité** : Bcrypt pour les mots de passe
* **Validation** : Validator

### Frontend

* **Framework** : React 18 + TypeScript
* **Build Tool** : Vite
* **Styling** : Tailwind CSS
* **Routing** : React Router v6
* **State Management** : Zustand
* **HTTP Client** : Axios
* **Icons** : Lucide React

### DevOps

* **Conteneurisation** : Docker + Docker Compose
* **Reverse Proxy** : Nginx
* **CI/CD** : Prêt pour GitHub Actions

---

## 📦 Prérequis

Assurez-vous d'avoir installé :

* **Node.js** >= 18.x ([Télécharger](https://nodejs.org/))
* **Rust** >= 1.75 ([Installer via rustup](https://rustup.rs/))
* **PostgreSQL** >= 15 ([Télécharger](https://www.postgresql.org/download/))
* **Docker Desktop** ([Télécharger](https://www.docker.com/products/docker-desktop/))
* **Git** ([Télécharger](https://git-scm.com/))

---

## 🚀 Installation

### 1. Cloner le projet

```bash
git clone https://github.com/votre-username/location-platform.git
cd location-platform
```

### 2. Configuration Backend

```bash
cd backend

# Copier le fichier d'environnement
copy .env.example .env

# Éditer le fichier .env avec vos paramètres
notepad .env
```

**Contenu du fichier `.env`** :

```env
DATABASE_URL=postgres://location_user:location_pass@localhost:5432/location_db
JWT_SECRET=votre-clé-secrète-super-sécurisée-changez-moi
JWT_EXPIRATION=3600
PORT=8000
RUST_LOG=debug
CORS_ORIGIN=http://localhost:5173
```

### 3. Configuration Frontend

```bash
cd ../frontend

# Installer les dépendances
npm install

# Créer le fichier .env
echo VITE_API_URL=http://localhost:8000/api > .env
```

---

## 🏃 Démarrage

### Option 1 : Avec Docker (Recommandé)

La méthode la plus simple pour tout démarrer :

```bash
# À la racine du projet
docker-compose up --build
```

Accédez ensuite à :

* **Frontend** : http://localhost:5173
* **Backend API** : http://localhost:8000
* **Health Check** : http://localhost:8000/health

### Option 2 : Manuel (Développement)

#### Démarrer PostgreSQL

```bash
# Avec Docker
docker run --name location_postgres -e POSTGRES_USER=location_user -e POSTGRES_PASSWORD=location_pass -e POSTGRES_DB=location_db -p 5432:5432 -d postgres:15-alpine
```

Ou utilisez votre installation locale de PostgreSQL.

#### Démarrer le Backend

```bash
cd backend

# Installer SQLx CLI (une seule fois)
cargo install sqlx-cli --no-default-features --features postgres

# Créer la base de données
sqlx database create

# Exécuter les migrations
sqlx migrate run

# Démarrer le serveur
cargo run

# Ou en mode watch (redémarrage auto)
cargo install cargo-watch
cargo watch -x run
```

Le backend sera accessible sur **http://localhost:8000**

#### Démarrer le Frontend

```bash
cd frontend

# Démarrer le serveur de développement
npm run dev
```

Le frontend sera accessible sur **http://localhost:5173**

---

## 📁 Structure du Projet

```
location-platform/
├── backend/                    # API Rust + Axum
│   ├── src/
│   │   ├── main.rs            # Point d'entrée
│   │   ├── config.rs          # Configuration
│   │   ├── database.rs        # Connexion DB
│   │   ├── models/            # Modèles de données
│   │   ├── handlers/          # Handlers HTTP
│   │   ├── services/          # Logique métier
│   │   ├── middleware/        # Middlewares
│   │   └── utils/             # Utilitaires
│   ├── migrations/            # Migrations SQL
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── .env.example
│
├── frontend/                   # Application React
│   ├── src/
│   │   ├── main.tsx           # Point d'entrée
│   │   ├── App.tsx            # Composant racine
│   │   ├── pages/             # Pages
│   │   ├── components/        # Composants réutilisables
│   │   ├── services/          # Appels API
│   │   ├── store/             # State management
│   │   ├── types/             # Types TypeScript
│   │   └── utils/             # Utilitaires
│   ├── public/
│   ├── index.html
│   ├── package.json
│   ├── tailwind.config.js
│   ├── vite.config.ts
│   ├── Dockerfile
│   └── nginx.conf
│
├── docker-compose.yml          # Orchestration complète
├── README.md                   # Ce fichier
└── DEPLOYMENT.md               # Guide de déploiement
```

---

## 🔌 API Endpoints

### Authentification

| Méthode | Endpoint               | Description                         |
| -------- | ---------------------- | ----------------------------------- |
| POST     | `/api/auth/register` | Inscription d'un nouvel utilisateur |
| POST     | `/api/auth/login`    | Connexion utilisateur               |

### Propriétés

| Méthode | Endpoint                   | Description                       |
| -------- | -------------------------- | --------------------------------- |
| GET      | `/api/properties`        | Liste toutes les propriétés     |
| GET      | `/api/properties/:id`    | Détails d'une propriété        |
| POST     | `/api/properties`        | Créer une propriété (bailleur) |
| PUT      | `/api/properties/:id`    | Modifier une propriété          |
| DELETE   | `/api/properties/:id`    | Supprimer une propriété         |
| GET      | `/api/properties/search` | Recherche avec filtres            |

### Health Check

| Méthode | Endpoint    | Description                  |
| -------- | ----------- | ---------------------------- |
| GET      | `/health` | Vérifier l'état du serveur |

---

## 💻 Développement

### Backend (Rust)

```bash
# Vérifier le code
cargo check

# Formatter le code
cargo fmt

# Lancer les tests
cargo test

# Build en mode release
cargo build --release
```

### Frontend (React)

```bash
# Lancer le linter
npm run lint

# Build pour la production
npm run build

# Prévisualiser le build
npm run preview
```

### Base de Données

```bash
# Créer une nouvelle migration
sqlx migrate add nom_de_la_migration

# Exécuter les migrations
sqlx migrate run

# Revenir en arrière
sqlx migrate revert
```

---

## 🌐 Déploiement

### Prérequis Production

* Serveur Linux (Ubuntu 22.04 recommandé)
* Docker + Docker Compose
* Nom de domaine configuré
* Certificat SSL (Let's Encrypt)

### Étapes de Déploiement

1. **Cloner le projet sur le serveur**

```bash
git clone https://github.com/votre-username/location-platform.git
cd location-platform
```

2. **Configurer les variables d'environnement**

```bash
# Backend
cp backend/.env.example backend/.env
nano backend/.env  # Modifier avec vos valeurs de production

# Frontend
echo "VITE_API_URL=https://votre-domaine.com/api" > frontend/.env
```

3. **Démarrer avec Docker Compose**

```bash
docker-compose -f docker-compose.prod.yml up -d --build
```

4. **Configurer Nginx + SSL (Optionnel si proxy externe)**

Voir le fichier `DEPLOYMENT.md` pour les instructions détaillées.

### Monitoring

```bash
# Voir les logs
docker-compose logs -f

# Redémarrer un service
docker-compose restart backend

# Arrêter tous les services
docker-compose down
```

---

## 🧪 Tests

### Backend

```bash
cd backend
cargo test
```

### Frontend

```bash
cd frontend
npm run test  # (à configurer avec Vitest)
```

---

## 📝 Variables d'Environnement

### Backend

| Variable           | Description                             | Défaut               |
| ------------------ | --------------------------------------- | --------------------- |
| `DATABASE_URL`   | URL de connexion PostgreSQL             | -                     |
| `JWT_SECRET`     | Clé secrète pour JWT                  | -                     |
| `JWT_EXPIRATION` | Durée de validité du token (secondes) | 3600                  |
| `PORT`           | Port du serveur                         | 8000                  |
| `RUST_LOG`       | Niveau de logs                          | debug                 |
| `CORS_ORIGIN`    | Origine autorisée pour CORS            | http://localhost:5173 |

### Frontend

| Variable         | Description          | Défaut                   |
| ---------------- | -------------------- | ------------------------- |
| `VITE_API_URL` | URL de l'API backend | http://localhost:8000/api |

---

## 🤝 Contribution

Les contributions sont les bienvenues ! Pour contribuer :

1. Fork le projet
2. Créer une branche (`git checkout -b feature/AmazingFeature`)
3. Commit vos changements (`git commit -m 'Add AmazingFeature'`)
4. Push vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrir une Pull Request

---

## 📄 Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.

---

## 👥 Auteurs

* **Votre Nom** - Développeur Full Stack

---

## 🙏 Remerciements

* [Axum](https://github.com/tokio-rs/axum) - Framework web Rust performant
* [React](https://react.dev/) - Bibliothèque UI
* [Tailwind CSS](https://tailwindcss.com/) - Framework CSS utilitaire
* [PostgreSQL](https://www.postgresql.org/) - Base de données relationnelle

---

## 📞 Support

Pour toute question ou problème :

* 📧 Email : support@location.com
* 🐛 Issues : [GitHub Issues](https://github.com/votre-username/location-platform/issues)

---

**Bon développement ! 🚀**
