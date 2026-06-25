# Stratégie de tests

## Fonctionnalités critiques

### 1. Création de tâche (POST /tasks)

**Pourquoi critique ?**
C'est le point d'entrée principal de l'application. Sans création de tâche, l'utilisateur ne peut rien faire. Une régression ici bloque tout le workflow.

**Tests identifiés :**

#### Test unitaire : validation du payload
- **Fichier** : `backend/src/main.rs` (module `tests`)
- **Nom** : `should_reject_empty_title`
- **Description** : Vérifie qu'une tâche avec un titre vide est rejetée
- **Pattern AAA** :
  - ARRANGE : Créer un payload avec `title: ""`
  - ACT : Appeler la fonction de validation
  - ASSERT : Retourne une erreur de validation

#### Test d'intégration : création complète
- **Fichier** : `backend/tests/integration_tests.rs`
- **Nom** : `should_create_task_and_return_201`
- **Description** : Vérifie qu'un POST valide crée la tâche en base et retourne 201
- **Setup** : Base SQLite en mémoire (`sqlite::memory:`)
- **Pattern AAA** :
  - ARRANGE : Initialiser la DB, créer un payload valide
  - ACT : Appeler `create_task()`
  - ASSERT : Status 201 + tâche retournée avec ID généré

#### Test bonus (e2e) : création via l'API
- **Fichier** : `backend/tests/e2e_tests.rs`
- **Nom** : `should_create_task_via_http`
- **Description** : Test HTTP complet avec serveur Axum réel
- **Setup** : Lancer le serveur sur port aléatoire
- **Pattern AAA** :
  - ARRANGE : Démarrer le serveur
  - ACT : `reqwest::post("http://localhost:PORT/tasks")`
  - ASSERT : Response 201 + body contient la tâche

---

### 2. Récupération d'une tâche par ID (GET /tasks/{id})

**Pourquoi critique ?**
C'est la base de la consultation. Si cette fonctionnalité casse, l'utilisateur ne peut plus voir les détails d'une tâche. De plus, la gestion du 404 est cruciale pour l'UX.

**Tests identifiés :**

#### Test unitaire : gestion du cas "non trouvé"
- **Fichier** : `backend/src/main.rs` (module `tests`)
- **Nom** : `should_return_not_found_for_invalid_id`
- **Description** : Vérifie qu'un ID inexistant retourne 404
- **Pattern AAA** :
  - ARRANGE : DB vide, ID = 999
  - ACT : Appeler `get_task()`
  - ASSERT : Retourne `Err(StatusCode::NOT_FOUND)`

#### Test d'intégration : récupération réussie
- **Fichier** : `backend/tests/integration_tests.rs`
- **Nom** : `should_return_task_when_exists`
- **Description** : Crée une tâche puis la récupère par son ID
- **Setup** : Base SQLite en mémoire avec 1 tâche insérée
- **Pattern AAA** :
  - ARRANGE : Insérer une tâche manuellement
  - ACT : Appeler `get_task()` avec son ID
  - ASSERT : Status 200 + tâche complète retournée

#### Test bonus (e2e) : récupération via HTTP
- **Fichier** : `backend/tests/e2e_tests.rs`
- **Nom** : `should_get_task_via_http`
- **Description** : Test HTTP complet GET /tasks/1
- **Setup** : Serveur Axum + 1 tâche en base
- **Pattern AAA** :
  - ARRANGE : Démarrer serveur, insérer tâche
  - ACT : `reqwest::get("http://localhost:PORT/tasks/1")`
  - ASSERT : Response 200 + JSON valide

---

### 3. Mise à jour d'une tâche (PUT /tasks/{id})

**Pourquoi critique ?**
C'est la fonctionnalité la plus complexe car elle gère des champs optionnels (partial update). Une erreur ici peut écraser des données ou laisser des champs incohérents.

**Tests identifiés :**

#### Test unitaire : fusion des champs optionnels
- **Fichier** : `backend/src/main.rs` (module `tests`)
- **Nom** : `should_merge_optional_fields_correctly`
- **Description** : Vérifie que seuls les champs fournis sont mis à jour
- **Pattern AAA** :
  - ARRANGE : Tâche existante + payload avec seulement `title`
  - ACT : Appliquer la logique de fusion
  - ASSERT : `title` changé, `description` et `task_type` inchangés

#### Test d'intégration : mise à jour partielle
- **Fichier** : `backend/tests/integration_tests.rs`
- **Nom** : `should_update_only_provided_fields`
- **Description** : Met à jour uniquement le titre, vérifie que le reste est intact
- **Setup** : Base SQLite avec 1 tâche complète
- **Pattern AAA** :
  - ARRANGE : Tâche avec title="Old", description="Desc"
  - ACT : PUT avec `{"title": "New"}`
  - ASSERT : title="New", description="Desc" (inchangé)

#### Test bonus (e2e) : mise à jour via HTTP
- **Fichier** : `backend/tests/e2e_tests.rs`
- **Nom** : `should_update_task_via_http`
- **Description** : Test HTTP complet PUT /tasks/1
- **Setup** : Serveur Axum + tâche pré-existante
- **Pattern AAA** :
  - ARRANGE : Démarrer serveur, insérer tâche
  - ACT : `reqwest::put("http://localhost:PORT/tasks/1")` avec JSON partiel
  - ASSERT : Response 200 + tâche mise à jour

---

## Couverture cible

- **Backend** : 60% minimum (lignes couvertes par `cargo tarpaulin`)
- **Frontend** : 50% minimum (composants Svelte testés avec Vitest)

## Outils utilisés

- **Backend** : `cargo test` (natif) + `cargo-tarpaulin` (couverture)
- **Frontend** : `vitest` + `@vitest/coverage-v8`
- **Mock** : SQLite en mémoire (`sqlite::memory:`) pour isoler les tests
