use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub latest_login: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct NewUser {
    pub name: String,
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
