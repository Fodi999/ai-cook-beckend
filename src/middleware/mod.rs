use axum::{
    extract::State,
    http::{Request, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
    body::Body,
};
use async_trait::async_trait;

use crate::{
    services::auth::{AuthService, Claims},
    utils::errors::AppError,
    db::DbPool,
};

pub struct AuthMiddleware;

pub async fn auth_middleware(
    State(pool): State<DbPool>,
    mut request: Request<Body>,
    next: Next<Body>,
) -> Result<Response, AppError> {
    println!("🔐 AUTH MIDDLEWARE: Processing request to {}", request.uri());
    
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let token = match auth_header {
        Some(token) => {
            println!("🔐 AUTH MIDDLEWARE: Token found");
            token
        },
        None => {
            println!("🔐 AUTH MIDDLEWARE: No token found");
            return Err(AppError::Unauthorized("Missing authorization token".to_string()));
        }
    };

    let auth_service = AuthService::new(pool);
    let claims = match auth_service.verify_token(token) {
        Ok(claims) => {
            println!("🔐 AUTH MIDDLEWARE: Token verified for user {}", claims.sub);
            claims
        },
        Err(e) => {
            println!("🔐 AUTH MIDDLEWARE: Token verification failed: {:?}", e);
            return Err(e);
        }
    };
    
    // Add claims to request extensions
    request.extensions_mut().insert(claims);
    
    println!("🔐 AUTH MIDDLEWARE: Proceeding to handler");
    Ok(next.run(request).await)
}

// Extractor for claims
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or_else(|| AppError::Unauthorized("Missing claims".to_string()))
    }
}
