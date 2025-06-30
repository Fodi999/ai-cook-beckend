use std::path::Path;
use tokio::fs;
use uuid::Uuid;
use crate::utils::errors::AppError;

#[derive(Debug, Clone)]
pub struct MediaService {
    upload_dir: String,
    max_file_size: usize,
}

impl MediaService {
    pub fn new() -> Self {
        Self {
            upload_dir: "uploads".to_string(),
            max_file_size: 10 * 1024 * 1024, // 10MB default
        }
    }

    pub async fn upload_file(&self, user_id: Uuid, data: Vec<u8>) -> Result<crate::api::community::MediaUploadResponse, AppError> {
        // Validate file size
        if data.len() > self.max_file_size {
            return Err(AppError::BadRequest(format!(
                "File size exceeds maximum limit of {} bytes",
                self.max_file_size
            )));
        }

        // Generate unique filename
        let file_id = Uuid::new_v4();
        let filename = format!("media_{}_{}.jpg", user_id, file_id);

        // Create user-specific directory
        let user_dir = Path::new(&self.upload_dir).join("media").join(user_id.to_string());
        fs::create_dir_all(&user_dir).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to create upload directory: {}", e)))?;

        // Save file
        let file_path = user_dir.join(&filename);
        fs::write(&file_path, &data).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to save file: {}", e)))?;

        // Generate public URLs
        let public_url = format!("/uploads/media/{}/{}", user_id, filename);
        let thumbnail_url = Some(format!("/uploads/media/{}/thumb_{}", user_id, filename));

        Ok(crate::api::community::MediaUploadResponse {
            url: public_url,
            thumbnail_url,
            media_type: "image/jpeg".to_string(),
            file_size: data.len() as i64,
        })
    }

    pub async fn upload_image(&self, file_data: Vec<u8>, filename: &str, user_id: Uuid) -> Result<String, AppError> {
        // Validate file size
        if file_data.len() > self.max_file_size {
            return Err(AppError::BadRequest(format!(
                "File size exceeds maximum limit of {} bytes",
                self.max_file_size
            )));
        }

        // Validate file extension
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| AppError::BadRequest("Invalid file format".to_string()))?;

        let allowed_extensions = ["jpg", "jpeg", "png", "gif", "webp"];
        if !allowed_extensions.contains(&extension.to_lowercase().as_str()) {
            return Err(AppError::BadRequest("File format not supported. Allowed formats: jpg, jpeg, png, gif, webp".to_string()));
        }

        // Generate unique filename
        let file_id = Uuid::new_v4();
        let new_filename = format!("{}_{}.{}", user_id, file_id, extension);

        // Create user-specific directory
        let user_dir = Path::new(&self.upload_dir).join("images").join(user_id.to_string());
        fs::create_dir_all(&user_dir).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to create upload directory: {}", e)))?;

        // Save file
        let file_path = user_dir.join(&new_filename);
        fs::write(&file_path, file_data).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to save file: {}", e)))?;

        // Return public URL
        let public_url = format!("/uploads/images/{}/{}", user_id, new_filename);
        Ok(public_url)
    }

    pub async fn delete_file(&self, file_url: &str) -> Result<(), AppError> {
        // Extract file path from URL
        let file_path = if file_url.starts_with("/uploads/") {
            Path::new(&self.upload_dir).join(&file_url[9..]) // Remove "/uploads/" prefix
        } else {
            return Err(AppError::BadRequest("Invalid file URL".to_string()));
        };

        if file_path.exists() {
            fs::remove_file(file_path).await
                .map_err(|e| AppError::InternalServerError(format!("Failed to delete file: {}", e)))?;
        }

        Ok(())
    }
}
