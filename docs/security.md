# Dans GITHUB

![Dependabot](./secuGit.jpg)

+ secret safe cause public repo

## Risques DevOps

### R1 — Clé d'API (ou Token Docker) exposée dans le code
+ **Probabilité :** Moyenne (erreur de commit fréquente)
+ **Impact :** Critique (accès non autorisé aux dépôts, facturation ou compromission d'infrastructure)
+ **Action corrective :** Utilisation systématique du fichier `.env` ajouté au `.gitignore`, configuration des `GitHub Secrets`, et activation du *Secret scanning* sur le dépôt.

### R2 — Conteneur Docker exécuté en tant que "root"
+ **Probabilité :** Élevée (comportement par défaut de Docker)
+ **Impact :** Critique (escalade de privilèges si l'application est compromise)
+ **Action corrective :** Utiliser des images de type *Distroless* ou créer un utilisateur `nonroot` avec des droits limités (ex: `USER 65532`) à la fin du Dockerfile pour le runtime.

### R3 — Défaillance ou temps d'arrêt de la base de données PostgreSQL
+ **Probabilité :** Faible à Moyenne (crash matériel, saturation disque)
+ **Impact :** Élevé (indisponibilité totale du backend)
+ **Action corrective :** Mise en place d'un système de `healthcheck` strict dans le `docker-compose.yml`, montage de volumes persistants (`volumes: db_data`), et politique de redémarrage automatique (`restart: unless-stopped`).

### R4 — Faille de sécurité critique dans une dépendance (Rust ou NPM)
+ **Probabilité :** Élevée (nouvelles vulnérabilités découvertes quotidiennement)
+ **Impact :** Modéré à Élevé (risque d'injection, exécution de code à distance)
+ **Action corrective :** Activation des alertes de sécurité (*Dependabot alerts*) et des mises à jour automatiques (*Dependabot security updates*) pour maintenir le registre `Cargo.lock` et `package-lock.json` à jour en continu.

### R5 — CI/CD excessivement lent ou bloqué
+ **Probabilité :** Élevée (sans optimisation, compilation Rust très longue)
+ **Impact :** Faible (retard de livraison, frustration de l'équipe de développement)
+ **Action corrective :** Utilisation du système de mise en cache `Swatinem/rust-cache` dans les GitHub Actions et des montages de type cache BuildKit (`--mount=type=cache`) dans les Dockerfiles.
