use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub openai_api_key: Option<String>,
    pub cloudinary_url: Option<String>,
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        println!("🔍 Config::new() started");
        
        // Load .env file
        match dotenvy::dotenv() {
            Ok(_) => println!("✅ .env file loaded"),
            Err(e) => println!("⚠️ .env file not found: {}", e)
        }
        
        println!("🔍 Reading environment variables directly...");
        
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| {
                println!("⚠️ DATABASE_URL not found, using default");
                "postgresql://user:password@localhost/itcook".to_string()
            });
        
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| {
                println!("⚠️ JWT_SECRET not found, using default");
                "your-secret-key-here".to_string()
            });

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .unwrap_or_else(|_| {
                println!("⚠️ PORT parse failed, using 3000");
                3000
            });

        println!("✅ Config created successfully");

        Ok(Config {
            database_url,
            jwt_secret,
            openai_api_key: env::var("OPENAI_API_KEY").ok(),
            cloudinary_url: env::var("CLOUDINARY_URL").ok(),
            port,
        })
    }
}
