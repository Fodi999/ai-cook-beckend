// Placeholder models
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "goal_type", rename_all = "lowercase")]
pub enum GoalType {
    WeightLoss,
    WeightGain,
    MaintainWeight,
    CalorieIntake,
    ProteinIntake,
    Exercise,
    Water,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "goal_status", rename_all = "lowercase")]
pub enum GoalStatus {
    Active,
    Completed,
    Paused,
    Cancelled,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub user_id: Uuid,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGoal {
    pub user_id: Uuid,
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
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WeightEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub weight: f32,
    pub date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Achievement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub earned_at: DateTime<Utc>,
    pub goal_related: Option<Uuid>,
}
