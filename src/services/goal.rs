use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use crate::{
    models::goal::{Goal, CreateGoal, GoalType, GoalStatus, WeightEntry, Achievement},
    utils::errors::AppError,
};

pub struct GoalService {
    pool: crate::db::DbPool,
}

impl GoalService {
    pub fn new(pool: crate::db::DbPool) -> Self {
        Self { pool }
    }

    pub async fn create_goal(&self, goal: CreateGoal) -> Result<Goal, AppError> {
        // Mock implementation - in production, this would save to database
        let goal_id = Uuid::new_v4();
        
        Ok(Goal {
            id: goal_id,
            user_id: goal.user_id,
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
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn get_user_goals(
        &self,
        user_id: Uuid,
        goal_type: Option<GoalType>,
        status: Option<GoalStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Goal>, AppError> {
        // Mock implementation
        self.get_mock_goals(user_id, goal_type, status, limit, offset).await
    }

    pub async fn get_goal_by_id(&self, id: Uuid, user_id: Uuid) -> Result<Goal, AppError> {
        // Mock implementation
        self.get_mock_goal(id, user_id).await
    }

    pub async fn update_goal(
        &self,
        id: Uuid,
        user_id: Uuid,
        payload: crate::api::goals::CreateGoalRequest,
    ) -> Result<Goal, AppError> {
        // Mock implementation - in production, verify ownership and update database
        Ok(Goal {
            id,
            user_id,
            title: payload.title,
            description: payload.description,
            goal_type: payload.goal_type,
            target_value: payload.target_value,
            current_value: payload.current_value.unwrap_or(0.0),
            unit: payload.unit,
            target_date: payload.target_date,
            daily_target: payload.daily_target,
            weekly_target: payload.weekly_target,
            status: GoalStatus::Active, // Default status
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn delete_goal(&self, _id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        // Mock implementation - in production, verify ownership and delete from database
        Ok(())
    }

    pub async fn update_progress(
        &self,
        id: Uuid,
        user_id: Uuid,
        value: f32,
        _notes: Option<String>,
    ) -> Result<Goal, AppError> {
        // Mock implementation - in production, update current_value and check if goal is completed
        let mut goal = self.get_mock_goal(id, user_id).await?;
        goal.current_value = value;
        
        // Check if goal is completed
        if value >= goal.target_value {
            goal.status = GoalStatus::Completed;
        }
        
        goal.updated_at = Utc::now();
        Ok(goal)
    }

    pub async fn add_weight_entry(
        &self,
        user_id: Uuid,
        weight: f32,
        date: NaiveDate,
        notes: Option<String>,
    ) -> Result<WeightEntry, AppError> {
        // Validate weight
        if weight <= 0.0 || weight > 1000.0 {
            return Err(AppError::BadRequest("Invalid weight value".to_string()));
        }

        // Mock implementation
        Ok(WeightEntry {
            id: Uuid::new_v4(),
            user_id,
            weight,
            date,
            notes,
            created_at: Utc::now(),
        })
    }

    pub async fn get_weight_history(
        &self,
        user_id: Uuid,
        _start_date: Option<NaiveDate>,
        _end_date: Option<NaiveDate>,
        limit: i64,
    ) -> Result<Vec<WeightEntry>, AppError> {
        // Mock implementation
        self.get_mock_weight_entries(user_id, limit).await
    }

    pub async fn get_user_achievements(&self, user_id: Uuid) -> Result<Vec<Achievement>, AppError> {
        // Mock implementation
        self.get_mock_achievements(user_id).await
    }

    // Mock implementations for testing without database
    async fn get_mock_goal(&self, id: Uuid, user_id: Uuid) -> Result<Goal, AppError> {
        Ok(Goal {
            id,
            user_id,
            title: "Lose 5kg in 3 months".to_string(),
            description: Some("Target weight loss for summer".to_string()),
            goal_type: GoalType::WeightLoss,
            target_value: 5.0,
            current_value: 2.5,
            unit: "kg".to_string(),
            target_date: Some(NaiveDate::from_ymd_opt(2024, 8, 1).unwrap()),
            daily_target: Some(0.05),
            weekly_target: Some(0.35),
            status: GoalStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    async fn get_mock_goals(
        &self,
        user_id: Uuid,
        goal_type: Option<GoalType>,
        status: Option<GoalStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Goal>, AppError> {
        let mut goals = vec![];
        
        // Generate different mock goals
        for i in 0..std::cmp::min(limit, 5) {
            let goal_id = Uuid::new_v4();
            let mock_goal_type = match i % 4 {
                0 => GoalType::WeightLoss,
                1 => GoalType::WeightGain,
                2 => GoalType::CalorieIntake,
                _ => GoalType::Exercise,
            };
            
            let mock_status = match i % 3 {
                0 => GoalStatus::Active,
                1 => GoalStatus::Completed,
                _ => GoalStatus::Paused,
            };

            // Filter by goal_type if specified
            if let Some(filter_type) = &goal_type {
                if mock_goal_type != *filter_type {
                    continue;
                }
            }

            // Filter by status if specified
            if let Some(filter_status) = &status {
                if mock_status != *filter_status {
                    continue;
                }
            }
            
            let goal = Goal {
                id: goal_id,
                user_id,
                title: format!("Goal {} - {}", i + 1, match &mock_goal_type {
                    GoalType::WeightLoss => "Lose weight",
                    GoalType::WeightGain => "Gain weight",
                    GoalType::CalorieIntake => "Daily calories",
                    GoalType::Exercise => "Exercise time",
                    _ => "Other goal",
                }),
                description: Some(format!("Description for goal {}", i + 1)),
                goal_type: mock_goal_type.clone(),
                target_value: match &mock_goal_type {
                    GoalType::WeightLoss | GoalType::WeightGain => 5.0 + (i as f32),
                    GoalType::CalorieIntake => 2000.0 + (i as f32 * 200.0),
                    GoalType::Exercise => 30.0 + (i as f32 * 15.0),
                    _ => 100.0 + (i as f32 * 50.0),
                },
                current_value: match mock_status {
                    GoalStatus::Completed => 5.0 + (i as f32),
                    _ => (2.5 + (i as f32)) / 2.0,
                },
                unit: match &mock_goal_type {
                    GoalType::WeightLoss | GoalType::WeightGain => "kg".to_string(),
                    GoalType::CalorieIntake => "kcal".to_string(),
                    GoalType::Exercise => "minutes".to_string(),
                    _ => "units".to_string(),
                },
                target_date: Some(NaiveDate::from_ymd_opt(2024, 8 + i as u32, 1).unwrap()),
                daily_target: Some(0.1 + (i as f32 * 0.05)),
                weekly_target: Some(0.7 + (i as f32 * 0.3)),
                status: mock_status,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            goals.push(goal);
        }
        
        let start = offset as usize;
        let end = std::cmp::min(start + limit as usize, goals.len());
        
        if start >= goals.len() {
            Ok(vec![])
        } else {
            Ok(goals[start..end].to_vec())
        }
    }

    async fn get_mock_weight_entries(&self, user_id: Uuid, limit: i64) -> Result<Vec<WeightEntry>, AppError> {
        let mut entries = vec![];
        
        for i in 0..std::cmp::min(limit, 10) {
            let entry = WeightEntry {
                id: Uuid::new_v4(),
                user_id,
                weight: 70.0 - (i as f32 * 0.5), // Simulating weight loss
                date: NaiveDate::from_ymd_opt(2024, 6, 1 + i as u32).unwrap(),
                notes: if i % 3 == 0 { Some("Good progress".to_string()) } else { None },
                created_at: Utc::now(),
            };
            entries.push(entry);
        }
        
        Ok(entries)
    }

    async fn get_mock_achievements(&self, user_id: Uuid) -> Result<Vec<Achievement>, AppError> {
        let achievements = vec![
            Achievement {
                id: Uuid::new_v4(),
                user_id,
                title: "First Goal".to_string(),
                description: "Created your first goal".to_string(),
                icon: "🎯".to_string(),
                earned_at: Utc::now(),
                goal_related: None,
            },
            Achievement {
                id: Uuid::new_v4(),
                user_id,
                title: "Consistency King".to_string(),
                description: "Logged data for 7 days straight".to_string(),
                icon: "⭐".to_string(),
                earned_at: Utc::now(),
                goal_related: None,
            },
            Achievement {
                id: Uuid::new_v4(),
                user_id,
                title: "Goal Crusher".to_string(),
                description: "Completed your first goal".to_string(),
                icon: "🏆".to_string(),
                earned_at: Utc::now(),
                goal_related: Some(Uuid::new_v4()),
            },
        ];
        
        Ok(achievements)
    }
}
