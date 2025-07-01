use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::Json as ResponseJson,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    db::DbPool,
    models::user::{User, CreateUser, UserRole},
    services::auth::{AuthService, Claims},
    utils::errors::AppError,
};

pub fn routes() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
}

pub fn protected_routes() -> Router {
    Router::new()
        .route("/me", get(get_current_user))
        .route("/logout", post(logout))
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 100))]
    pub password: String,
    #[validate(length(min = 2, max = 50))]
    pub first_name: String,
    #[validate(length(min = 2, max = 50))]
    pub last_name: String,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub height: Option<f32>,
    pub weight: Option<f32>,
    pub activity_level: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
            created_at: user.created_at,
        }
    }
}

pub async fn register(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ResponseJson<AuthResponse>, AppError> {
    payload.validate()?;

    let create_user = CreateUser {
        email: payload.email,
        password: payload.password,
        first_name: payload.first_name,
        last_name: payload.last_name,
        date_of_birth: payload.date_of_birth,
        gender: payload.gender,
        height: payload.height,
        weight: payload.weight,
        activity_level: payload.activity_level,
        role: UserRole::User,
    };

    let auth_service = AuthService::new(pool);
    let (user, tokens) = auth_service.register(create_user).await?;

    Ok(ResponseJson(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user: user.into(),
    }))
}

pub async fn login(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<ResponseJson<AuthResponse>, AppError> {
    payload.validate()?;

    let auth_service = AuthService::new(pool);
    let (user, tokens) = auth_service.login(&payload.email, &payload.password).await?;

    Ok(ResponseJson(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user: user.into(),
    }))
}

pub async fn refresh_token(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let refresh_token = payload["refresh_token"]
        .as_str()
        .ok_or(AppError::BadRequest("Missing refresh token".to_string()))?;

    let auth_service = AuthService::new(pool);
    let tokens = auth_service.refresh_token(refresh_token).await?;

    Ok(ResponseJson(serde_json::json!({
        "access_token": tokens.access_token,
        "refresh_token": tokens.refresh_token
    })))
}

pub async fn get_current_user(
    Extension(_pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<UserResponse>, AppError> {
    // Claims содержат информацию о пользователе из JWT
    Ok(ResponseJson(UserResponse {
        id: claims.sub,
        email: claims.email,
        first_name: claims.first_name,
        last_name: claims.last_name,
        role: claims.role,
        created_at: chrono::DateTime::from_timestamp(claims.iat as i64, 0).unwrap_or_else(|| Utc::now()),
    }))
}

pub async fn logout(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<StatusCode, AppError> {
    let auth_service = AuthService::new(pool);
    auth_service.logout(claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}
