mod config;
mod error;

use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // .envファイルの読み込み
    dotenvy::dotenv().ok();

    // ロギングの初期化
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rust_web_test02=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 設定の読み込み
    let config = config::Config::from_env().expect("Failed to load configuration");

    tracing::info!("Configuration loaded: {:?}", config);

    // ルーターの作成
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/user/{name}", get(greet_user))
        .route("/error/notfound", get(test_not_found))
        .route("/error/badrequest", get(test_bad_request))
        .route("/error/internal", get(test_internal_error))
        .layer(TraceLayer::new_for_http());

    let addr = config.addr();

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");

    tracing::info!("Server runnning on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

// ハンドラ関数
async fn hello_world() -> &'static str {
    tracing::info!("hello_world endpoint accessed");
    "Hello, World!"
}

async fn greet_user(axum::extract::Path(name): axum::extract::Path<String>) -> String {
    tracing::info!("Greeting user: {}", name);
    format!("Hello, {}!", name)
}

// エラーハンドリングのテスト用エンドポイント
async fn test_not_found() -> error::Result<String> {
    Err(error::AppError::NotFound("User not found".to_string()))
}

async fn test_bad_request() -> error::Result<String> {
    Err(error::AppError::BadRequest(
        "Invalid parameters".to_string(),
    ))
}

async fn test_internal_error() -> error::Result<String> {
    Err(error::AppError::InternalError)
}
