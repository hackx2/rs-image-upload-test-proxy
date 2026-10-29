use axum::{body::Bytes, http::StatusCode};
use serde::Serialize;
use tokio::task::spawn_blocking;

use crate::{
    hash, internal, sampler,
    storage::{
        self,
        FileVariant::{self, Original, Sample},
        SaveResult, VideoSampleVariant,
    },
};

#[derive(Serialize)]
pub struct UploadRes {
    hash: String,
    flags: u8, // pretty self explanatory, no?
    ext: &'static str,
    mime:&'static str
}

use crate::video;

async fn process_video(hash: &str, ext: &'static str) -> Result<(), (StatusCode, String)> {
    let src = storage::path(Original, hash, ext);

    let Some(height) = video::probe_height(&src).await else {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("an error occurred when trying to read file ({:?})", src).to_string(),
        ));
    };

    println!("height: {height}");
    //let Some(h) = video::
    //println!("{:?}", src);

    let hash = hash.to_owned();
    tokio::spawn(async move {
        let src = storage::path(FileVariant::Original, &hash, ext);

        // loop thru all the vid variants (480 & 720)p
        for res in VideoSampleVariant::ALL {
            if height <= res.h() {
                continue; // never make the video bigger than the original
            }

            let variant = FileVariant::Video(res);
            if storage::exists(variant, &hash, "mp4").await
                || storage::make_dir(variant, &hash, "mp4").await.is_err()
            {
                continue;
            }

            let tmp_p = storage::tmp_path(variant, &hash, "mp4");
            println!("{:?}", tmp_p);
            if video::transcode(&src, &tmp_p, res.h()).await {
                let _ = storage::promote(variant, &hash, "mp4").await;
            } else {
                let _ = tokio::fs::remove_file(&tmp_p).await;
                eprintln!("failed to sample ({}p) ~ {hash}", res.h());
            }
        }
    });

    Ok(())
}

/// Returns a file's extension by inferring the bytes.
/// This prevents people from uploading arbitrary file types spoofed to be something else.
/// e.g. javascript masked as a png
fn mime_and_ext_for(body: &[u8]) -> Option<(&'static str, &'static str)> {
    let mime = infer::get(body)?.mime_type();
    let ext = match mime {
        // imgs
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        // video
        "video/webm" => "webm",
        "video/mp4" => "mp4",
        _ => return None,
    };

    Some((mime, ext))
}

use crate::flags;
use axum::Json;

pub async fn handle(body: Bytes) -> Result<Json<UploadRes>, (StatusCode, String)> {
    if body.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "empty body".to_string()));
    }

    let (mime, ext) = mime_and_ext_for(&body).ok_or((
        StatusCode::UNSUPPORTED_MEDIA_TYPE,
        "Only IMAGES & VIDEOS are ALLOWED >:3".to_string(),
    ))?;

    //println!("{ext}");

    let hash = hash::get_sha256(&body);

    let is_new = storage::save(FileVariant::Original, &hash, ext, &body)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let fresh_upload = matches!(is_new, SaveResult::Saved);

    let is_video = matches!(ext, "webm" | "mp4");

    if is_video {
        println!("is video btw");
        process_video(&hash, ext).await?;
    } else if fresh_upload && ext != "gif" {
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
    let mut flags = flags::HAS_ORIGINAL;
    if fresh_upload {
        flags |= flags::IS_NEW;
    }
    if is_video {
        for res in VideoSampleVariant::ALL {
            if storage::exists(FileVariant::Video(res), &hash, "mp4").await {
                flags |= flags::HAS_SAMPLE | res.flag();
            }
        }
    } else if storage::exists(Sample, &hash, "webp").await {
        // add
        flags |= flags::HAS_SAMPLE;
    }

    println!(
        "{hash}.{ext} ({})",
        if fresh_upload {
            "saved <3"
        } else {
            "already exists lol"
        }
    );

    //Ok(format!("{hash}.{ext}"))
    Ok(Json(UploadRes {
        hash,
        ext,
        flags,
        mime
    }))
}
