use axum::body::Bytes;
use sha2::{Digest, Sha256};

pub fn get_sha256(bytes: &Bytes) -> String {
	let digest = Sha256::digest(bytes);
	hex::encode(digest)
}