use env::env::Env;
use log::info;
use routes::app::app;
use tokio::net::TcpListener;

mod env;
mod routes;

#[tokio::main]
async fn main() {
    let env = Env::new();
    let state = env::state::AppState::from(env.clone());
    let app = app().with_state(state);

    let address = format!("{}:{}", env.host, env.port);
    let listener = TcpListener::bind(&address).await.unwrap();

    info!("Server running on {}", address);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
