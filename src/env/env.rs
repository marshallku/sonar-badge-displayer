use core::panic;
use dotenv::dotenv;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Env {
    pub host: Cow<'static, str>,
    pub port: Cow<'static, str>,
    pub sonarqube_url: Cow<'static, str>,
    pub sonarqube_token: Cow<'static, str>,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();

        let sonarqube_url = match std::env::var("SONARQUBE_URL") {
            Ok(url) => Cow::Owned(url),
            Err(_) => panic!("SONARQUBE_URL is not set"),
        };
        let sonarqube_token = match std::env::var("SONARQUBE_TOKEN") {
            Ok(url) => Cow::Owned(url),
            Err(_) => panic!("SONARQUBE_TOKEN is not set"),
        };
        let host = match std::env::var("HOST") {
            Ok(url) => Cow::Owned(url),
            Err(_) => Cow::Borrowed("127.0.0.1"),
        };
        let port = match std::env::var("PORT") {
            Ok(url) => Cow::Owned(url),
            Err(_) => Cow::Borrowed("3000"),
        };

        Self {
            host,
            port,
            sonarqube_url,
            sonarqube_token,
        }
    }
}
