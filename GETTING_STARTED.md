# 🚀 Guide de Démarrage Rapide - Location Platform

Bienvenue ! Ce guide vous aidera à lancer le projet en  **moins de 10 minutes** .

---

## ✅ Vérification des Prérequis

Avant de commencer, vérifiez que vous avez installé :

### Windows 11

```powershell
# Vérifier Node.js
node --version
# Devrait afficher v18.x ou supérieur

# Vérifier Rust
rustc --version
# Devrait afficher rustc 1.75.x ou supérieur

# Vérifier Docker
docker --version
docker-compose --version

# Vérifier Git
git --version
```

Si une commande ne fonctionne pas, installez le logiciel manquant :

* **Node.js** : https://nodejs.org/
* **Rust** : https://rustup.rs/
* **Docker Desktop** : https://www.docker.com/products/docker-desktop/
* **Git** : https://git-scm.com/

---

## 📥 Étape 1 : Récupérer le Projet

### Option A : Vous avez déjà le dossier

Si vous avez déjà le dossier `location-platform`, ouvrez-le dans VS Code :

```powershell
cd chemin\vers\location-platform
code .
```

### Option B : Cloner depuis Git

```powershell
git clone https://github.com/votre-username/location-platform.git
cd location-platform
code .
```

---

## ⚙️ Étape 2 : Configuration

### 2.1 Backend - Fichier .env

```powershell
cd backend
copy .env.example .env
notepad .env
```

**Contenu minimal pour le développement** :

```env
DATABASE_URL=postgres://location_user:location_pass@localhost:5432/location_db
JWT_SECRET=dev-secret-key-change-in-production
JWT_EXPIRATION=3600
PORT=8000
RUST_LOG=debug
CORS_ORIGIN=http://localhost:5173
```

### 2.2 Frontend - Fichier .env

```powershell
cd ..\frontend
echo VITE_API_URL=http://localhost:8000/api > .env
```

---

## 🐳 Étape 3 : Démarrage avec Docker (Recommandé)

### Méthode Simple - Un seul clic

À la racine du projet, double-cliquez sur :

```
start.bat
```

OU en ligne de commande :

```powershell
.\start.bat
```

### Méthode manuelle

```powershell
# À la racine du projet
docker-compose up -d --build
```

**Attendez environ 2-3 minutes** que tout se construise et démarre.

### Vérification

Ouvrez votre navigateur :

* ✅ **Frontend** : http://localhost:5173
* ✅ **API Health** : http://localhost:8000/health

Vous devriez voir :

* Le site Location s'afficher
* Un JSON `{"status":"ok"}` pour l'API

---

## 🛠️ Étape 4 : Développement Manuel (Optionnel)

Si vous préférez lancer les services séparément sans Docker :

### 4.1 Démarrer PostgreSQL

```powershell
docker run --name location_postgres `
  -e POSTGRES_USER=location_user `
  -e POSTGRES_PASSWORD=location_pass `
  -e POSTGRES_DB=location_db `
  -p 5432:5432 `
  -d postgres:15-alpine
```

### 4.2 Démarrer le Backend

**Terminal 1 (Backend)** :

```powershell
cd backend

# Installer SQLx CLI (une seule fois)
cargo install sqlx-cli --no-default-features --features postgres

# Créer et migrer la base de données
sqlx database create
sqlx migrate run

# Lancer le serveur
cargo run
```

Attendez le message : `🚀 Server listening on 0.0.0.0:8000`

### 4.3 Démarrer le Frontend

**Terminal 2 (Frontend)** :

```powershell
cd frontend

# Installer les dépendances (une seule fois)
npm install

# Lancer le serveur de dev
npm run dev
```

Attendez le message avec l'URL : `http://localhost:5173`

---

## 🎉 Étape 5 : Premier Test

### 5.1 Créer un compte

1. Allez sur http://localhost:5173
2. Cliquez sur **"Inscription"**
3. Remplissez le formulaire :
   * Email : `test@example.com`
   * Mot de passe : `password123`
   * Prénom : `John`
   * Nom : `Doe`
   * Type : **Locataire** ou **Bailleur**
4. Cliquez sur **"Créer mon compte"**

### 5.2 Explorer l'application

* 🏠 **Page d'accueil** : Découvrez les fonctionnalités
* 🔍 **Rechercher** : Parcourez les logements disponibles
* 📊 **Dashboard** : Accédez à votre espace personnel
* ℹ️ **À Propos** : En savoir plus sur la plateforme

---

## 📁 Structure du Projet

