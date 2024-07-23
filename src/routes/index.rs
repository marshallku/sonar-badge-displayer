use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::IntoResponse,
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{api::project_badges::get_project_badges, env::state::AppState};

#[derive(Deserialize)]
pub struct BadgeOption {
    project: String,
    metric: String,
}

pub async fn get(query: Query<BadgeOption>, State(state): State<AppState>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "image/svg+xml".parse().unwrap());

    let badge = get_project_badges(query.project.to_string(), query.metric.to_string(), &state)
        .await
        .unwrap();

    (StatusCode::OK, headers, badge).into_response()
}

