use axum::{extract::State, response::IntoResponse};

use crate::env::state::AppState;

pub async fn get(State(state): State<AppState>) -> impl IntoResponse {
    "Hello, World!".into_response()
}
