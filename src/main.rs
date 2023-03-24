use std::sync::{Arc, Mutex};

use actix::{Actor, Addr};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use askama::Template;
use askama_actix::TemplateToResponse;

mod client;
mod messages;
mod server;
use client::*;
use messages::*;
use server::*;

#[derive(Template)]
#[template(path = "./index.html")]
struct IndexTemplate {
    chat_messages: Vec<ChatMsg>,
}

#[get("/ws")]
async fn websocket_endpoint(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let username = match req.headers().get("ngrok-auth-user-name") {
        Some(header) => header.to_str().unwrap_or("None").to_string(),
        None => "None".to_string(),
    };

    ws::start(
        Client {
            id: rand::random(),
            username,
            addr: server.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[get("/")]
async fn index(chat_messages: web::Data<Arc<Mutex<Vec<ChatMsg>>>>) -> Result<HttpResponse, Error> {
    let chat_messages = chat_messages.into_inner();
    let chat_messages = chat_messages.lock().unwrap().to_vec();
    Ok(IndexTemplate { chat_messages }.to_response())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let chat_messages = Arc::new(Mutex::new(Vec::<ChatMsg>::new()));
    let server = Server::new(chat_messages.clone()).start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(chat_messages.clone()))
            .app_data(web::Data::new(server.clone()))
            .service(index)
            .service(websocket_endpoint)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
