
# Task Manager DevOps

![CI](https://github.com/QuentinGouttaya/devops-tp-quentin-gouttaya/actions/workflows/ci.yml/badge.svg)
![CI](https://github.com/USER/REPO/actions/workflows/ci.yml/badge.svg)

Un gestionnaire de tâches CRUD simple et fortement typé, conçu pour valider une pipeline CI/CD et des déploiements conteneurisés.

## Stack

- **Backend :** Rust (Axum, SQLx)
- **Base de données :** SQLite
- **Frontend :** Svelte 5 (SPA)
- **Déploiement :** Docker & Docker Compose

## 🚀 Lancer le projet

### Pré-requis

Assurez-vous d'avoir installé les outils suivants sur votre machine locale :

- **Docker** et **Docker Compose plugin** (via Docker Desktop ou Docker Engine)
- **Git** (pour cloner le dépôt)
*(Node.js et Rust ne sont pas nécessaires grâce à l'isolation Docker)*

### Déploiement

Lancez l'intégralité de la stack (Frontend, Backend, Base de données) en 4 commandes :

```bash
git clone <URL_DE_VOTRE_DEPOT>
cd devops-tp-quentin-gouttaya
cp .env.example .env
docker compose up -d --build
```

*Note : N'oubliez pas d'éditer le fichier `.env` si des variables spécifiques sont requises par votre application.*

### Accès aux services

Une fois les conteneurs démarrés, l'application est accessible via :

- 🌐 **Frontend (Svelte/NGINX)** : [http://localhost](http://localhost)
- ⚙️ **Backend (Rust/Axum)** : [http://localhost:3000](http://localhost:3000)

## Tester

Testez le endpoint principal `/tasks` de l'API REST en utilisant `curl` :

```bash
# Ajouter une tâche
curl -X POST http://localhost:3000/tasks \
     -H "Content-Type: application/json" \
     -d '{"title": "Configurer le pipeline GitHub Actions", "completed": false}'

# Récupérer toutes les tâches
curl -X GET http://localhost:3000/tasks
```

## Architecture

Le projet repose sur une architecture découplée avec une API REST en backend et une Single Page Application en frontend, le tout isolé via des conteneurs.
Les détails des flux de données et du schéma de base de données se trouvent ici : [docs/architecture.md](docs/architecture.md).

## Auteur

Quentin Gouttaya
