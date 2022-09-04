use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run()
        .await
}
