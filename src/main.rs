use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // ロギングの初期化
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rust_web_test02=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ルーターの作成
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/user/{name}", get(greet_user))
        .layer(TraceLayer::new_for_http());

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Server runnning on http://0.0.0.0:3000");

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
