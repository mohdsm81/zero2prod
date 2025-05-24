// use actix_web::{App, HttpResponse, HttpServer, Responder, web};
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello, {}", &name)
// }
// async fn health_check() -> impl Responder {
//     HttpResponse::Ok().finish()
// }

// pub async fn run() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
//         .bind("0.0.0.0:80")?
//         .run()
//         .await
// }

use actix_web::dev::Server;
use std::{io::Result, net::TcpListener};

use actix_web::{App, HttpResponse, HttpServer, web};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener) -> Result<Server> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();
    // .await

    Ok(server)
}
