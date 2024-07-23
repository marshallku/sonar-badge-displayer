use super::env::Env;

#[derive(Clone)]
pub struct AppState {
    pub sonarqube_url: String,
    pub sonarqube_token: String,
}

impl AppState {
    pub fn from(env: Env) -> Self {
        Self {
            sonarqube_url: env.sonarqube_url.into_owned(),
            sonarqube_token: env.sonarqube_token.into_owned(),
        }
    }
}
