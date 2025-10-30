use std::path::{Path, PathBuf};
use std::process::Command;

use crate::encoder::EncoderOptions;
use crate::error::PixyError;
use crate::frames::{extract_frames, FrameExtractOptions};
use crate::models::ModelSpec;
use crate::upscalers::{UpscalerBinary, UpscalerKind};

#[derive(Debug, Clone)]
pub struct UpscaleJob {
    pub input: PathBuf,
    pub output: PathBuf,
    pub model: ModelSpec,
    pub upscaler: UpscalerBinary,
    pub gpu_index: usize,
    pub tile_size: Option<u32>,
    pub threads: Option<u32>,
    pub target_width: Option<u32>,
    pub target_height: Option<u32>,
    pub scale: Option<u32>,
    pub extract: FrameExtractOptions,
    pub encoder: EncoderOptions,
    pub container: String,
}

#[derive(Debug, Clone)]
pub struct UpscaleJobProgress {
    pub stage: String,
    pub percent: f32,
}

pub fn run_upscale_job(job: &UpscaleJob) -> Result<(), PixyError> {
    let temp_root = std::env::temp_dir().join("pixy-uppy");
    std::fs::create_dir_all(&temp_root)?;
    let frames_dir = temp_root.join("frames");
    let upscaled_dir = temp_root.join("upscaled");

    let in_path = Path::new(&job.input);
    let frames_pattern = extract_frames(in_path, &frames_dir, &job.extract)?;
    let upscaled_pattern = upscaled_dir.join(frames_pattern.file_name().unwrap());
    std::fs::create_dir_all(&upscaled_dir)?;

    job.upscaler.run(&frames_pattern, &upscaled_pattern, job.gpu_index, job.tile_size, job.threads, &job.model)?;

    let vf = build_vf(job);
    let mut args = vec![
        "-y".into(),
        "-framerate".into(),
        "24".into(),
        "-i".into(),
        upscaled_pattern.to_string_lossy().to_string(),
        "-i".into(),
        job.input.to_string_lossy().to_string(),
        "-map".into(),
        "0:v:0".into(),
        "-map".into(),
        "1:a?".into(),
        "-map".into(),
        "1:s?".into(),
        "-map".into(),
        "1:t?".into(),
        "-c:a".into(),
        "copy".into(),
        "-c:s".into(),
        "copy".into(),
        "-c:t".into(),
        "copy".into(),
    ];

    if let Some(vf) = vf { args.push("-vf".into()); args.push(vf); }
    args.extend(job.encoder.to_ffmpeg_args());
    args.push(job.output.to_string_lossy().to_string());

    let status = Command::new("ffmpeg").args(args.clone()).status()?;
    if !status.success() {
        return Err(PixyError::ProcessFailed { cmd: format!("ffmpeg {:?}", args), code: status.code(), stderr: String::new() });
    }

    Ok(())
}

fn build_vf(job: &UpscaleJob) -> Option<String> {
    let mut vf_parts: Vec<String> = Vec::new();
    match (job.target_width, job.target_height, job.scale) {
        (Some(w), Some(h), _) => vf_parts.push(format!("zscale=w={}:h={}:filter=spline36", w, h)),
        (Some(w), None, _) => vf_parts.push(format!("zscale=w={}:h=-1:filter=spline36", w)),
        (None, Some(h), _) => vf_parts.push(format!("zscale=w=-1:h={}:filter=spline36", h)),
        (None, None, Some(_s)) => { /* rely on model */ }
        _ => {}
    }
    if vf_parts.is_empty() { None } else { Some(vf_parts.join(",")) }
}


