use sea_orm::{Database, DatabaseConnection};
use std::env;

pub type DbPool = DatabaseConnection;

pub async fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL không được tìm thấy");
    Database::connect(&database_url)
        .await
        .expect("Không thể kết nối database")
}
