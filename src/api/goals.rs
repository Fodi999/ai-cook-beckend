use axum::{
    extract::{Extension, Json, Path, Query},
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

use crate::{
    db::DbPool,
    models::goal::{Goal, CreateGoal, GoalType, GoalStatus, WeightEntry, Achievement},
    services::{auth::Claims, goal::GoalService, health::HealthService},
    utils::errors::AppError,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_goal))
        .route("/", get(get_goals))
        .route("/{id}", get(get_goal))
        .route("/{id}", put(update_goal))
        .route("/{id}", delete(delete_goal))
        .route("/{id}/progress", post(update_progress))
        .route("/weight", post(add_weight_entry))
        .route("/weight", get(get_weight_history))
        .route("/bmr", get(calculate_bmr))
        .route("/tdee", get(calculate_tdee))
        .route("/achievements", get(get_achievements))
        .route("/stats", get(get_health_stats))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGoalRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    pub description: Option<String>,
    pub goal_type: GoalType,
    pub target_value: f32,
    pub current_value: Option<f32>,
    pub unit: String,
    pub target_date: Option<NaiveDate>,
    pub daily_target: Option<f32>,
    pub weekly_target: Option<f32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProgressRequest {
    pub value: f32,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct WeightEntryRequest {
    pub weight: f32,
    pub date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoalQueryParams {
    pub goal_type: Option<GoalType>,
    pub status: Option<GoalStatus>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct WeightQueryParams {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct GoalResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub goal_type: GoalType,
    pub target_value: f32,
    pub current_value: f32,
    pub unit: String,
    pub target_date: Option<NaiveDate>,
    pub daily_target: Option<f32>,
    pub weekly_target: Option<f32>,
    pub status: GoalStatus,
    pub progress_percentage: f32,
    pub days_remaining: Option<i32>,
    pub is_on_track: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Goal> for GoalResponse {
    fn from(goal: Goal) -> Self {
        let progress_percentage = if goal.target_value > 0.0 {
            (goal.current_value / goal.target_value * 100.0).min(100.0)
        } else {
            0.0
        };

        let days_remaining = goal.target_date.map(|target| {
            let today = chrono::Utc::now().date_naive();
            (target - today).num_days() as i32
        });

        let is_on_track = match (goal.target_date, days_remaining) {
            (Some(_), Some(days)) if days > 0 => {
                let expected_progress = 100.0 - (days as f32 / 30.0 * 100.0); // Simplified calculation
                progress_percentage >= expected_progress
            }
            _ => progress_percentage >= 50.0, // Default threshold
        };

        Self {
            id: goal.id,
            title: goal.title,
            description: goal.description,
            goal_type: goal.goal_type,
            target_value: goal.target_value,
            current_value: goal.current_value,
            unit: goal.unit,
            target_date: goal.target_date,
            daily_target: goal.daily_target,
            weekly_target: goal.weekly_target,
            status: goal.status,
            progress_percentage,
            days_remaining,
            is_on_track,
            created_at: goal.created_at,
            updated_at: goal.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WeightEntryResponse {
    pub id: Uuid,
    pub weight: f32,
    pub date: NaiveDate,
    pub notes: Option<String>,
    pub bmi: Option<f32>,
    pub weight_change: Option<f32>, // vs previous entry
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct HealthStatsResponse {
    pub bmr: f32,
    pub tdee: f32,
    pub current_weight: Option<f32>,
    pub target_weight: Option<f32>,
    pub weight_change_7days: Option<f32>,
    pub weight_change_30days: Option<f32>,
    pub bmi: Option<f32>,
    pub bmi_category: Option<String>,
    pub daily_calories_goal: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct AchievementResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub earned_at: DateTime<Utc>,
    pub goal_related: Option<Uuid>,
}

pub async fn create_goal(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Json(payload): Json<CreateGoalRequest>,
) -> Result<ResponseJson<GoalResponse>, AppError> {
    payload.validate()?;

    let create_goal = CreateGoal {
        user_id: claims.sub,
        title: payload.title,
        description: payload.description,
        goal_type: payload.goal_type,
        target_value: payload.target_value,
        current_value: payload.current_value.unwrap_or(0.0),
        unit: payload.unit,
        target_date: payload.target_date,
        daily_target: payload.daily_target,
        weekly_target: payload.weekly_target,
        status: GoalStatus::Active,
    };

    let goal_service = GoalService::new(pool);
    let goal = goal_service.create_goal(create_goal).await?;

    Ok(ResponseJson(goal.into()))
}

pub async fn get_goals(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Query(params): Query<GoalQueryParams>,
) -> Result<ResponseJson<Vec<GoalResponse>>, AppError> {
    let goal_service = GoalService::new(pool);
    let goals = goal_service.get_user_goals(
        claims.sub,
        params.goal_type,
        params.status,
        params.limit.unwrap_or(50),
        params.offset.unwrap_or(0),
    ).await?;

    let response: Vec<GoalResponse> = goals.into_iter().map(Into::into).collect();
    Ok(ResponseJson(response))
}

pub async fn get_goal(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<GoalResponse>, AppError> {
    let goal_service = GoalService::new(pool);
    let goal = goal_service.get_goal_by_id(id, claims.sub).await?;

    Ok(ResponseJson(goal.into()))
}

pub async fn update_goal(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateGoalRequest>,
) -> Result<ResponseJson<GoalResponse>, AppError> {
    payload.validate()?;

    let goal_service = GoalService::new(pool);
    let goal = goal_service.update_goal(id, claims.sub, payload).await?;

    Ok(ResponseJson(goal.into()))
}

pub async fn delete_goal(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let goal_service = GoalService::new(pool);
    goal_service.delete_goal(id, claims.sub).await?;

    Ok(ResponseJson(serde_json::json!({"message": "Goal deleted successfully"})))
}

pub async fn update_progress(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProgressRequest>,
) -> Result<ResponseJson<GoalResponse>, AppError> {
    let goal_service = GoalService::new(pool);
    let goal = goal_service.update_progress(id, claims.sub, payload.value, payload.notes).await?;

    Ok(ResponseJson(goal.into()))
}

pub async fn add_weight_entry(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Json(payload): Json<WeightEntryRequest>,
) -> Result<ResponseJson<WeightEntryResponse>, AppError> {
    let goal_service = GoalService::new(pool.clone());
    let health_service = HealthService::new(pool);
    
    let entry = goal_service.add_weight_entry(
        claims.sub,
        payload.weight,
        payload.date.unwrap_or_else(|| chrono::Utc::now().date_naive()),
        payload.notes,
    ).await?;

    // Calculate BMI if user has height
    let user_profile = health_service.get_user_profile(claims.sub).await.ok();
    let bmi = user_profile.and_then(|profile| {
        profile.height.map(|height| payload.weight / (height / 100.0).powi(2))
    });

    let response = WeightEntryResponse {
        id: entry.id,
        weight: entry.weight,
        date: entry.date,
        notes: entry.notes,
        bmi,
        weight_change: None, // Will be calculated by service
        created_at: entry.created_at,
    };

    Ok(ResponseJson(response))
}

pub async fn get_weight_history(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Query(params): Query<WeightQueryParams>,
) -> Result<ResponseJson<Vec<WeightEntryResponse>>, AppError> {
    let goal_service = GoalService::new(pool);
    let entries = goal_service.get_weight_history(
        claims.sub,
        params.start_date,
        params.end_date,
        params.limit.unwrap_or(100),
    ).await?;

    let response: Vec<WeightEntryResponse> = entries.into_iter().map(|entry| {
        WeightEntryResponse {
            id: entry.id,
            weight: entry.weight,
            date: entry.date,
            notes: entry.notes,
            bmi: None, // Calculate in service if needed
            weight_change: None, // Calculate in service if needed
            created_at: entry.created_at,
        }
    }).collect();

    Ok(ResponseJson(response))
}

pub async fn calculate_bmr(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let health_service = HealthService::new(pool);
    let bmr = health_service.calculate_bmr(claims.sub).await?;

    Ok(ResponseJson(serde_json::json!({
        "bmr": bmr,
        "description": "Basal Metabolic Rate - calories burned at rest"
    })))
}

pub async fn calculate_tdee(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let health_service = HealthService::new(pool);
    let tdee = health_service.calculate_tdee(claims.sub).await?;

    Ok(ResponseJson(serde_json::json!({
        "tdee": tdee,
        "description": "Total Daily Energy Expenditure - calories needed per day"
    })))
}

pub async fn get_achievements(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<Vec<AchievementResponse>>, AppError> {
    let goal_service = GoalService::new(pool);
    let achievements = goal_service.get_user_achievements(claims.sub).await?;

    let response: Vec<AchievementResponse> = achievements.into_iter().map(|achievement| {
        AchievementResponse {
            id: achievement.id,
            title: achievement.title,
            description: achievement.description,
            icon: achievement.icon,
            earned_at: achievement.earned_at,
            goal_related: achievement.goal_related,
        }
    }).collect();

    Ok(ResponseJson(response))
}

pub async fn get_health_stats(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<HealthStatsResponse>, AppError> {
    let health_service = HealthService::new(pool);
    let stats = health_service.get_comprehensive_stats(claims.sub).await?;

    Ok(ResponseJson(stats))
}
