use std::path::PathBuf;

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::Response,
    response::{Html, IntoResponse, Json},
    routing::get,
    serve, Router,
};
use env_logger::init;
use metmac::storage::connection::Database;
use serde_json::json;

enum AppError {
    DatabaseError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DatabaseError(message) => {
                let payload = json!({"error": message});
                (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response()
            }
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init(); // Init env logger

    // TODO: Make configurable path
    let db = Database::new(PathBuf::from("~/.metmac/data.db")).await?;
    db.run_migrations().await?;

    let app = Router::new()
        .route("/", get(serve_dashboard))
        .route("/api/stats", get(get_stats).with_state(db.clone()))
        .route(
            "/api/keyboard-stats",
            get(get_keyboard_stats).with_state(db),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3004").await?;
    serve(listener, app).await?;

    Ok(())
}

async fn serve_dashboard() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

async fn get_stats(State(db): State<Database>) -> impl IntoResponse {
    match db.get_stats().await {
        Ok(stats) => Json(stats).into_response(),
        Err(err) => AppError::from(err).into_response(),
    }
}

async fn get_keyboard_stats(State(db): State<Database>) -> impl IntoResponse {
    match db.get_keyboard_stats().await {
        Ok(stats) => Json(stats).into_response(),
        Err(err) => AppError::from(err).into_response(),
    }
}
