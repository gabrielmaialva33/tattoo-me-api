mod api;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");

    HttpServer::new(|| {
        App::new().service(api::ping_controller::ping)
            //.route("/", web::get().to(api::ping_controller::ping))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
