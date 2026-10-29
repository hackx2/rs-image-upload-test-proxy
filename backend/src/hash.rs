use axum::body::Bytes;
use sha2::{Digest, Sha256};

/// Return's `sha256` digest of `bytes` as a hex-encoded string
pub fn get_sha256(bytes: &Bytes) -> String {
	let digest = Sha256::digest(bytes);
	hex::encode(digest)
}