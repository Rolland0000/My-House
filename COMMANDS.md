# 📝 Commandes Utiles - Location Platform

Guide de référence rapide des commandes fréquemment utilisées.

---

## 🐳 Docker & Docker Compose

### Démarrage

```bash
# Démarrer tous les services
docker-compose up -d

# Démarrer avec rebuild
docker-compose up -d --build

# Démarrer en mode développement (avec logs)
docker-compose up
```

### Arrêt

```bash
# Arrêter tous les services
docker-compose down

# Arrêter et supprimer les volumes
docker-compose down -v

# Redémarrer un service spécifique
docker-compose restart backend
```

### Logs

```bash
# Voir tous les logs
docker-compose logs -f

# Logs d'un service spécifique
docker-compose logs -f backend
docker-compose logs -f frontend
docker-compose logs -f postgres

# Dernières 100 lignes
docker-compose logs --tail=100 backend
```

### État des conteneurs

```bash
# Voir l'état des services
docker-compose ps

# Statistiques d'utilisation
docker stats
```

---

## 🦀 Backend (Rust)

### Développement

```bash
cd backend

# Lancer en mode développement
cargo run

# Lancer avec auto-reload
cargo watch -x run

# Vérifier le code
cargo check

# Formatter le code
cargo fmt

# Lancer les tests
cargo test

# Build en release
cargo build --release
```

### Base de données

```bash
# Créer la base de données
sqlx database create

# Exécuter les migrations
sqlx migrate run

# Revenir en arrière d'une migration
sqlx migrate revert

# Créer une nouvelle migration
sqlx migrate add nom_de_la_migration

# Vérifier l'état des migrations
sqlx migrate info
```

### Tests & Qualité

```bash
# Tests unitaires
cargo test

# Tests avec output détaillé
cargo test -- --nocapture

# Lint avec Clippy
cargo clippy

# Vérifier la sécurité
cargo audit
```

---

## ⚛️ Frontend (React)

### Développement

```bash
cd frontend

# Installer les dépendances
npm install

# Démarrer le serveur de développement
npm run dev

# Build pour la production
npm run build

# Prévisualiser le build
npm run preview
```

### Qualité de code

```bash
# Linter
npm run lint

# Formatter avec Prettier (si configuré)
npm run format

# Vérifier les types TypeScript
npm run type-check
```

### Gestion des dépendances

```bash
# Ajouter une dépendance
npm install nom-du-package

# Ajouter une dépendance de dev
npm install -D nom-du-package

# Mettre à jour les dépendances
npm update

# Auditer la sécurité
npm audit

# Corriger les vulnérabilités
npm audit fix
```

---

## 🗄️ PostgreSQL

### Connexion

```bash
# Se connecter à la base de données
docker exec -it location_db psql -U location_user -d location_db

# Via psql local
psql -U location_user -d location_db -h localhost
```

### Commandes SQL utiles

```sql
-- Lister les tables
\dt

-- Décrire une table
\d users

-- Lister les bases de données
\l

-- Quitter
\q

-- Voir les connexions actives
SELECT * FROM pg_stat_activity;

-- Taille de la base de données
SELECT pg_size_pretty(pg_database_size('location_db'));
```

### Sauvegarde et Restauration

```bash
# Sauvegarde
docker exec location_db pg_dump -U location_user location_db > backup.sql

# Restauration
docker exec -i location_db psql -U location_user location_db < backup.sql

# Sauvegarde compressée
docker exec location_db pg_dump -U location_user location_db | gzip > backup.sql.gz

# Restauration depuis fichier compressé
gunzip < backup.sql.gz | docker exec -i location_db psql -U location_user location_db
```

---

## 🔧 Nettoyage

### Docker

```bash
# Supprimer les conteneurs arrêtés
docker container prune

# Supprimer les images non utilisées
docker image prune

# Supprimer les volumes non utilisés
docker volume prune

# Nettoyage complet
docker system prune -a

# Nettoyage avec volumes
docker system prune -a --volumes
```

### Projet

```bash
# Backend
cd backend
cargo clean

# Frontend
cd frontend
rm -rf node_modules
rm -rf dist
npm install
```

---

## 🚀 Déploiement

### Build Production

```bash
# Backend
cd backend
cargo build --release

# Frontend
cd frontend
npm run build
```

### Docker Production

