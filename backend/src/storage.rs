use std::path::PathBuf;

use tokio::{
	fs::{self, OpenOptions},
	io::{self, AsyncWriteExt},
};

fn path(hash: &str, ext: &str) -> PathBuf {
	let mut buf = PathBuf::from("..\\data"); // heh
	buf.push(&hash[0..2]);
	buf.push(&hash[2..4]);
	buf.push(format!("{hash}.{ext}"));
	buf
}

pub enum SaveResult {
	Saved,
	AlreadyExisted,
}

pub async fn save(hash: &str, ext: &str, bytes: &[u8]) -> io::Result<SaveResult> {
	let path = path(hash, ext);

	if let Some(dir) = path.parent() {
		fs::create_dir_all(dir).await?;
	}

	let mut file = match OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(&path)
		.await
	{
		Ok(f) => f,
		Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
			return Ok(SaveResult::AlreadyExisted);
		}
		Err(e) => return Err(e),
	};

	file.write_all(bytes).await?;

	Ok(SaveResult::Saved)
}
