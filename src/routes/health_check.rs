use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}