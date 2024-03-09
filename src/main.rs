use std::env;

use actix_web::{App, HttpServer};

mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to read .env file");

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    HttpServer::new(|| {
        App::new().service(api::ping_controller::ping)
        //.route("/", web::get().to(api::ping_controller::ping))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
