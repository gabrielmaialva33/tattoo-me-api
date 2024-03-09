use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
