use crate::{
    error::Result,
    models::user::{CreateUserRequest, CreateUserResponse, User},
};
use axum::{Json, extract::Path};

// ユーザー一覧取得(GET)
pub async fn list_users() -> Result<Json<Vec<User>>> {
    // ダミーデータ
    let users = vec![
        User {
            id: 1,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    tracing::info!("Fetched {} users", users.len());
    Ok(Json(users))
}

// ユーザー詳細取得(パスパラメータ)
pub async fn get_user(Path(user_id): Path<u32>) -> Result<Json<User>> {
    tracing::info!("Getting user with id: {}", user_id);

    // ダミーデータ
    let user = User {
        id: user_id,
        username: format!("user_{}", user_id),
        email: format!("user_{}@example.com", user_id),
    };

    Ok(Json(user))
}

// ユーザー作成(POST)
pub async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>> {
    tracing::info!("Creating user: {:?}", payload);

    // バリデーション
    if payload.username.is_empty() {
        return Err(crate::error::AppError::BadRequest(
            "Username cannot be empty".to_string(),
        ));
    }

    if payload.email.is_empty() || !payload.email.contains('@') {
        return Err(crate::error::AppError::BadRequest(
            "Invalid email address".to_string(),
        ));
    }

    // ダミーユーザー作成
    let user = User {
        id: 999,
        username: payload.username,
        email: payload.email,
    };

    let response = CreateUserResponse {
        message: "User created successfully".to_string(),
        user,
    };

    Ok(Json(response))
}

// ユーザー削除(DELETE)
pub async fn delete_user(Path(user_id): Path<u32>) -> Result<Json<serde_json::Value>> {
    tracing::info!("Deleting user with id: {}", user_id);

    // ダミー削除成功レスポンス
    Ok(Json(serde_json::json!({
        "message": format!("User {} deleted successfully", user_id),
    })))
}
