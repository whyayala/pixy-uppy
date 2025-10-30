use std::path::Path;
use std::process::Command;

use serde::Deserialize;
use which::which;

use crate::error::PixyError;

#[derive(Debug, Clone, Deserialize)]
pub struct MediaInfo {
    pub format: serde_json::Value,
    pub streams: Vec<serde_json::Value>,
}

pub fn probe_media(input: &Path) -> Result<MediaInfo, PixyError> {
    if which("ffprobe").is_err() {
        return Err(PixyError::CommandNotFound("ffprobe"));
    }
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
        ])
        .arg(input)
        .output()?;

    if !output.status.success() {
        return Err(PixyError::ProcessFailed {
            cmd: "ffprobe".to_string(),
            code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }

    #[derive(Deserialize)]
    struct ProbeOut {
        format: serde_json::Value,
        streams: Vec<serde_json::Value>,
    }

    let parsed: ProbeOut = serde_json::from_slice(&output.stdout)?;
    Ok(MediaInfo { format: parsed.format, streams: parsed.streams })
}


