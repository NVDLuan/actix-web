use crate::configs::swagger::swagger_routes;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::fmt;
use crate::modules::authentication::middleware::AuthMiddleware;

mod router;
mod modules;
mod configs;
mod utils;

mod migration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load biến môi trường từ .env
    let file_appender = rolling::daily("logs", "app.log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    fmt()
        .with_writer(file_writer.and(std::io::stdout)) // Ghi log ra cả file và console
        .with_env_filter("info") // Chỉ log từ mức info trở lên
        .init();

    info!("Ứng dụng đã khởi động!");
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let db_pool = configs::database::establish_connection().await; // GỌI ASYNC!
    // migration::run_migration(&db_pool).await.expect("Migration failed");

    info!("🚀 Server running on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(AuthMiddleware { db: db_pool.clone().into() })
            .configure(router::init_routes) // Load routes
            .configure(swagger_routes) // add swagger
    })
        .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
        .run()
        .await
}
