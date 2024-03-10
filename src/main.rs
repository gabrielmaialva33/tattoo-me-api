use actix_web::{App, HttpServer};
use log::info;

mod api;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to read .env file");
    pretty_env_logger::init();

    info!("Starting server...");

    HttpServer::new(|| {
        App::new().service(api::ping_controller::ping)
        //.route("/", web::get().to(api::ping_controller::ping))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
