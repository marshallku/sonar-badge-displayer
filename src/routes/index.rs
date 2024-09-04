use std::env::var;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};

use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    api::project_badges::get_project_badges, env::state::AppState,
    utils::http::generate_header_with_age,
};

#[derive(Deserialize)]
pub struct BadgeOption {
    project: String,
    metric: String,
}

pub async fn get(query: Query<BadgeOption>, State(state): State<AppState>) -> impl IntoResponse {
    let mut headers = generate_header_with_age("1h");

    headers.insert("Content-Type", "image/svg+xml".parse().unwrap());

    let token = var(query.project.clone()).unwrap_or("".to_string());
    let badge = get_project_badges(&query.project, &query.metric, &state.sonarqube_url, &token)
        .await
        .unwrap();

    (StatusCode::OK, headers, badge).into_response()
}
