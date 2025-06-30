use uuid::Uuid;
use chrono::{Utc, Datelike};
use crate::{
    models::user::UserProfile,
    api::goals::HealthStatsResponse,
    utils::errors::AppError,
};

pub struct HealthService {
    pool: crate::db::DbPool,
}

impl HealthService {
    pub fn new(pool: crate::db::DbPool) -> Self {
        Self { pool }
    }

    pub async fn get_user_profile(&self, user_id: Uuid) -> Result<UserProfile, AppError> {
        // Mock implementation - in production, this would fetch from database
        self.get_mock_user_profile(user_id).await
    }

    pub async fn calculate_bmr(&self, user_id: Uuid) -> Result<f32, AppError> {
        // Get user profile for BMR calculation
        let profile = self.get_user_profile(user_id).await?;
        
        // Calculate BMR using Mifflin-St Jeor Equation
        let bmr = if let (Some(weight), Some(height), Some(age), Some(gender)) = 
            (profile.weight, profile.height, profile.age, profile.gender.as_ref()) {
            
            let base = 10.0 * weight + 6.25 * (height * 100.0) - 5.0 * age as f32;
            
            match gender.to_lowercase().as_str() {
                "male" | "m" => base + 5.0,
                "female" | "f" => base - 161.0,
                _ => base, // Default calculation
            }
        } else {
            // Default BMR if missing data
            if let Some(weight) = profile.weight {
                weight * 22.0 // Rough estimate: 22 calories per kg
            } else {
                1800.0 // Default value
            }
        };

        Ok(bmr)
    }

    pub async fn calculate_tdee(&self, user_id: Uuid) -> Result<f32, AppError> {
        let bmr = self.calculate_bmr(user_id).await?;
        let profile = self.get_user_profile(user_id).await?;
        
        // Activity multipliers
        let activity_multiplier = match profile.activity_level.as_deref() {
            Some("sedentary") => 1.2,
            Some("lightly_active") => 1.375,
            Some("moderately_active") => 1.55,
            Some("very_active") => 1.725,
            Some("extremely_active") => 1.9,
            _ => 1.375, // Default to lightly active
        };

        Ok(bmr * activity_multiplier)
    }

    pub async fn get_comprehensive_stats(&self, user_id: Uuid) -> Result<HealthStatsResponse, AppError> {
        let profile = self.get_user_profile(user_id).await?;
        let bmr = self.calculate_bmr(user_id).await?;
        let tdee = self.calculate_tdee(user_id).await?;
        
        // BMI is already calculated in profile
        let bmi_category = profile.bmi.map(|bmi| self.get_bmi_category(bmi));

        // Mock weight changes - in production, calculate from weight history
        let weight_change_7days = Some(-0.3); // Mock: lost 0.3kg in 7 days
        let weight_change_30days = Some(-1.2); // Mock: lost 1.2kg in 30 days

        // Calculate daily calories goal based on goals
        let daily_calories_goal = Some(tdee - 300.0); // Mock: 300 calorie deficit for weight loss

        Ok(HealthStatsResponse {
            bmr,
            tdee,
            current_weight: profile.weight,
            target_weight: Some(profile.weight.unwrap_or(70.0) - 5.0), // Mock target
            weight_change_7days,
            weight_change_30days,
            bmi: profile.bmi,
            bmi_category,
            daily_calories_goal,
        })
    }

    // Helper methods
    fn get_bmi_category(&self, bmi: f32) -> String {
        match bmi {
            bmi if bmi < 18.5 => "Underweight".to_string(),
            bmi if bmi < 25.0 => "Normal weight".to_string(),
            bmi if bmi < 30.0 => "Overweight".to_string(),
            _ => "Obese".to_string(),
        }
    }

    // Mock implementations for testing without database
    async fn get_mock_user_profile(&self, user_id: Uuid) -> Result<UserProfile, AppError> {
        // Calculate age for a mock birth date
        let birth_date = chrono::DateTime::parse_from_rfc3339("1990-05-15T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        
        let now = Utc::now().date_naive();
        let dob_date = birth_date.date_naive();
        let years = now.year() - dob_date.year();
        let age = if now.month() < dob_date.month() || (now.month() == dob_date.month() && now.day() < dob_date.day()) {
            years - 1
        } else {
            years
        };

        // Calculate BMI
        let weight = 75.0;
        let height = 175.0; // cm
        let height_meters = height / 100.0;
        let bmi = weight / (height_meters * height_meters);

        Ok(UserProfile {
            id: user_id,
            email: "user@example.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            date_of_birth: Some(birth_date),
            gender: Some("male".to_string()),
            height: Some(height),
            weight: Some(weight),
            activity_level: Some("moderately_active".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            age: Some(age),
            bmi: Some(bmi),
            followers_count: 125,
            following_count: 89,
            posts_count: 23,
            recipes_count: 15,
            created_at: Utc::now(),
        })
    }
}
