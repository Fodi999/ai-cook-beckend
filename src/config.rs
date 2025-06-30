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
        dotenvy::dotenv().ok();
        
        let config = config::Config::builder()
            .set_default("port", 3000)?
            .add_source(config::Environment::with_prefix("ITCOOK"))
            .build()?;

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://user:password@localhost/itcook".to_string());
        
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-here".to_string());

        Ok(Config {
            database_url,
            jwt_secret,
            openai_api_key: env::var("OPENAI_API_KEY").ok(),
            cloudinary_url: env::var("CLOUDINARY_URL").ok(),
            port: config.get("port").unwrap_or(3000),
        })
    }
}
