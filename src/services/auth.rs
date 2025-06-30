use uuid::Uuid;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::{
    db::DbPool,
    models::user::{User, CreateUser, UserSession, CreateUserSession, UserRole},
    utils::errors::AppError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Clone)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct AuthService {
    pool: DbPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: DbPool) -> Self {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-here".to_string());
        
        Self { pool, jwt_secret }
    }

    pub async fn register(&self, create_user: CreateUser) -> Result<(User, AuthTokens), AppError> {
        // Check if user already exists
        let existing_user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&create_user.email)
        .fetch_optional(&self.pool)
        .await?;

        if existing_user.is_some() {
            return Err(AppError::BadRequest("Email already registered".to_string()));
        }

        // Hash password
        let password_hash = hash(&create_user.password, DEFAULT_COST)
            .map_err(|e| AppError::InternalServerError(format!("Password hashing failed: {}", e)))?;

        // Create user
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, first_name, last_name, 
                              date_of_birth, gender, height, weight, activity_level, role)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&create_user.email)
        .bind(&password_hash)
        .bind(&create_user.first_name)
        .bind(&create_user.last_name)
        .bind(create_user.date_of_birth)
        .bind(create_user.gender)
        .bind(create_user.height)
        .bind(create_user.weight)
        .bind(create_user.activity_level)
        .bind(create_user.role)
        .fetch_one(&self.pool)
        .await?;

        // Generate tokens
        let tokens = self.generate_tokens(&user).await?;

        Ok((user, tokens))
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(User, AuthTokens), AppError> {
        // Find user by email
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        // Verify password
        let password_valid = verify(password, &user.password_hash)
            .map_err(|e| AppError::InternalServerError(format!("Password verification failed: {}", e)))?;

        if !password_valid {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // Update last login
        sqlx::query("UPDATE users SET last_login_at = NOW() WHERE id = $1")
            .bind(user.id)
            .execute(&self.pool)
            .await?;

        // Generate tokens
        let tokens = self.generate_tokens(&user).await?;

        Ok((user, tokens))
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthTokens, AppError> {
        // Find session by refresh token
        let session = sqlx::query_as::<_, UserSession>(
            "SELECT * FROM user_sessions WHERE refresh_token = $1 AND expires_at > NOW()"
        )
        .bind(refresh_token)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        // Get user
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(session.user_id)
        .fetch_one(&self.pool)
        .await?;

        // Generate new tokens
        let tokens = self.generate_tokens(&user).await?;

        // Delete old session
        sqlx::query("DELETE FROM user_sessions WHERE id = $1")
            .bind(session.id)
            .execute(&self.pool)
            .await?;

        Ok(tokens)
    }

    pub async fn logout(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM user_sessions WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn generate_tokens(&self, user: &User) -> Result<AuthTokens, AppError> {
        let now = Utc::now();
        let access_exp = now + Duration::hours(1);
        let refresh_exp = now + Duration::days(30);

        // Create access token claims
        let access_claims = Claims {
            sub: user.id,
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            role: user.role.clone(),
            exp: access_exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        // Generate access token
        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| AppError::InternalServerError(format!("Token generation failed: {}", e)))?;

        // Generate refresh token
        let refresh_token = Uuid::new_v4().to_string();

        // Store refresh token in database
        let session = CreateUserSession {
            user_id: user.id,
            refresh_token: refresh_token.clone(),
            expires_at: refresh_exp,
        };

        sqlx::query(
            r#"
            INSERT INTO user_sessions (id, user_id, refresh_token, expires_at)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(session.user_id)
        .bind(&session.refresh_token)
        .bind(session.expires_at)
        .execute(&self.pool)
        .await?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
        })
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }
}
