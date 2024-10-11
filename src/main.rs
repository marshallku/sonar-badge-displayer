use env::{app::Env, config::set_config_as_env_vars};
use log::info;
use routes::app::app;
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utils::log::trace_layer_on_request;

mod api;
mod env;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    set_config_as_env_vars();

    let env = Env::new();
    let state = env::state::AppState::from(env.clone());
    let app = app()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_request(trace_layer_on_request),
        )
        .with_state(state);

    let address = format!("{}:{}", env.host, env.port);
    let listener = TcpListener::bind(&address).await.unwrap();

    info!("Server running on http://{}", address);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
