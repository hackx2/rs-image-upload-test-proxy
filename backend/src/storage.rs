use std::path::PathBuf;

use tokio::{
	fs::{self, OpenOptions},
	io::{self, AsyncWriteExt},
};

use crate::flags;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VideoSampleVariant {
	_480p,
	_720p,
}

impl VideoSampleVariant {
	pub const ALL: [Self; 2] = [Self::_480p, Self::_720p];

	// returns the height of the video
	pub fn h(self) -> u32 {
		match self {
			Self::_480p => 480,
			Self::_720p => 720,
		}
	}

	pub fn flag(self) -> u8 {
		match self {
			Self::_480p => flags::HAS_480P,
			Self::_720p => flags::HAS_720P,
		}
	}
}

#[derive(Clone, Copy)]
pub enum FileVariant {
	Sample,
	Original,

	// video
	Video(VideoSampleVariant),
}

impl FileVariant {
	// Return's the file variants data directory
	fn dir(self) -> &'static str {
		match self {
			Self::Original => "raw",
			Self::Sample | Self::Video(_) => "sample",
		}
	}

	// return's the file variant's file name
	fn fname(self, hash: &str, ext: &str) -> String {
		match self {
			Self::Video(res) => format!("{hash}_{}p.{ext}", res.h()),
			_ => format!("{hash}.{ext}"),
		}
	}
}

pub fn path(variant: FileVariant, hash: &str, ext: &str) -> PathBuf {
	let mut buf = PathBuf::from("../data"); // heh
	buf.push(variant.dir());
	buf.push(&hash[0..2]); // e.g. 12
	buf.push(&hash[2..4]); // e.g. 34
	buf.push(variant.fname(hash, ext));
	buf
}

/// temporary path
pub fn tmp_path(variant: FileVariant, hash: &str, ext: &str) -> PathBuf {
	path(variant, hash, ext).with_extension(format!("tmp.{ext}"))
}

/// creates the directory
pub async fn make_dir(variant: FileVariant, hash: &str, ext: &str) -> io::Result<PathBuf> {
	let p = path(variant, hash, ext);
	if let Some(dir) = p.parent() {
		fs::create_dir_all(dir).await?;
	}
	Ok(p)
}

/// promotes a tmp file
pub async fn promote(variant: FileVariant, hash: &str, ext: &str) -> io::Result<()> {
	fs::rename(tmp_path(variant, hash, ext), path(variant, hash, ext)).await
}

pub enum SaveResult {
	Saved,
	AlreadyExisted,
}

pub async fn save(
	variant: FileVariant,
	hash: &str,
	ext: &str,
	bytes: &[u8],
) -> io::Result<SaveResult> {
	let path = make_dir(variant, hash, ext).await.unwrap();

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

/// whether a file exists using `variant`, `hash`, and `ext`
pub async fn exists(variant: FileVariant, hash: &str, ext: &str) -> bool {
	fs::try_exists(path(variant, hash, ext))
		.await
		.unwrap_or(false)
}
