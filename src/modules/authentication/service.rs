use crate::configs::database::DbPool;
use crate::modules::authentication::crud;
use crate::modules::authentication::model::{LoginRequest, NewUser};
use crate::modules::authentication::utils::hash_password;
use actix_web::{HttpResponse, web};
use serde_json::json;

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth",
    responses(
        (status = 200, description = "User Registered Successfully")
    )
)]
pub async fn register_user(pool: web::Data<DbPool>, user: web::Json<NewUser>) -> HttpResponse {
    let new_user = user.into_inner(); // Lấy dữ liệu từ JSON

    let password = hash_password(&new_user.password);

    match crud::insert_user(
        pool.get_ref(),
        &new_user.name,
        &new_user.username,
        &password,
    )
    .await
    {
        Ok(_) => HttpResponse::Created().json("Đăng ký thành công"),
        Err(err) => HttpResponse::InternalServerError()
            .json(json!({ "message": "ERROR", "BUGS": err.to_string().as_str() })),
    }
}

#[utoipa::path(
    get,
    path = "/auth/get_all_users",
    tag = "Auth",
    responses(
        (status = 200, description = "List of Users Retrieved")
    )
)]
pub async fn get_all_users(pool: web::Data<DbPool>) -> HttpResponse {
    match crud::fetch_all_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().json("Lỗi khi lấy danh sách users"),
    }
}
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    responses(
        (status = 200, description = "User login Successfully")
    )
)]
pub async fn login(pool: web::Data<DbPool>, req: web::Json<LoginRequest>) -> HttpResponse {
    let username: String = req.username.clone();
    match crud::get_user_by_email(pool.get_ref(), &username).await {
        
    }
}
