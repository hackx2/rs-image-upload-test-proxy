#[macro_use]
extern crate dotenv_codegen;

use axum::{Router, extract::DefaultBodyLimit, http::StatusCode, routing::post};
use tower_http::services::ServeDir;

mod flags;
mod hash;
mod routes;
mod sampler;
mod storage;
mod video;

const ADDR: &str = dotenv!("BACKEND_IP");
const PORT: &str = dotenv!("BACKEND_PORT");

const BODY_LIMIT: usize = 100 * 1024 * 1024; // ~100mb (also = 100 << 20)

#[tokio::main]
async fn main() {
    dotenv::from_path("../.env").expect("failed to load .env :c");

    // cast from str -> int
    let port: i32 = PORT.parse().unwrap();

    let app = Router::new()
        .route("/upload", post(routes::upload::handle))
        .nest_service("/data", ServeDir::new("../data")) // nginx should do this instead but eh
        .layer(DefaultBodyLimit::max(BODY_LIMIT));

    let listener = tokio::net::TcpListener::bind(format!("{ADDR}:{port}"))
        .await
        .unwrap();
    println!("listening on {ADDR}:{port}");
    axum::serve(listener, app).await.unwrap();
}

// suree

pub fn internal<E: ToString>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
