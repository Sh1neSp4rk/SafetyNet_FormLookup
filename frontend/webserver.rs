use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::fs;

async fn index() -> impl Responder {
    // Read the contents of the index.html file
    let contents = fs::read_to_string("frontend/templates/index.html")
        .unwrap_or_else(|_| String::from("Error reading index.html"));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(contents)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Define routes
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}