use axum::{
    Router,
    body::Bytes,
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::post,
};
use tower_http::services::ServeDir;

mod hash;
mod storage;

use crate::storage::{FileVariant, SaveResult};

async fn handle_upload(body: Bytes) -> Result<String, (StatusCode, String)> {
    if body.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "empty body".to_string()));
    }

    let ext = match infer::get(&body).map(|t| t.mime_type()) {
        Some("image/png") => "png",
        Some("image/jpeg") => "jpg",
        Some("image/gif") => "gif",
        Some("image/webp") => "webp",
        _ => {
            return Err((
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "images only >:3".to_string(),
            ));
        }
    };

	//println!("{ext}");

    let hash = hash::get_sha256(&body);

    let is_new = storage::save(FileVariant::Original, &hash, ext, &body)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    println!(
        "{hash}.{ext} ({})",
        match is_new {
            SaveResult::AlreadyExisted => "already exists lol",
            SaveResult::Saved => "saved <3",
        }
    );

    Ok(format!("{hash}.{ext}"))
}

const ADDR: &str = "127.0.0.1";
const PORT: i32 = 3001;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/upload", post(handle_upload))
        // nginx should do this instead but eh
        .nest_service("/data", ServeDir::new("../data"))
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024));

    let listener = tokio::net::TcpListener::bind(format!("{ADDR}:{PORT}"))
        .await
        .unwrap();
    println!("listening on {ADDR}:{PORT}");
    axum::serve(listener, app).await.unwrap();
}