```
location-platform/
├── backend/                # API Rust + Axum
│   ├── src/
│   │   ├── main.rs        # 🚀 Point d'entrée
│   │   ├── handlers/      # 🎯 Routes API
│   │   ├── models/        # 📦 Structures de données
│   │   └── database.rs    # 🗄️ Connexion DB
│   ├── migrations/        # 📝 Migrations SQL
│   └── Cargo.toml         # ⚙️ Dépendances Rust
│
├── frontend/              # Application React
│   ├── src/
│   │   ├── App.tsx        # 🎨 Composant racine
│   │   ├── pages/         # 📄 Pages
│   │   ├── components/    # 🧩 Composants
│   │   ├── services/      # 🔌 API calls
│   │   └── store/         # 💾 State management
│   ├── package.json       # ⚙️ Dépendances npm
│   └── vite.config.ts     # ⚡ Config Vite
│
├── docker-compose.yml     # 🐳 Orchestration
├── README.md              # 📚 Documentation
├── COMMANDS.md            # 📝 Commandes utiles
└── start.bat              # ▶️ Script de démarrage
```

---

## 🔧 Commandes Essentielles

### Docker

```powershell
# Démarrer
docker-compose up -d

# Voir les logs
docker-compose logs -f

# Arrêter
docker-compose down

# Redémarrer
docker-compose restart
```

### Backend (si lancé manuellement)

```powershell
cd backend

# Mode développement
cargo run

# Mode watch (redémarre automatiquement)
cargo watch -x run

# Tests
cargo test
```

### Frontend (si lancé manuellement)

```powershell
cd frontend

# Développement
npm run dev

# Build production
npm run build

# Tests linting
npm run lint
```

---

## 🐛 Problèmes Courants

### ❌ "Port already in use"

**Solution** : Un service utilise déjà le port.

```powershell
# Trouver le processus
netstat -ano | findstr :8000
netstat -ano | findstr :5173

# Tuer le processus
taskkill /PID <PID> /F
```

### ❌ "Database connection failed"

**Solution** : PostgreSQL n'est pas démarré.

```powershell
# Vérifier si le conteneur tourne
docker ps

# Redémarrer PostgreSQL
docker-compose restart postgres
```

### ❌ "Module not found" (Frontend)

**Solution** : Réinstaller les dépendances.

```powershell
cd frontend
Remove-Item node_modules -Recurse -Force
npm install
```

### ❌ "cargo: command not found"

**Solution** : Rust n'est pas dans le PATH.

Redémarrez votre terminal ou PowerShell après l'installation de Rust.

---

## 📖 Prochaines Étapes

Maintenant que tout fonctionne, vous pouvez :

1. **Explorer le code** :
   * `backend/src/handlers/` : Routes API
   * `frontend/src/pages/` : Pages React
2. **Lire la documentation complète** :
   * `README.md` : Vue d'ensemble
   * `COMMANDS.md` : Toutes les commandes
   * `DEPLOYMENT.md` : Déploiement en production
3. **Modifier et tester** :
   * Changez du code et voyez les changements en temps réel
   * Créez de nouvelles fonctionnalités
4. **Contribuer** :
   * Créez une branche pour vos modifications
   * Testez vos changements
   * Créez une Pull Request

---

## 💡 Conseils pour VS Code

### Extensions Recommandées

Installez ces extensions dans VS Code :

* **rust-analyzer** : Autocomplétion Rust
* **Tailwind CSS IntelliSense** : Suggestions Tailwind
* **ES7+ React snippets** : Snippets React
* **Prettier** : Formatage automatique
* **Docker** : Gestion des conteneurs
* **Thunder Client** : Tester l'API

### Raccourcis utiles

* `Ctrl + ù` : Ouvrir/fermer le terminal intégré
* `Ctrl + P` : Recherche rapide de fichier
* `Ctrl + Shift + P` : Palette de commandes
* `Ctrl + B` : Toggle sidebar
* `F5` : Débugger (si configuré)

---

## 🆘 Besoin d'Aide ?

### Documentation

* 📚 `README.md` : Documentation complète
* 📝 `COMMANDS.md` : Toutes les commandes
* 🚀 `DEPLOYMENT.md` : Guide de déploiement
* 🔍 `fiche_fonctionnelle_v2` : Spécifications

### Support

* 📧 Email : support@location.com
* 🐛 Issues : GitHub Issues
* 💬 Discord : [Lien vers Discord]

---

## ✅ Checklist de Démarrage

* [ ] Prérequis installés (Node, Rust, Docker, Git)
* [ ] Projet cloné ou téléchargé
* [ ] Fichiers `.env` configurés
* [ ] Docker Compose lancé
* [ ] Frontend accessible (http://localhost:5173)
* [ ] API fonctionnelle (http://localhost:8000/health)
* [ ] Compte test créé
* [ ] VS Code configuré avec extensions

---

**Félicitations ! Vous êtes prêt à développer ! 🎉**

Bon coding ! 💻🚀
