# Architecture Technique : Task Manager DevOps

Ce document décrit l'architecture système et logicielle du projet. L'application repose sur une architecture moderne en 3-tiers (Frontend, Backend, Base de données), entièrement conteneurisée pour garantir une stricte parité entre les environnements de développement et de production.

## 1. Vue d'ensemble (High-Level Architecture)

L'application suit un modèle client-serveur découplé :
- **Tier 1 (Présentation) :** SPA Svelte 5 servie par NGINX.
- **Tier 2 (Logique Métier) :** API RESTful en Rust (Axum).
- **Tier 3 (Persistance) :** Base de données relationnelle PostgreSQL 18.

## 2. Composants Détaillés

### 2.1. Frontend (Svelte 5 / Vite)
- **Framework :** Single Page Application (SPA) développée en Svelte 5 pour une réactivité granulaire et sans Virtual DOM.
- **Build Tool :** Vite pour un HMR (Hot Module Replacement) ultra-rapide en développement et un bundling optimisé (Rollup) pour la production.
- **Serveur Web :** Les assets statiques compilés sont servis par un serveur **NGINX** allégé (basé sur Alpine Linux), configuré pour router toutes les requêtes non trouvées vers `index.html` (comportement standard SPA).

### 2.2. Backend (Rust / Axum)
- **Framework Web :** Axum (basé sur l'écosystème Tokio/Hyper), offrant des performances maximales, un routage asynchrone type-safe et une empreinte mémoire minimale.
- **Sécurité & Compilation :** Utilisation de `rustls` pour la gestion de la cryptographie, permettant une compilation statique complète du binaire (idéal pour des images Docker distroless ou scratch).
- **Rôle :** Expose les endpoints CRUD de l'API REST (`/tasks`) et gère la logique d'accès aux données.

### 2.3. Base de Données (PostgreSQL)
- **Moteur :** PostgreSQL 18, assurant la durabilité et la conformité ACID des transactions métier.
- **Persistance :** Les données sont stockées sur des volumes Docker persistants pour éviter la perte d'état lors du redémarrage des conteneurs.

## 3. Infrastructure & Conteneurisation (Docker)

L'orchestration locale et de production est gérée par Docker et Docker Compose.

- **Isolation :** Chaque composant (Frontend NGINX, Backend Rust, et DB PostgreSQL) tourne dans son propre conteneur.
- **Réseau :** Les conteneurs communiquent via un réseau Docker interne privé. Seul le port d'entrée NGINX (ou l'API selon le reverse proxy global) est exposé à l'hôte.
- **Parité Dev/Prod :** La stack définie dans le `docker-compose.yml` garantit que l'environnement testé par la pipeline de CI/CD (GitHub Actions) est structurellement identique à celui de production.

## 4. Topologie Monorepo

La structure du dépôt centralise l'infrastructure et le code applicatif :

```text
.
├── .github/workflows/   # Pipelines CI/CD (Lint, Build, Deploy)
├── .husky/              # Hooks Git (pre-commit : check Rust & Svelte)
├── backend/             # Code source Rust (Axum)
│   ├── src/
│   ├── Cargo.toml
│   └── Dockerfile       # Build multi-stage Rust
├── frontend/            # Code source Svelte 5
│   ├── src/
│   ├── package.json
│   └── Dockerfile       # Build Node.js + Serve NGINX
├── docs/                # Documentation projet
└── docker-compose.yml   # Orchestration des services
```
