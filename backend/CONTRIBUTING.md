# Contribuer au projet Task Manager DevOps

Ce document décrit le workflow Git de l'équipe pour assurer une collaboration fluide, maintenir la qualité du code et garantir l'intégration continue.

## Workflow Git

Nous utilisons une approche basée sur le **Feature Branch Workflow**. La branche `main` doit toujours contenir du code stable, testé et déployable. Ne pushez jamais directement sur `main`.

### 1. Stratégie de nommage des branches
Créez toujours une nouvelle branche à partir de `main` pour isoler vos développements :
- `feat/nom-de-la-feature` : Pour une nouvelle fonctionnalité (ex: `feat/auth-jwt`).
- `fix/nom-du-bug` : Pour corriger un bug (ex: `fix/sqlite-lock`).
- `chore/nom-de-la-tache` : Pour la maintenance, la configuration CI/CD ou la mise à jour de dépendances (ex: `chore/update-rust-1.78`).
- `docs/nom-du-sujet` : Pour les ajouts ou corrections de documentation.

### 2. Conventions de Commits
Nous suivons la spécification [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/). Les messages de commit doivent être clairs pour générer des changelogs automatiques et tracer les évolutions.

**Format :** `<type>(<scope optionnel>): <description courte>`

**Exemples :**
- `feat(api): implémentation de la route POST /tasks`
- `fix(front): correction de la réactivité Svelte sur la liste`
- `ci(docker): optimisation du cache dans le Dockerfile Rust`
- `refactor(db): migration de la logique SQLx vers un repository pattern`

### 3. Processus de Pull Request (PR)
Pour fusionner votre code, suivez ces étapes :
1. Poussez votre branche sur le dépôt distant (`git push origin ma-branche`).
2. Ouvrez une Pull Request ciblant la branche `main`.
3. Remplissez la description de la PR en expliquant le "pourquoi" et le "comment" de vos modifications. Liez l'issue correspondante si elle existe (ex: `Closes #12`).
4. **Intégration Continue (CI) :** Assurez-vous que le pipeline GitHub Actions (build Docker, tests cargo, linting) réussisse.
5. **Revue de code :** Demandez une "Code Review". Au moins une approbation (*Approve*) d'un autre membre de l'équipe est requise.
6. **Merge :** Une fois approuvée, utilisez de préférence la stratégie **Squash and Merge** pour garder un historique `main` propre et linéaire.
