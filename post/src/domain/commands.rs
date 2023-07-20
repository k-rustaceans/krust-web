pub struct CreatePost {
    pub user_id: String,
    pub title: String,
    pub content: String,
}

pub struct LikePost {
    pub user_id: String,
    pub post_id: i64,
    //TODO validation required
}

pub struct EditPost {
    pub user_id: String,
    pub title: Option<String>,
    pub content: Option<String>,
}

pub struct DeletePost {
    pub user_id: String,
    pub post_id: i64,
}

//TODO -> CommentAdded
pub struct AddComment {
    pub post_id: i64,
    pub user_id: String,
    pub content: String,
}

pub struct EditComment {
    pub comment_id: i64,
    pub user_id: String,
    pub content: String,
}

pub struct DeleteComment {
    pub comment_id: i64,
    pub user_id: String,
}

pub struct LikeComment {
    pub user_id: String,
    pub comment_id: i64,
}
