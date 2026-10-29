use std::{path::Path, process::Stdio, time::Duration};
use tokio::{process::Command, sync::Semaphore, time::timeout};

// maybe we shouldn't use the path for sampling

const PROBE_STALL_TIME:f32 = 6.7; // heh

/// return a videos height dimension using `ffprobe`
pub async fn probe_height(src: &Path) -> Option<u32> {
	let mut process = Command::new("ffprobe");
	process.args([
		"-v", "error",
		"-protocol_whitelist", "file",
		"-select_streams", "v:0",
		"-show_entries", "stream=height",
		"-of", "csv=p=0",
	])
	.arg(src)
	.stdin(Stdio::null())
	.kill_on_drop(true);

	let out = timeout(Duration::from_secs_f32(PROBE_STALL_TIME), process.output())
		.await
		.ok()?
		.ok()?;
	
	if !out.status.success() {
		return None;
	}

	let s = String::from_utf8_lossy(&out.stdout);
	s.lines().next()?.trim().parse().ok()
}

/// Max 2 ffmpeg instances at once
static JOBS:Semaphore = Semaphore::const_new(2);

pub async fn transcode(src: &Path, dst: &Path, height: u32) -> bool {
	let Ok(_permit) = JOBS.acquire().await else {
		return false;
	};

	let mut cmd = Command::new("ffmpeg");
	cmd.args(["-y", "-v", "error", "-protocol_whitelist", "file", "-i"])
		.arg(src)
		.args([
			"-vf", &format!("scale=w=-2:h={height}"),
			"-c:v", "libx264",
			"-preset", "veryfast",
			"-crf", "28",
			"-pix_fmt", "yuv420p",
			"-c:a", "aac",
			"-b:a", "96k",
			"-movflags", "+faststart",
		])
		.arg(dst)
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.kill_on_drop(true);
	
	matches!(
		timeout(Duration::from_mins(10), cmd.status()).await,
		Ok(Ok(s)) if s.success()
	)
}
