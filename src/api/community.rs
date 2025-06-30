use axum::{
    extract::{Extension, Json, Path, Query},
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    db::DbPool,
    models::community::{Post, CreatePost, PostType, Comment, CreateComment, Like, Follow},
    services::{auth::Claims, community::CommunityService, media::MediaService},
    utils::errors::AppError,
};

pub fn routes() -> Router {
    Router::new()
        .route("/posts", post(create_post))
        .route("/posts", get(get_feed))
        .route("/posts/{id}", get(get_post))
        .route("/posts/{id}", put(update_post))
        .route("/posts/{id}", delete(delete_post))
        .route("/posts/{id}/like", post(toggle_like))
        .route("/posts/{id}/comments", post(create_comment))
        .route("/posts/{id}/comments", get(get_comments))
        .route("/comments/{id}", put(update_comment))
        .route("/comments/{id}", delete(delete_comment))
        .route("/users/{id}/follow", post(toggle_follow))
        .route("/users/{id}/posts", get(get_user_posts))
        .route("/users/{id}/followers", get(get_followers))
        .route("/users/{id}/following", get(get_following))
        .route("/trending", get(get_trending_posts))
        .route("/upload", post(upload_media))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 1000))]
    pub content: String,
    pub post_type: PostType,
    pub recipe_id: Option<Uuid>,
    pub media_urls: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, max = 500))]
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct FeedQueryParams {
    pub post_type: Option<PostType>,
    pub following_only: Option<bool>,
    pub tag: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UserPostsQueryParams {
    pub post_type: Option<PostType>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PostResponse {
    pub id: Uuid,
    pub content: String,
    pub post_type: PostType,
    pub recipe_id: Option<Uuid>,
    pub recipe_name: Option<String>,
    pub media_urls: Vec<String>,
    pub tags: Vec<String>,
    pub location: Option<String>,
    pub likes_count: i32,
    pub comments_count: i32,
    pub shares_count: i32,
    pub is_liked: bool,
    pub author: UserSummary,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CommentResponse {
    pub id: Uuid,
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
    pub likes_count: i32,
    pub replies_count: i32,
    pub is_liked: bool,
    pub author: UserSummary,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserSummary {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
    pub followers_count: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct FollowResponse {
    pub id: Uuid,
    pub user: UserSummary,
    pub followed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct MediaUploadResponse {
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub media_type: String,
    pub file_size: i64,
}

pub async fn create_post(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Json(payload): Json<CreatePostRequest>,
) -> Result<ResponseJson<PostResponse>, AppError> {
    payload.validate()?;

    let create_post = CreatePost {
        author_id: claims.sub,
        content: payload.content,
        post_type: payload.post_type,
        recipe_id: payload.recipe_id,
        media_urls: payload.media_urls.unwrap_or_default(),
        tags: payload.tags.unwrap_or_default(),
        location: payload.location,
    };

    let community_service = CommunityService::new(pool);
    let post = community_service.create_post(create_post).await?;

    Ok(ResponseJson(post))
}

pub async fn get_feed(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Query(params): Query<FeedQueryParams>,
) -> Result<ResponseJson<Vec<PostResponse>>, AppError> {
    let community_service = CommunityService::new(pool);
    let posts = community_service.get_feed(
        claims.sub,
        params.post_type,
        params.following_only.unwrap_or(false),
        params.tag,
        params.limit.unwrap_or(20),
        params.offset.unwrap_or(0),
    ).await?;

    Ok(ResponseJson(posts))
}

pub async fn get_post(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<PostResponse>, AppError> {
    let community_service = CommunityService::new(pool);
    let post = community_service.get_post_by_id(id, Some(claims.sub)).await?;

    Ok(ResponseJson(post))
}

pub async fn update_post(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<ResponseJson<PostResponse>, AppError> {
    payload.validate()?;

    let community_service = CommunityService::new(pool);
    let post = community_service.update_post(id, claims.sub, payload).await?;

    Ok(ResponseJson(post))
}

pub async fn delete_post(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let community_service = CommunityService::new(pool);
    community_service.delete_post(id, claims.sub).await?;

    Ok(ResponseJson(serde_json::json!({"message": "Post deleted successfully"})))
}

pub async fn toggle_like(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let community_service = CommunityService::new(pool);
    let is_liked = community_service.toggle_post_like(id, claims.sub).await?;

    Ok(ResponseJson(serde_json::json!({
        "is_liked": is_liked,
        "message": if is_liked { "Post liked" } else { "Post unliked" }
    })))
}

pub async fn create_comment(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(post_id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<ResponseJson<CommentResponse>, AppError> {
    payload.validate()?;

    let create_comment = CreateComment {
        post_id,
        author_id: claims.sub,
        content: payload.content,
        parent_comment_id: payload.parent_comment_id,
    };

    let community_service = CommunityService::new(pool);
    let comment = community_service.create_comment(create_comment).await?;

    Ok(ResponseJson(comment))
}

pub async fn get_comments(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(post_id): Path<Uuid>,
    Query(params): Query<FeedQueryParams>,
) -> Result<ResponseJson<Vec<CommentResponse>>, AppError> {
    let community_service = CommunityService::new(pool);
    let comments = community_service.get_post_comments(
        post_id,
        Some(claims.sub),
        params.limit.unwrap_or(50),
        params.offset.unwrap_or(0),
    ).await?;

    Ok(ResponseJson(comments))
}

pub async fn update_comment(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<ResponseJson<CommentResponse>, AppError> {
    payload.validate()?;

    let community_service = CommunityService::new(pool);
    let comment = community_service.update_comment(id, claims.sub, payload.content).await?;

    Ok(ResponseJson(comment))
}

pub async fn delete_comment(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let community_service = CommunityService::new(pool);
    community_service.delete_comment(id, claims.sub).await?;

    Ok(ResponseJson(serde_json::json!({"message": "Comment deleted successfully"})))
}

pub async fn toggle_follow(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    if claims.sub == user_id {
        return Err(AppError::BadRequest("Cannot follow yourself".to_string()));
    }

    let community_service = CommunityService::new(pool);
    let is_following = community_service.toggle_follow(claims.sub, user_id).await?;

    Ok(ResponseJson(serde_json::json!({
        "is_following": is_following,
        "message": if is_following { "Now following user" } else { "Unfollowed user" }
    })))
}

pub async fn get_user_posts(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(user_id): Path<Uuid>,
    Query(params): Query<UserPostsQueryParams>,
) -> Result<ResponseJson<Vec<PostResponse>>, AppError> {
    let community_service = CommunityService::new(pool);
    let posts = community_service.get_user_posts(
        user_id,
        Some(claims.sub),
        params.post_type,
        params.limit.unwrap_or(20),
        params.offset.unwrap_or(0),
    ).await?;

    Ok(ResponseJson(posts))
}

pub async fn get_followers(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<Vec<FollowResponse>>, AppError> {
    let community_service = CommunityService::new(pool);
    let followers = community_service.get_followers(user_id).await?;

    Ok(ResponseJson(followers))
}

pub async fn get_following(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<Vec<FollowResponse>>, AppError> {
    let community_service = CommunityService::new(pool);
    let following = community_service.get_following(user_id).await?;

    Ok(ResponseJson(following))
}

pub async fn get_trending_posts(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<Vec<PostResponse>>, AppError> {
    let community_service = CommunityService::new(pool);
    let posts = community_service.get_trending_posts(Some(claims.sub)).await?;

    Ok(ResponseJson(posts))
}

pub async fn upload_media(
    Extension(_pool): Extension<DbPool>,
    claims: Claims,
    // TODO: Implement multipart file upload
) -> Result<ResponseJson<MediaUploadResponse>, AppError> {
    let media_service = MediaService::new();
    
    // Placeholder implementation
    let placeholder_data = vec![0u8; 1024]; // 1KB placeholder
    let upload_result = media_service.upload_file(claims.sub, placeholder_data).await?;
    
    Ok(ResponseJson(upload_result))
}
