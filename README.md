
# Task Manager DevOps

![CI](https://github.com/QuentinGouttaya/devops-tp-quentin-gouttaya/actions/workflows/ci.yml/badge.svg)
![CI](https://github.com/USER/REPO/actions/workflows/ci.yml/badge.svg)

Un gestionnaire de tâches CRUD simple et fortement typé, conçu pour valider une pipeline CI/CD et des déploiements conteneurisés.

## Stack

- **Backend :** Rust (Axum, SQLx)
- **Base de données :** SQLite
- **Frontend :** Svelte 5 (SPA)
- **Déploiement :** Docker & Docker Compose

## Lancer le projet

**Via Docker (Recommandé) :**

```bash
docker-compose up -d --build
```

**En local (Développement) :**

```bash
# Terminal 1 : Lancer le backend Rust
cd backend
cargo run

# Terminal 2 : Lancer le frontend Svelte 5
cd frontend
npm install
npm run dev
```

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
