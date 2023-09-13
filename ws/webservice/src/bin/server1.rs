use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// route handlers
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// route handler functions
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix web service is up and running!")
}

// server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // start http server
    let app = move || App::new().configure(general_routes);

    // start http server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}