```bash
# Build et démarrage
docker-compose -f docker-compose.prod.yml up -d --build

# Voir les logs
docker-compose -f docker-compose.prod.yml logs -f

# Arrêter
docker-compose -f docker-compose.prod.yml down
```

---

## 🔍 Debugging

### Backend

```bash
# Logs détaillés
RUST_LOG=debug cargo run

# Backtrace en cas d'erreur
RUST_BACKTRACE=1 cargo run

# Full backtrace
RUST_BACKTRACE=full cargo run
```

### Frontend

```bash
# Ouvrir les DevTools du navigateur
# Chrome/Edge: F12 ou Ctrl+Shift+I
# Firefox: F12 ou Ctrl+Shift+K

# Voir les requêtes réseau
# Onglet Network dans les DevTools

# Console JavaScript
# Onglet Console dans les DevTools
```

### Docker

```bash
# Entrer dans un conteneur
docker exec -it location_backend sh
docker exec -it location_frontend sh

# Voir les processus dans un conteneur
docker top location_backend

# Inspecter un conteneur
docker inspect location_backend

# Voir l'utilisation des ressources
docker stats location_backend
```

---

## 🧪 Tests API

### Avec curl

```bash
# Health Check
curl http://localhost:8000/health

# Inscription
curl -X POST http://localhost:8000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123",
    "first_name": "John",
    "last_name": "Doe",
    "role": "tenant"
  }'

# Connexion
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'

# Lister les propriétés
curl http://localhost:8000/api/properties

# Avec authentification
curl http://localhost:8000/api/properties \
  -H "Authorization: Bearer VOTRE_TOKEN_JWT"
```

### Avec PowerShell (Windows)

```powershell
# Health Check
Invoke-RestMethod -Uri http://localhost:8000/health

# Inscription
$body = @{
    email = "test@example.com"
    password = "password123"
    first_name = "John"
    last_name = "Doe"
    role = "tenant"
} | ConvertTo-Json

Invoke-RestMethod -Uri http://localhost:8000/api/auth/register `
  -Method Post `
  -ContentType "application/json" `
  -Body $body

# Lister les propriétés
Invoke-RestMethod -Uri http://localhost:8000/api/properties
```

---

## 📊 Monitoring

### Logs système

```bash
# Voir les logs système (Linux)
tail -f /var/log/syslog

# Logs Nginx
tail -f /var/log/nginx/access.log
tail -f /var/log/nginx/error.log

# Utilisation mémoire
free -h

# Utilisation disque
df -h

# Processus
top
htop  # Si installé
```

### Monitoring Docker

```bash
# Utilisation des ressources en temps réel
docker stats

# Inspecter les logs d'un conteneur
docker logs -f location_backend

# Événements Docker
docker events

# Informations système Docker
docker system info
```

---

## 🔐 Sécurité

### Générer un secret JWT

```bash
# Linux/Mac
openssl rand -hex 32

# PowerShell (Windows)
[System.Convert]::ToBase64String((1..32 | ForEach-Object { Get-Random -Maximum 256 }))

# Node.js
node -e "console.log(require('crypto').randomBytes(32).toString('hex'))"
```

### Vérifier les vulnérabilités

```bash
# Backend (Rust)
cd backend
cargo audit

# Frontend (npm)
cd frontend
npm audit

# Fix automatique des vulnérabilités npm
npm audit fix
```

---

## 🌐 Réseau

### Tester la connectivité

```bash
# Ping
ping localhost

# Tester un port
telnet localhost 8000
# Ou avec nc (netcat)
nc -zv localhost 8000

# Windows PowerShell
Test-NetConnection -ComputerName localhost -Port 8000

# Voir les ports ouverts
netstat -tulpn | grep LISTEN  # Linux
netstat -ano | findstr LISTEN  # Windows
```

### DNS

```bash
# Résoudre un nom de domaine
nslookup votre-domaine.com
dig votre-domaine.com

# Windows
nslookup votre-domaine.com
```

---

## 📝 Git

### Workflow de base

```bash
# Voir le statut
git status

# Ajouter des fichiers
git add .
git add fichier.txt

# Commit
git commit -m "Description du commit"

# Push
git push origin main

# Pull
git pull origin main

# Créer une branche
git checkout -b feature/nouvelle-fonctionnalite

# Fusionner une branche
git checkout main
git merge feature/nouvelle-fonctionnalite

