# 🚀 Guide de Déploiement - Location Platform

Ce guide détaille les étapes pour déployer l'application Location en production.

---

## 📋 Table des Matières

* [Prérequis](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#pr%C3%A9requis)
* [Préparation](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#pr%C3%A9paration)
* [Déploiement avec Docker](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#d%C3%A9ploiement-avec-docker)
* [Configuration Nginx](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#configuration-nginx)
* [SSL avec Let&#39;s Encrypt](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#ssl-avec-lets-encrypt)
* [Maintenance](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#maintenance)
* [Monitoring](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#monitoring)
* [Sauvegarde](https://claude.ai/chat/61feac61-9f60-4f56-bfe3-7cafea567e02#sauvegarde)

---

## 🔧 Prérequis

### Serveur

* **OS** : Ubuntu 22.04 LTS (recommandé)
* **RAM** : Minimum 2GB (4GB recommandé)
* **CPU** : 2 vCPU minimum
* **Stockage** : 20GB minimum
* **Nom de domaine** : configuré et pointant vers votre serveur

### Logiciels

```bash
# Mettre à jour le système
sudo apt update && sudo apt upgrade -y

# Installer Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER

# Installer Docker Compose
sudo apt install docker-compose-plugin -y

# Installer Git
sudo apt install git -y

# Installer Nginx (pour le reverse proxy)
sudo apt install nginx -y

# Installer Certbot (pour SSL)
sudo apt install certbot python3-certbot-nginx -y
```

---

## 📦 Préparation

### 1. Cloner le projet

```bash
cd /opt
sudo git clone https://github.com/votre-username/location-platform.git
cd location-platform
sudo chown -R $USER:$USER .
```

### 2. Configuration des variables d'environnement

#### Backend

```bash
cd backend
cp .env.example .env
nano .env
```

**Configuration production** :

```env
DATABASE_URL=postgres://location_user:VOTRE_MOT_DE_PASSE_SECURISE@postgres:5432/location_db
JWT_SECRET=VOTRE_CLE_SECRETE_SUPER_LONGUE_ET_ALEATOIRE_CHANGEZ_MOI
JWT_EXPIRATION=3600
PORT=8000
RUST_LOG=info
CORS_ORIGIN=https://votre-domaine.com
```

#### Frontend

```bash
cd ../frontend
echo "VITE_API_URL=https://votre-domaine.com/api" > .env
```

### 3. Créer docker-compose.prod.yml

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    container_name: location_db_prod
    environment:
      POSTGRES_USER: location_user
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: location_db
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: always
    networks:
      - location_network

  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    container_name: location_backend_prod
    environment:
      DATABASE_URL: ${DATABASE_URL}
      JWT_SECRET: ${JWT_SECRET}
      JWT_EXPIRATION: 3600
      PORT: 8000
      RUST_LOG: info
      CORS_ORIGIN: ${CORS_ORIGIN}
    depends_on:
      - postgres
    restart: always
    networks:
      - location_network

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
    container_name: location_frontend_prod
    depends_on:
      - backend
    restart: always
    networks:
      - location_network

  nginx:
    image: nginx:alpine
    container_name: location_nginx
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/nginx/ssl:ro
      - /etc/letsencrypt:/etc/letsencrypt:ro
    depends_on:
      - frontend
      - backend
    restart: always
    networks:
      - location_network

volumes:
  postgres_data:

networks:
  location_network:
    driver: bridge
```

---

## 🐳 Déploiement avec Docker

### 1. Build et démarrage

```bash
# À la racine du projet
docker-compose -f docker-compose.prod.yml up -d --build
```

### 2. Vérifier le statut

```bash
docker-compose -f docker-compose.prod.yml ps
docker-compose -f docker-compose.prod.yml logs -f
```

### 3. Vérifier l'API

```bash
curl http://localhost:8000/health
```

---

## 🔒 Configuration Nginx

### Configuration complète

Créer `/etc/nginx/sites-available/location` :

```nginx
# Redirection HTTP vers HTTPS
server {
    listen 80;
    listen [::]:80;
    server_name votre-domaine.com www.votre-domaine.com;
  
    # Acme challenge pour Let's Encrypt
    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }
  
    location / {
        return 301 https://$server_name$request_uri;
    }
}

# Configuration HTTPS
server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name votre-domaine.com www.votre-domaine.com;

    # Certificats SSL
    ssl_certificate /etc/letsencrypt/live/votre-domaine.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/votre-domaine.com/privkey.pem;
  
    # Configuration SSL moderne
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;

    # Headers de sécurité
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript application/javascript application/xml+rss application/json;

    # Logs
    access_log /var/log/nginx/location_access.log;
    error_log /var/log/nginx/location_error.log;

    # API Backend
    location /api/ {
        proxy_pass http://localhost:8000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
        proxy_read_timeout 90;
    }

    # Frontend
    location / {
        proxy_pass http://localhost:5173;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

### Activer la configuration

```bash
sudo ln -s /etc/nginx/sites-available/location /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## 🔐 SSL avec Let's Encrypt

### Installation du certificat

```bash
# Obtenir le certificat
sudo certbot --nginx -d votre-domaine.com -d www.votre-domaine.com

# Tester le renouvellement automatique
sudo certbot renew --dry-run
```

### Renouvellement automatique

Le renouvellement est automatique via un cron job créé par certbot.

Vérifier :

```bash
sudo systemctl status certbot.timer
```

---

## 🛠 Maintenance

### Mise à jour de l'application

```bash
cd /opt/location-platform

# Récupérer les dernières modifications
git pull origin main

# Rebuild et redémarrer
docker-compose -f docker-compose.prod.yml down
docker-compose -f docker-compose.prod.yml up -d --build

# Vérifier les logs
docker-compose -f docker-compose.prod.yml logs -f
```

### Redémarrage des services

```bash
# Redémarrer un service spécifique
docker-compose -f docker-compose.prod.yml restart backend

# Redémarrer tous les services
docker-compose -f docker-compose.prod.yml restart
```

### Voir les logs

```bash
# Tous les services
docker-compose -f docker-compose.prod.yml logs -f

# Service spécifique
docker-compose -f docker-compose.prod.yml logs -f backend

# Dernières 100 lignes
docker-compose -f docker-compose.prod.yml logs --tail=100 backend
```

---

## 📊 Monitoring

### Installation de monitoring basique

```bash
# Installer htop pour monitoring système
sudo apt install htop -y

# Surveiller l'utilisation Docker
docker stats
```

### Vérification de santé

Script `health-check.sh` :

```bash
#!/bin/bash

# Health check API
API_STATUS=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8000/health)

if [ $API_STATUS -eq 200 ]; then
    echo "✅ API is healthy"
else
    echo "❌ API is down (Status: $API_STATUS)"
    # Redémarrer l'API
    docker-compose -f docker-compose.prod.yml restart backend
fi

# Check database
DB_STATUS=$(docker exec location_db_prod pg_isready -U location_user)
if [[ $DB_STATUS == *"accepting connections"* ]]; then
    echo "✅ Database is healthy"
else
    echo "❌ Database is down"
fi
```

Rendre exécutable et ajouter au cron :

```bash
chmod +x health-check.sh
crontab -e

# Ajouter cette ligne pour vérifier toutes les 5 minutes
*/5 * * * * /opt/location-platform/health-check.sh >> /var/log/health-check.log 2>&1
```

---

## 💾 Sauvegarde

### Script de sauvegarde de la base de données

`backup-db.sh` :

```bash
#!/bin/bash

BACKUP_DIR="/opt/backups/location"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_FILE="$BACKUP_DIR/location_db_$TIMESTAMP.sql"

# Créer le répertoire de sauvegarde
mkdir -p $BACKUP_DIR

# Sauvegarder la base de données
docker exec location_db_prod pg_dump -U location_user location_db > $BACKUP_FILE

# Compresser la sauvegarde
gzip $BACKUP_FILE

echo "✅ Backup created: $BACKUP_FILE.gz"

# Supprimer les sauvegardes de plus de 30 jours
find $BACKUP_DIR -name "*.gz" -mtime +30 -delete

echo "✅ Old backups cleaned"
```

Automatiser :

```bash
chmod +x backup-db.sh

# Sauvegarde quotidienne à 2h du matin
crontab -e
0 2 * * * /opt/location-platform/backup-db.sh >> /var/log/backup.log 2>&1
```

### Restauration

```bash
# Décompresser
gunzip location_db_YYYYMMDD_HHMMSS.sql.gz

# Restaurer
docker exec -i location_db_prod psql -U location_user location_db < location_db_YYYYMMDD_HHMMSS.sql
```

---

## 🔥 Firewall

### Configuration UFW

```bash
# Activer le firewall
sudo ufw enable

# Autoriser SSH
sudo ufw allow 22/tcp

# Autoriser HTTP/HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Vérifier le statut
sudo ufw status
```

---

## 📝 Checklist de déploiement

* [ ] Serveur configuré avec Ubuntu 22.04
* [ ] Docker et Docker Compose installés
* [ ] Nom de domaine configuré
* [ ] Variables d'environnement configurées
* [ ] Application buildée et démarrée
* [ ] Nginx configuré en reverse proxy
* [ ] Certificat SSL installé
* [ ] Firewall configuré
* [ ] Sauvegardes automatisées
* [ ] Monitoring en place
* [ ] Tests de charge effectués

---

## 🆘 Dépannage

### L'API ne répond pas

```bash
# Vérifier les logs
docker-compose logs backend

# Redémarrer le service
docker-compose restart backend
```

### Problème de base de données

```bash
# Se connecter à PostgreSQL
docker exec -it location_db_prod psql -U location_user -d location_db

# Vérifier les tables
\dt

# Quitter
\q
```

### Problème de certificat SSL

```bash
# Renouveler manuellement
sudo certbot renew --force-renewal

# Recharger Nginx
sudo systemctl reload nginx
```

---

**Bon déploiement ! 🚀**
