use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::{
    models::community::{CreatePost, CreateComment, PostType},
    api::community::{PostResponse, CommentResponse, FollowResponse, UserSummary},
    services::realtime::RealtimeService,
    utils::errors::AppError,
};

pub struct CommunityService {
    pool: crate::db::DbPool,
    realtime_service: Option<Arc<RealtimeService>>,
}

impl CommunityService {
    pub fn new(pool: crate::db::DbPool) -> Self {
        Self { 
            pool,
            realtime_service: None,
        }
    }

    pub fn with_realtime(pool: crate::db::DbPool, realtime_service: Arc<RealtimeService>) -> Self {
        Self { 
            pool,
            realtime_service: Some(realtime_service),
        }
    }

    pub async fn create_post(&self, post: CreatePost) -> Result<PostResponse, AppError> {
        // Mock implementation - in production, this would save to database
        let post_id = Uuid::new_v4();
        
        let post_response = PostResponse {
            id: post_id,
            content: post.content.clone(),
            post_type: post.post_type,
            recipe_id: post.recipe_id,
            recipe_name: if post.recipe_id.is_some() { 
                Some("Mock Recipe Name".to_string()) 
            } else { 
                None 
            },
            media_urls: post.media_urls,
            tags: post.tags,
            location: post.location,
            likes_count: 0,
            comments_count: 0,
            shares_count: 0,
            is_liked: false,
            author: self.get_mock_user_summary(post.author_id).await,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Отправляем WebSocket уведомление о новом посте
        if let Some(realtime_service) = &self.realtime_service {
            let author_name = format!("{} {}", 
                post_response.author.first_name, 
                post_response.author.last_name
            );
            let _ = realtime_service.notify_new_post(
                post_id,
                author_name,
                post.content.clone(),
            ).await;
        }
        
        Ok(post_response)
    }

    pub async fn get_feed(
        &self,
        user_id: Uuid,
        post_type: Option<PostType>,
        _following_only: bool,
        _tag: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostResponse>, AppError> {
        // Mock implementation
        self.get_mock_posts(Some(user_id), post_type, limit, offset).await
    }

    pub async fn get_post_by_id(&self, id: Uuid, user_id: Option<Uuid>) -> Result<PostResponse, AppError> {
        // Mock implementation
        self.get_mock_post(id, user_id).await
    }

    pub async fn update_post(
        &self,
        id: Uuid,
        user_id: Uuid,
        payload: crate::api::community::CreatePostRequest,
    ) -> Result<PostResponse, AppError> {
        // Mock implementation - in production, verify ownership and update database
        Ok(PostResponse {
            id,
            content: payload.content,
            post_type: payload.post_type,
            recipe_id: payload.recipe_id,
            recipe_name: if payload.recipe_id.is_some() { 
                Some("Updated Recipe Name".to_string()) 
            } else { 
                None 
            },
            media_urls: payload.media_urls.unwrap_or_default(),
            tags: payload.tags.unwrap_or_default(),
            location: payload.location,
            likes_count: 15,
            comments_count: 8,
            shares_count: 3,
            is_liked: true,
            author: self.get_mock_user_summary(user_id).await,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn delete_post(&self, _id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        // Mock implementation - in production, verify ownership and delete from database
        Ok(())
    }

    pub async fn toggle_post_like(&self, _post_id: Uuid, _user_id: Uuid) -> Result<bool, AppError> {
        // Mock implementation - in production, toggle like status in database
        Ok(true) // Return true indicating post is now liked
    }

    pub async fn create_comment(&self, comment: CreateComment) -> Result<CommentResponse, AppError> {
        // Mock implementation
        let comment_id = Uuid::new_v4();
        
        Ok(CommentResponse {
            id: comment_id,
            content: comment.content,
            parent_comment_id: comment.parent_comment_id,
            likes_count: 0,
            replies_count: 0,
            is_liked: false,
            author: self.get_mock_user_summary(comment.author_id).await,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn get_post_comments(
        &self,
        _post_id: Uuid,
        user_id: Option<Uuid>,
        limit: i64,
        _offset: i64,
    ) -> Result<Vec<CommentResponse>, AppError> {
        // Mock implementation
        self.get_mock_comments(user_id, limit).await
    }

    pub async fn update_comment(
        &self,
        id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Result<CommentResponse, AppError> {
        // Mock implementation
        Ok(CommentResponse {
            id,
            content,
            parent_comment_id: None,
            likes_count: 5,
            replies_count: 2,
            is_liked: false,
            author: self.get_mock_user_summary(user_id).await,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn delete_comment(&self, _id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        // Mock implementation - in production, verify ownership and delete from database
        Ok(())
    }

    pub async fn toggle_follow(&self, _follower_id: Uuid, _following_id: Uuid) -> Result<bool, AppError> {
        // Mock implementation - in production, toggle follow status in database
        Ok(true) // Return true indicating now following
    }

    pub async fn get_user_posts(
        &self,
        user_id: Uuid,
        _viewer_id: Option<Uuid>,
        post_type: Option<PostType>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostResponse>, AppError> {
        // Mock implementation
        self.get_mock_posts(Some(user_id), post_type, limit, offset).await
    }

    pub async fn get_followers(&self, user_id: Uuid) -> Result<Vec<FollowResponse>, AppError> {
        // Mock implementation
        self.get_mock_follows(user_id, true).await
    }

    pub async fn get_following(&self, user_id: Uuid) -> Result<Vec<FollowResponse>, AppError> {
        // Mock implementation
        self.get_mock_follows(user_id, false).await
    }

    pub async fn get_trending_posts(&self, user_id: Option<Uuid>) -> Result<Vec<PostResponse>, AppError> {
        // Mock implementation - return posts sorted by popularity
        self.get_mock_posts(user_id, None, 10, 0).await
    }

    // Mock implementations for testing without database
    async fn get_mock_user_summary(&self, user_id: Uuid) -> UserSummary {
        UserSummary {
            id: user_id,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            is_verified: true,
            followers_count: 1250,
        }
    }

    async fn get_mock_post(&self, id: Uuid, user_id: Option<Uuid>) -> Result<PostResponse, AppError> {
        let author_id = user_id.unwrap_or_else(Uuid::new_v4);
        
        Ok(PostResponse {
            id,
            content: "Check out this amazing recipe I just made! 🍝".to_string(),
            post_type: PostType::Recipe,
            recipe_id: Some(Uuid::new_v4()),
            recipe_name: Some("Delicious Pasta Carbonara".to_string()),
            media_urls: vec!["https://example.com/pasta1.jpg".to_string()],
            tags: vec!["pasta".to_string(), "italian".to_string(), "dinner".to_string()],
            location: Some("Kitchen".to_string()),
            likes_count: 42,
            comments_count: 18,
            shares_count: 7,
            is_liked: user_id.is_some(),
            author: self.get_mock_user_summary(author_id).await,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    async fn get_mock_posts(
        &self,
        user_id: Option<Uuid>,
        post_type: Option<PostType>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostResponse>, AppError> {
        let mut posts = vec![];
        
        // Generate different mock posts
        for i in 0..std::cmp::min(limit, 10) {
            let post_id = Uuid::new_v4();
            let author_id = user_id.unwrap_or_else(Uuid::new_v4);
            
            let mock_post_type = match i % 3 {
                0 => PostType::Recipe,
                1 => PostType::Photo,
                _ => PostType::Text,
            };

            // Filter by post_type if specified
            if let Some(filter_type) = &post_type {
                if mock_post_type != *filter_type {
                    continue;
                }
            }
            
            let post = PostResponse {
                id: post_id,
                content: format!("This is mock post {} with some interesting content!", i + 1),
                post_type: mock_post_type.clone(),
                recipe_id: if mock_post_type == PostType::Recipe { 
                    Some(Uuid::new_v4()) 
                } else { 
                    None 
                },
                recipe_name: if mock_post_type == PostType::Recipe { 
                    Some(format!("Recipe {}", i + 1)) 
                } else { 
                    None 
                },
                media_urls: if mock_post_type != PostType::Text {
                    vec![format!("https://example.com/image{}.jpg", i + 1)]
                } else {
                    vec![]
                },
                tags: vec![format!("tag{}", i + 1), "food".to_string()],
                location: Some(format!("Location {}", i + 1)),
                likes_count: (i as i32 + 1) * 10,
                comments_count: (i as i32 + 1) * 3,
                shares_count: (i as i32 + 1),
                is_liked: i % 2 == 0,
                author: self.get_mock_user_summary(author_id).await,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            posts.push(post);
        }
        
        let start = offset as usize;
        let end = std::cmp::min(start + limit as usize, posts.len());
        
        if start >= posts.len() {
            Ok(vec![])
        } else {
            Ok(posts[start..end].to_vec())
        }
    }

    async fn get_mock_comments(&self, user_id: Option<Uuid>, limit: i64) -> Result<Vec<CommentResponse>, AppError> {
        let mut comments = vec![];
        
        for i in 0..std::cmp::min(limit, 5) {
            let comment_id = Uuid::new_v4();
            let author_id = user_id.unwrap_or_else(Uuid::new_v4);
            
            let comment = CommentResponse {
                id: comment_id,
                content: format!("This is a great comment number {}!", i + 1),
                parent_comment_id: if i > 2 { Some(Uuid::new_v4()) } else { None },
                likes_count: (i as i32 + 1) * 2,
                replies_count: if i < 2 { i as i32 } else { 0 },
                is_liked: i % 2 == 1,
                author: self.get_mock_user_summary(author_id).await,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            comments.push(comment);
        }
        
        Ok(comments)
    }

    async fn get_mock_follows(&self, _user_id: Uuid, _is_followers: bool) -> Result<Vec<FollowResponse>, AppError> {
        let mut follows = vec![];
        
        for i in 0..5 {
            let follow_id = Uuid::new_v4();
            let user_id = Uuid::new_v4();
            
            let follow = FollowResponse {
                id: follow_id,
                user: UserSummary {
                    id: user_id,
                    first_name: format!("User{}", i + 1),
                    last_name: "Smith".to_string(),
                    avatar_url: Some(format!("https://example.com/avatar{}.jpg", i + 1)),
                    is_verified: i % 2 == 0,
                    followers_count: (i as i32 + 1) * 100,
                },
                followed_at: Utc::now(),
            };
            follows.push(follow);
        }
        
        Ok(follows)
    }
}
