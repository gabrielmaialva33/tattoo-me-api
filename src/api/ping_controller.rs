use actix_web::{HttpResponse, Responder, get};
use serde_json::json;
use serde::{Deserialize, Serialize};

fn s_ping() -> &'static str {
    "pong"
}

#[derive(Serialize,Deserialize)]
struct Ping {
    pong: &'static str,
    ms: f64
}

#[get("/")]
pub async fn ping() -> impl Responder {
    let start = std::time::Instant::now();
    let pong = s_ping();

    let ms = start.elapsed().as_millis();
    let response = json!(Ping {
        pong,
        ms: ms as f64
    });

    HttpResponse::Ok().json(response)
}