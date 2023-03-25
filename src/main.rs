use axum::{routing::get, Router};
use ngrok::{config::OauthOptions, prelude::*};
use rustingwithngrok::AppState;
use std::{env, net::SocketAddr, process::exit, sync::Arc};

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/ws", get(routes::websocket))
        .with_state(app_state);

    let ngrok_token = match env::var("NGROK_AUTHTOKEN") {
        Ok(t) => t,
        Err(_) => {
            eprintln!("NGROK_AUTHTOKEN environment variable must be set");
            exit(1)
        }
    };

    let tunnel = ngrok::Session::builder()
        .authtoken(ngrok_token)
        .connect()
        .await?
        .http_endpoint()
        .oauth(OauthOptions::new("google"))
        .listen()
        .await?;

    log::info!("Tunnel started on URL: {:?}", tunnel.url());

    axum::Server::builder(tunnel)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
