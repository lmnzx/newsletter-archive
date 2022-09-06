pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

#[allow(dead_code)]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[allow(dead_code)]
pub async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    println!("Email - {}", form.email);
    println!("Name - {}", form.name);
    HttpResponse::Ok().finish()
}

#[allow(dead_code)]
pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run()
    .await
}
