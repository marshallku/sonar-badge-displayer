use log::error;
use reqwest::{get, Error};

pub async fn get_project_badges(
    project_name: &str,
    metric: &str,
    sonarqube_url: &str,
    token: &str,
) -> Result<String, Error> {
    let url = format!(
        "https://{}/api/project_badges/measure?project={}&metric={}&token={}",
        sonarqube_url, project_name, metric, token
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
