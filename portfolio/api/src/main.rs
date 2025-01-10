mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use log::info;

use services::db::init_db;
use services::rss::get_latest_tweets;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Charge les variables d'environnement depuis .env
    dotenv().ok();

    println!("MONGO_URL: {:?}", std::env::var("MONGO_URL"));

    // Initialize logger
    env_logger::init_from_env(Env::new().default_filter_or("info"));
    info!("Starting server...");

    // Test de connexion à la base
    let db = init_db().await.expect("Failed to initialize database");
    let tweets = get_latest_tweets(&db, 10)
        .await
        .expect("Failed to get tweets");
    info!(
        "Connected to database and retrieved {} tweets",
        tweets.len()
    );

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(routes::health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
