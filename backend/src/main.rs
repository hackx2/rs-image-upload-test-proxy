use axum::{
	Router,
	body::Bytes,
	extract::DefaultBodyLimit,
	http::StatusCode,
	routing::post,
};
use tokio::task::spawn_blocking;
use tower_http::services::ServeDir;

mod sampler;
mod hash;
mod storage;

use crate::storage::{FileVariant::{self, Sample}, SaveResult};

fn internal<E:ToString>(err:E) -> (StatusCode, String) {
	(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// stupid bitmaskin' stuff
const IMG_HAS_ORIGINAL:u8 = 1 << 0;
const IMG_HAS_SAMPLE:u8 = 1 << 1;

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
struct UploadRes {
	hash: String,
	ext: &'static str,
	flags: u8,
	new_upload:bool // pretty self explanatory, no?
}

async fn handle_upload(body: Bytes) -> Result<Json<UploadRes>, (StatusCode, String)> {
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

	let fresh_upload = matches!(is_new, SaveResult::Saved);

	if fresh_upload {
		let b = body.clone();

		let sample = spawn_blocking(move || sampler::sample_image(&b))
			.await
			.map_err(internal)?;

		if let Some(sample) = sample {
			storage::save(FileVariant::Sample, &hash, "webp", &sample)
				.await
				.map_err(internal)?;	
		}
	}

	// create bit flags ^w^
	// idk what it is but i prefer using bits instead of booleans
	let mut flags = IMG_HAS_ORIGINAL;
	if storage::exists(Sample, &hash, "webp").await {
		// add
		flags |= IMG_HAS_SAMPLE;
	}

	println!(
		"{hash}.{ext} ({})",
		if fresh_upload { "saved <3" } else { "already exists lol" }
	);
	
	//Ok(format!("{hash}.{ext}"))
	Ok(Json(UploadRes { hash, ext, flags, new_upload: fresh_upload}))
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
