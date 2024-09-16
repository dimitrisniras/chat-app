use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use crate::database::db;

mod database;
mod init;

use init::initialize_app;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the application, including loading environment variables and database connection
    let (_db_client, port) = initialize_app().await.expect("Failed to initialize application");

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
