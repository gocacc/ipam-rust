use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::EnvLogger;
use tera::Tera;

mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    EnvLogger::init();

    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(|| {
        App::new()
            .data(tera.clone())
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
