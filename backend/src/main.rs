use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    id: i64,
    title: String,
    description: Option<String>,
    task_type: String,
    completed: bool,
}

#[derive(Deserialize)]
pub struct CreateTask {
    title: String,
    description: Option<String>,
    task_type: String,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    title: Option<String>,
    description: Option<String>,
    task_type: Option<String>,
    completed: Option<bool>,
}

#[derive(Clone)]
pub struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = SqlitePool::connect("sqlite:db.sqlite3?mode=rwc").await?;
    sqlx::query(include_str!("../migrations/001_create_tasks.sql"))
        .execute(&db)
        .await?;

    let state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route(
            "/tasks/{id}",
            get(get_task).put(update_task).delete(delete_task),
        )
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

pub async fn list_tasks(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
    Json(tasks)
}

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Task>, StatusCode> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(task))
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    // Validation du titre
    if let Err(e) = validate_title(&payload.title) {
        return (StatusCode::BAD_REQUEST, e).into_response();
    }

    // Détection de titre trop court
    if is_title_too_short(&payload.title) {
        return (
            StatusCode::BAD_REQUEST,
            "Title must be at least 3 characters",
        )
            .into_response();
    }

    // Normalisation du type
    let normalized_type = normalize_task_type(&payload.task_type);

    let result = sqlx::query("INSERT INTO tasks (title, description, task_type) VALUES (?, ?, ?)")
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(&normalized_type)
        .execute(&state.db)
        .await;

    match result {
        Ok(res) => {
            let id = res.last_insert_rowid();
            let task = Task {
                id,
                title: payload.title,
                description: payload.description,
                task_type: normalized_type,
                completed: false,
            };
            (StatusCode::CREATED, Json(task)).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
pub async fn update_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, StatusCode> {
    // ÉTAPE 1 : Validation AVANT extraction (emprunt avec &)
    if let Some(title) = &payload.title {
        if validate_title(title).is_err() {
            return Err(StatusCode::BAD_REQUEST);
        }
        if is_title_too_short(title) {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    // ÉTAPE 2 : Récupérer la tâche existante
    let existing = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // ÉTAPE 3 : Extraction des valeurs (maintenant safe)
    let title = payload.title.unwrap_or(existing.title);
    let description = payload.description.or(existing.description);
    let task_type = payload
        .task_type
        .map(|t| normalize_task_type(&t))
        .unwrap_or(existing.task_type);
    let completed = payload.completed.unwrap_or(existing.completed);

    // ÉTAPE 4 : Mise à jour en base
    sqlx::query(
        "UPDATE tasks SET title = ?, description = ?, task_type = ?, completed = ? WHERE id = ?",
    )
    .bind(&title)
    .bind(&description)
    .bind(&task_type)
    .bind(completed)
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let updated = Task {
        id,
        title,
        description,
        task_type,
        completed,
    };
    Ok(Json(updated))
}
///Testing helper functions => to be split
/// Valide qu'un titre de tâche n'est pas vide
pub fn validate_title(title: &str) -> Result<(), String> {
    if title.trim().is_empty() {
        Err("Title cannot be empty".to_string())
    } else {
        Ok(())
    }
}

pub async fn delete_task(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> StatusCode {
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_or(StatusCode::INTERNAL_SERVER_ERROR, |_| {
            StatusCode::NO_CONTENT
        })
}

/// Normalise le type de tâche en lowercase
pub fn normalize_task_type(task_type: &str) -> String {
    task_type.to_lowercase().trim().to_string()
}

/// Détecte si le titre est trop court (moins de 3 caractères)
pub fn is_title_too_short(title: &str) -> bool {
    title.trim().len() < 3
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    // Test 1 : validation du titre (valide / invalide)
    #[test]
    fn should_reject_empty_title_when_title_is_blank() {
        // ARRANGE
        let empty_title = "   ";

        // ACT
        let result = validate_title(empty_title);

        // ASSERT
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Title cannot be empty");
    }

    #[test]
    fn should_accept_valid_title_when_title_has_content() {
        // ARRANGE
        let valid_title = "Learn Rust";

        // ACT
        let result = validate_title(valid_title);

        // ASSERT
        assert!(result.is_ok());
    }

    // Test 2 : normalisation du type de tâche
    #[test]
    fn should_normalize_task_type_when_mixed_case() {
        // ARRANGE
        let messy_type = "  FEATURE  ";

        // ACT
        let normalized = normalize_task_type(messy_type);

        // ASSERT
        assert_eq!(normalized, "feature");
    }

    // Test 3 : détection de titre trop court
    #[test]
    fn should_detect_title_too_short_when_less_than_3_chars() {
        // ARRANGE
        let short_title = "ab";

        // ACT
        let is_too_short = is_title_too_short(short_title);

        // ASSERT
        assert!(is_too_short);
    }

    #[test]
    fn should_accept_title_when_3_chars_or_more() {
        // ARRANGE
        let valid_title = "abc";

        // ACT
        let is_too_short = is_title_too_short(valid_title);

        // ASSERT
        assert!(!is_too_short);
    }
}
