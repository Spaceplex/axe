use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:3030").await.unwrap();

    let service = Router::new();

    axum::serve(listener, service.into_make_service())
        .await
        .unwrap();
}
