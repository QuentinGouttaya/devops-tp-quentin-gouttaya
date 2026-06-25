use sqlx::SqlitePool;
use std::sync::Arc;
use task_manager_backend::*; // Importe les structs et fonctions

#[tokio::test]
async fn should_create_and_retrieve_task_with_mocked_db() {
    // ARRANGE - Mock de la DB avec sqlite::memory:
    let db = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory DB");

    // Initialiser le schéma
    sqlx::query(include_str!("../migrations/001_create_tasks.sql"))
        .execute(&db)
        .await
        .expect("Failed to run migrations");

    let state = Arc::new(AppState { db });

    // Créer une tâche via le handler (simule un appel API)
    let payload = CreateTask {
        title: "Test task".to_string(),
        description: Some("Description".to_string()),
        task_type: "feature".to_string(),
    };

    // ACT - Appeler create_task (pas d'appel réseau, tout en mémoire)
    let create_response = create_task(State(state.clone()), Json(payload))
        .await
        .into_response();

    assert_eq!(create_response.status(), StatusCode::CREATED);

    // Récupérer la tâche créée
    let get_response = get_task(State(state), Path(1))
        .await
        .expect("Failed to get task");

    // ASSERT
    assert_eq!(get_response.title, "Test task");
    assert_eq!(get_response.task_type, "feature");
    assert!(!get_response.completed);
}
