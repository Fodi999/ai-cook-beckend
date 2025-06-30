use serde::{Deserialize, Serialize, Deserializer};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, Datelike};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
    Moderator,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub height: Option<f32>, // in cm
    pub weight: Option<f32>, // in kg
    pub activity_level: Option<String>, // sedentary, lightly_active, moderately_active, very_active, extremely_active
    pub role: UserRole,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub height: Option<f32>,
    pub weight: Option<f32>,
    pub activity_level: Option<String>,
    #[serde(default = "default_user_role")]
    pub role: UserRole,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub height: Option<f32>,
    pub weight: Option<f32>,
    pub activity_level: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateUserSession {
    pub user_id: Uuid,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

impl User {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn age(&self) -> Option<i32> {
        self.date_of_birth.map(|dob| {
            let now = Utc::now().date_naive();
            let dob_date = dob.date_naive();
            let years = now.year() - dob_date.year();
            if now.month() < dob_date.month() || (now.month() == dob_date.month() && now.day() < dob_date.day()) {
                years - 1
            } else {
                years
            }
        })
    }

    pub fn bmi(&self) -> Option<f32> {
        match (self.weight, self.height) {
            (Some(weight), Some(height)) => {
                let height_meters = height / 100.0;
                Some(weight / (height_meters * height_meters))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub height: Option<f32>,
    pub weight: Option<f32>,
    pub activity_level: Option<String>,
    pub avatar_url: Option<String>,
    pub age: Option<i32>,
    pub bmi: Option<f32>,
    pub followers_count: i32,
    pub following_count: i32,
    pub posts_count: i32,
    pub recipes_count: i32,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        let age = user.age();
        let bmi = user.bmi();
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            date_of_birth: user.date_of_birth,
            gender: user.gender,
            height: user.height,
            weight: user.weight,
            activity_level: user.activity_level,
            avatar_url: user.avatar_url,
            age,
            bmi,
            followers_count: 0, // Will be populated by service
            following_count: 0, // Will be populated by service
            posts_count: 0,     // Will be populated by service
            recipes_count: 0,   // Will be populated by service
            created_at: user.created_at,
        }
    }
}

// Helper functions for deserialization
fn default_user_role() -> UserRole {
    UserRole::User
}

fn deserialize_optional_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => {
            // Try different date formats
            if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
                Ok(Some(dt.with_timezone(&Utc)))
            } else if let Ok(dt) = chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
                Ok(Some(dt.and_hms_opt(0, 0, 0).unwrap().and_utc()))
            } else {
                Err(Error::custom(format!("Invalid date format: {}", s)))
            }
        }
        None => Ok(None),
    }
}
