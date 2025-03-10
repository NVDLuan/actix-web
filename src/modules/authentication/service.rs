use crate::configs::database::DbPool;
use crate::modules::authentication::auth::{generate_access_token, generate_refresh_token};
use crate::modules::authentication::crud;
use crate::modules::authentication::model::{LoginRequest, NewUser};
use crate::modules::authentication::utils::{hash_password, verify_password};
use actix_web::cookie::Cookie;
use actix_web::{web, HttpResponse};
use serde_json::json;
use tracing::{info, debug};
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
        &new_user.email,
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
    path = "/auth/users",
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
    info!("Service: call login");
    let email: String = req.email.clone();
    match crud::get_user_by_email(pool.get_ref(), &email).await {
        Ok(user) => {
            if verify_password(&req.password, &user.password) {
                info!("set cookie");
                let access_token = generate_access_token(&email);
                let refresh_token = generate_refresh_token();

                let access_cookie = Cookie::build("access_token", access_token.clone())
                    .path("/")
                    .http_only(true)
                    .secure(true)
                    .finish();

                let refresh_cookie = Cookie::build("refresh_token", refresh_token.clone())
                    .path("/")
                    .http_only(true)
                    .secure(true)
                    .finish();

                HttpResponse::Ok()
                    .cookie(access_cookie)
                    .cookie(refresh_cookie)
                    .json("Login successful")
            } else {
                HttpResponse::Unauthorized().json("Invalid credentials")
            }
        }
        Err(_) => {
            HttpResponse::Unauthorized().json("Invalid credentials")
        }
    }
}
