use axum::{routing::get, Router};
use rustingwithngrok::AppState;
use std::{net::SocketAddr, sync::Arc};

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/ws", get(routes::websocket))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    log::info!("App started on URL: {:?}", addr.to_string());

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
