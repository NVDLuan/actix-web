use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::task::{Context, Poll};
use std::rc::Rc;
use crate::configs::database::DbPool;
use crate::modules::authentication::crud;
use crate::modules::authentication::model::Model as UserModel;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

const SECRET_KEY: &[u8] = b"your_secret_key";

#[derive(Debug, serde::Deserialize)]
struct Claims {
    sub: Uuid, // User ID
    exp: usize, // Thời gian hết hạn
}

/// Middleware kiểm tra Authorization Token
pub struct AuthMiddleware {
    pub db: Rc<DbPool>,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    // ✅ Sửa lỗi: Trả về Ready<Result<_, _>>
    fn new_transform(&self, service: S) -> Ready<Result<Self::Transform, Self::InitError>> {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
            db: self.db.clone(),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    db: Rc<DbPool>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let db = self.db.clone();
        let service = self.service.clone();

        Box::pin(async move {
            match get_user_from_request(&req, &db).await {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                    service.call(req).await
                }
                Err(err) => Ok(ServiceResponse::new(
                    req.into_parts().0,
                    HttpResponse::Unauthorized().json(err).map_into_right_body::<B>(), // ✅ Chuyển đổi body đúng kiểu
                )),

            }
        })
    }
}

/// Xác thực User từ Token (Cookie hoặc Header)
async fn get_user_from_request(req: &ServiceRequest, db: &DbPool) -> Result<UserModel, &'static str> {
    // Kiểm tra Access Token trong Cookie
    if let Some(access_cookie) = req.cookie("access_token") {
        return get_user_from_token(access_cookie.value(), db).await;
    }

    // Kiểm tra Bearer Token trong Authorization Header
    if let Some(auth) = req.headers().get("Authorization") {
        let token = auth.to_str().map_err(|_| "Invalid header format")?;
        if token.starts_with("Bearer ") {
            return get_user_from_token(&token[7..], db).await;
        }
    }

    Err("Missing token")
}

/// Giải mã JWT Token để lấy User
async fn get_user_from_token(token: &str, db: &DbPool) -> Result<UserModel, &'static str> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ).map_err(|_| "Invalid token")?;

    let user_id = token_data.claims.sub;
    crud::get_user_by_id(db, user_id).await.map_err(|_| "User not found")
}
