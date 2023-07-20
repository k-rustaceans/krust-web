use chrono::{DateTime, Utc};

pub struct Post {
    pub id: i64,
    pub user_id: String,
    pub create_dt: DateTime<Utc>,
    pub update_dt: DateTime<Utc>,
    pub state: PostState,
}
pub enum PostState {
    Created,
    Deleted,
}

pub struct PostContent {
    pub id: i64,
    pub post_id: i64,
    pub title: String,
    pub content: String,
    pub create_dt: DateTime<Utc>,
}
pub struct PostLike {
    pub user_id: String,
    pub post_id: i64,
}

pub struct Comment {
    pub id: i64,
    pub post_id: i64,
    pub user_id: String,
    pub content: String,
    pub create_dt: DateTime<Utc>,
    pub update_dt: DateTime<Utc>,
}

pub struct CommentLike {
    pub user_id: String,
    pub comment_id: i64,
}
