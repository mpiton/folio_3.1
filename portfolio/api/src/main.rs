mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use log::info;

use services::db::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Starting server...");

    // Initialize database
    let db_pool = init_db().await.expect("Failed to initialize database");

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(routes::health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
