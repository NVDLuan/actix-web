use sea_orm::{DatabaseConnection, Schema, DbErr};
use sea_orm::prelude::*;

pub async fn run_migration(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema = Schema::new(db.get_database_backend());

    let stmt = schema.create_table_from_entity(super::modules::authentication::model::Entity);
    db.execute(db.get_database_backend().build(&stmt)).await?;

    Ok(())
}