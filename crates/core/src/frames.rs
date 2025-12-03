use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::PixyError;
use crate::paths::resolve_tool;

/// Optional denoise/deinterlace/deband filters applied before frame extraction.
#[derive(Debug, Clone, Copy)]
pub enum Prefilter {
    Yadif,
    Hqdn3d,
    Deband,
    None,
}

impl Prefilter {
    fn filter_str(self) -> Option<&'static str> {
        match self {
            Prefilter::Yadif => Some("yadif=0:-1:1"),
            Prefilter::Hqdn3d => Some("hqdn3d"),
            Prefilter::Deband => Some("deband"),
            Prefilter::None => None,
        }
    }
}

/// Options controlling frame extraction behavior and image format.
#[derive(Debug, Clone)]
pub struct FrameExtractOptions {
    pub prefilter: Prefilter,
    pub frame_format: String, // png|webp|bmp
}

impl Default for FrameExtractOptions {
    fn default() -> Self {
        Self {
            prefilter: Prefilter::None,
            frame_format: "png".into(),
        }
    }
}

/// Extracts frames from the input using ffmpeg to an output directory.
/// Why: Upscaler binaries operate on image sequences; we preserve PTS for sync.
pub fn extract_frames(
    input: &Path,
    out_dir: &Path,
    opts: &FrameExtractOptions,
) -> Result<PathBuf, PixyError> {
    std::fs::create_dir_all(out_dir)?;
    let pattern = out_dir.join(format!("%08d.{}", opts.frame_format));
    let mut args = vec![
        "-y".into(),
        "-i".into(),
        input.to_string_lossy().to_string(),
        "-vsync".into(),
        "0".into(),
        "-frame_pts".into(),
        "1".into(),
    ];

    if let Some(f) = opts.prefilter.filter_str() {
        args.push("-vf".into());
        args.push(f.into());
    }

    args.push(pattern.to_string_lossy().to_string());

    let ffmpeg = resolve_tool("ffmpeg")?;
    let status = Command::new(ffmpeg).args(args.clone()).status()?;
    if !status.success() {
        return Err(PixyError::ProcessFailed {
            cmd: format!("ffmpeg {:?}", args),
            code: status.code(),
            stderr: String::new(),
        });
    }
    Ok(pattern)
}
