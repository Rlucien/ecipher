use axum::{
    routing::post,
    Router,
    Json,
};
use shared::{HelloRequest, HelloResponse};
use std::net::SocketAddr;

async fn hello(req: Json<HelloRequest>) -> Json<HelloResponse> {
    let message = format!("Hello, {}! (from server)", req.name);
    Json(HelloResponse { message })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", post(hello));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}