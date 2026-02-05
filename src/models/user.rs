use serde::{Deserialize, Serialize};

// ユーザー情報を表す構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

// ユーザー作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

// ユーザー作成レスポンス
#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub message: String,
    pub user: User,
}
