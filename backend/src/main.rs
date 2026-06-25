use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Task {
    id: i64,
    title: String,
    description: Option<String>,
    task_type: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
    description: Option<String>,
    task_type: String,
}

#[derive(Deserialize)]
struct UpdateTask {
    title: Option<String>,
    description: Option<String>,
    task_type: Option<String>,
    completed: Option<bool>,
}

#[derive(Clone)]
struct AppState {
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

async fn list_tasks(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
    Json(tasks)
}

async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Task>, StatusCode> {
    sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_or(Err(StatusCode::NOT_FOUND), |t| Ok(Json(t)))
}

async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let result = sqlx::query("INSERT INTO tasks (title, description, task_type) VALUES (?, ?, ?)")
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(&payload.task_type)
        .execute(&state.db)
        .await;

    match result {
        Ok(res) => {
            let id = res.last_insert_rowid();
            let task = Task {
                id,
                title: payload.title,
                description: payload.description,
                task_type: payload.task_type,
                completed: false,
            };
            (StatusCode::CREATED, Json(task)).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn update_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, StatusCode> {
    let existing = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let title = payload.title.unwrap_or(existing.title);
    let description = payload.description.or(existing.description);
    let task_type = payload.task_type.unwrap_or(existing.task_type);
    let completed = payload.completed.unwrap_or(existing.completed);

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

async fn delete_task(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> StatusCode {
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_or(StatusCode::INTERNAL_SERVER_ERROR, |_| {
            StatusCode::NO_CONTENT
        })
}
