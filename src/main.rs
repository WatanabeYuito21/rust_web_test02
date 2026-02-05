use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // ルーターの作成
    let app = Router::new().route("/", get(hello_world));

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

// ハンドラ関数
async fn hello_world() -> &'static str {
    "Hello, World!"
}
