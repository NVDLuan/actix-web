use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use crate::configs::swagger::swagger_routes;
mod router;
mod modules;
mod configs;
mod utils;

extern crate diesel;

pub mod schema;
mod migration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load biến môi trường từ .env

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let db_pool = configs::database::establish_connection().await; // GỌI ASYNC!
    // migration::run_migration(&db_pool).await.expect("Migration failed");

    println!("🚀 Server running on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(router::init_routes) // Load routes
            .configure(swagger_routes) // add swagger
    })
        .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
        .run()
        .await
}
