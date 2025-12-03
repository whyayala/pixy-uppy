use std::env;
use std::path::{Path, PathBuf};

use which::which;

use crate::error::PixyError;

/// Returns the platform subdirectory name used under `third_party/bin`.
/// This is used to look up bundled binaries for Windows and Linux builds.
pub fn platform_dir() -> &'static str {
    if cfg!(target_os = "windows") {
        "win64"
    } else {
        "linux64"
    }
}

/// Attempts to resolve a tool (e.g., `ffmpeg`, `ffprobe`, `realesrgan-ncnn-vulkan`) by:
/// 1) `which` on PATH
/// 2) `PIXY_UPPY_BIN_DIR` env var
/// 3) `third_party/bin/<platform>/` relative to current working directory
/// 4) `third_party/bin/<platform>/` relative to executable directory
pub fn resolve_tool(tool_name: &str) -> Result<PathBuf, PixyError> {
    if let Ok(p) = which(tool_name) {
        return Ok(p);
    }

    if let Ok(dir) = env::var("PIXY_UPPY_BIN_DIR") {
        let candidate = Path::new(&dir).join(tool_name);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    // relative to CWD
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let candidate = cwd
        .join("third_party")
        .join("bin")
        .join(platform_dir())
        .join(tool_name);
    if candidate.exists() {
        return Ok(candidate);
    }

    // relative to executable dir
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir
                .join("third_party")
                .join("bin")
                .join(platform_dir())
                .join(tool_name);
            if candidate.exists() {
                return Ok(candidate);
            }
        }
    }

    Err(PixyError::CommandNotFound(Box::leak(
        tool_name.to_string().into_boxed_str(),
    )))
}
