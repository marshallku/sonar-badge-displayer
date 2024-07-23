use log::error;
use reqwest::{get, Error};

use crate::env::state::AppState;

pub async fn get_project_badges(
    project_name: String,
    metric: String,
    state: &AppState,
) -> Result<String, Error> {
    let url = format!(
        "https://{}/api/project_badges/measure?project={}&metric={}&token={}",
        state.sonarqube_url, project_name, metric, state.sonarqube_token
    );
    let response = match get(url.clone()).await {
        Ok(response) => response,
        Err(_) => {
            error!("Error fetching data: {}", url);
            return Ok("Error fetching data".to_string());
        }
    };

    return Ok(response.text().await.unwrap());
}