# Voir l'historique
git log
git log --oneline --graph
```

### Annuler des modifications

```bash
# Annuler les modifications non commitées
git checkout -- fichier.txt

# Annuler le dernier commit (garder les modifications)
git reset --soft HEAD~1

# Annuler le dernier commit (supprimer les modifications)
git reset --hard HEAD~1

# Revenir à un commit spécifique
git checkout COMMIT_HASH
```

---

## 🔄 Mise à jour

### Mettre à jour l'application

```bash
# Récupérer les dernières modifications
git pull origin main

# Rebuild les conteneurs
docker-compose down
docker-compose up -d --build

# Mettre à jour les dépendances Backend
cd backend
cargo update

# Mettre à jour les dépendances Frontend
cd frontend
npm update
```

---

## 📦 Backup

### Backup complet

```bash
# Créer un répertoire de backup
mkdir -p backups/$(date +%Y%m%d)

# Backup de la base de données
docker exec location_db pg_dump -U location_user location_db > backups/$(date +%Y%m%d)/database.sql

# Backup des fichiers uploadés (si applicable)
cp -r uploads/ backups/$(date +%Y%m%d)/uploads/

# Backup de la configuration
cp .env backups/$(date +%Y%m%d)/.env
cp docker-compose.yml backups/$(date +%Y%m%d)/docker-compose.yml

# Compresser
tar -czf backups/backup_$(date +%Y%m%d_%H%M%S).tar.gz backups/$(date +%Y%m%d)/
```

### Restauration

```bash
# Extraire le backup
tar -xzf backup_YYYYMMDD_HHMMSS.tar.gz

# Restaurer la base de données
docker exec -i location_db psql -U location_user location_db < backups/YYYYMMDD/database.sql

# Restaurer les fichiers
cp -r backups/YYYYMMDD/uploads/ uploads/
```

---

## 🎯 Raccourcis utiles

### Alias à ajouter dans ~/.bashrc ou ~/.zshrc (Linux/Mac)

```bash
# Raccourcis Docker
alias dc='docker-compose'
alias dcu='docker-compose up -d'
alias dcd='docker-compose down'
alias dcl='docker-compose logs -f'
alias dcr='docker-compose restart'

# Raccourcis projet
alias backend='cd /path/to/location-platform/backend'
alias frontend='cd /path/to/location-platform/frontend'
alias location='cd /path/to/location-platform'

# Raccourcis cargo
alias cr='cargo run'
alias ct='cargo test'
alias cb='cargo build'
alias cw='cargo watch -x run'
```

Après avoir ajouté ces alias :

```bash
source ~/.bashrc  # ou ~/.zshrc
```

### PowerShell Aliases (Windows)

Ajouter dans `$PROFILE` :

```powershell
# Raccourcis Docker
function dc { docker-compose $args }
function dcu { docker-compose up -d $args }
function dcd { docker-compose down $args }
function dcl { docker-compose logs -f $args }

# Raccourcis navigation
function backend { Set-Location C:\path\to\location-platform\backend }
function frontend { Set-Location C:\path\to\location-platform\frontend }
function location { Set-Location C:\path\to\location-platform }
```

---

## 🆘 Problèmes courants

### Le port est déjà utilisé

```bash
# Trouver le processus utilisant le port 8000
# Linux/Mac
lsof -i :8000

# Windows
netstat -ano | findstr :8000

# Tuer le processus
kill -9 PID  # Linux/Mac
taskkill /PID PID /F  # Windows
```

### Problèmes de permissions Docker

```bash
# Linux - Ajouter l'utilisateur au groupe docker
sudo usermod -aG docker $USER

# Redémarrer la session ou exécuter
newgrp docker
```

### Réinitialisation complète

```bash
# Arrêter tous les conteneurs
docker-compose down -v

# Supprimer les images
docker rmi location-backend location-frontend

# Nettoyer complètement Docker
docker system prune -a --volumes

# Rebuild from scratch
docker-compose up -d --build
```

---

## 📚 Ressources

* [Documentation Rust](https://doc.rust-lang.org/)
* [Documentation Axum](https://docs.rs/axum/latest/axum/)
* [Documentation React](https://react.dev/)
* [Documentation Docker](https://docs.docker.com/)
* [Documentation PostgreSQL](https://www.postgresql.org/docs/)

---

**Gardez ce guide à portée de main pour un développement efficace ! 🚀**
