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

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    // print!("{}", &_form.name);
    // print!("{}", &_form.email);

    if !form.name.is_empty() && !form.email.is_empty() {
        return HttpResponse::Ok().finish();
    }

    HttpResponse::Ok().finish()
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener) -> Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscription", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // .await

    Ok(server)
}
