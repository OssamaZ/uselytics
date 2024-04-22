use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
  let router = Router::new().route("/", get(handler));
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
  println!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, router).await.unwrap();
}

async fn handler() -> &'static str {
  "Hello World"
}
