use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthProfile {
    pub user_id: Uuid,
    pub age: Option<i32>,
    pub medical_conditions: Vec<String>,
    pub allergies: Vec<String>,
    pub medications: Vec<String>,
    pub fitness_level: FitnessLevel,
    pub stress_level: Option<i32>, // 1-10
    pub sleep_goal_hours: Option<f32>,
    pub water_goal_ml: Option<i32>,
    pub preferred_meal_times: Vec<String>,
    pub dietary_restrictions: Vec<String>,
    pub health_goals: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FitnessLevel {
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    SuperActive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyWellbeing {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub mood_score: Option<i32>, // 1-10
    pub energy_level: Option<i32>, // 1-10
    pub stress_level: Option<i32>, // 1-10
    pub sleep_hours: Option<f32>,
    pub sleep_quality: Option<i32>, // 1-10
    pub water_intake_ml: Option<i32>,
    pub exercise_minutes: Option<i32>,
    pub notes: Option<String>,
    pub symptoms: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDailyWellbeing {
    pub mood_score: Option<i32>,
    pub energy_level: Option<i32>,
    pub stress_level: Option<i32>,
    pub sleep_hours: Option<f32>,
    pub sleep_quality: Option<i32>,
    pub water_intake_ml: Option<i32>,
    pub exercise_minutes: Option<i32>,
    pub notes: Option<String>,
    pub symptoms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthInsight {
    pub id: Uuid,
    pub user_id: Uuid,
    pub insight_type: InsightType,
    pub title: String,
    pub message: String,
    pub priority: Priority,
    pub action_items: Vec<String>,
    pub data_sources: Vec<String>, // что использовалось для анализа
    pub created_at: DateTime<Utc>,
    pub is_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    Sleep,
    Hydration,
    Nutrition,
    Exercise,
    Mood,
    Stress,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizedRecommendation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: RecommendationCategory,
    pub title: String,
    pub description: String,
    pub benefits: Vec<String>,
    pub steps: Vec<String>,
    pub frequency: String, // "daily", "weekly", "as needed"
    pub difficulty: i32, // 1-5
    pub estimated_time_minutes: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    Sleep,
    Hydration,
    Nutrition,
    Exercise,
    MindfulnessStress,
    Routine,
}
