use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::PixyError;
use crate::models::ModelSpec;
use crate::paths::resolve_tool;

#[derive(Debug, Clone, Copy)]
pub enum UpscalerKind {
    RealEsrgan,
    RealCugan,
    Waifu2x,
}

/// Represents an upscaler binary by kind and path on disk.
#[derive(Debug, Clone)]
pub struct UpscalerBinary {
    pub kind: UpscalerKind,
    pub path: PathBuf,
}

impl UpscalerBinary {
    /// Runs the upscaler on an image sequence, writing an output sequence.
    /// Why: We isolate invocation details and flags per binary in one place.
    pub fn run(
        &self,
        input_pattern: &Path,
        output_pattern: &Path,
        gpu: usize,
        tile_size: Option<u32>,
        threads: Option<u32>,
        model: &ModelSpec,
    ) -> Result<(), PixyError> {
        let mut cmd = match self.kind {
            UpscalerKind::RealEsrgan => {
                let mut c = Command::new(&self.path);
                let input_arg = sequence_dir_arg(input_pattern);
                let output_arg = sequence_dir_arg(output_pattern);
                c.args([
                    "-i",
                    input_arg.as_ref(),
                    "-o",
                    output_arg.as_ref(),
                    "-n",
                    &model.name,
                    "-g",
                ])
                .arg(gpu.to_string());
                if let Some(t) = tile_size {
                    c.args(["-t", &t.to_string()]);
                }
                if let Some(th) = threads {
                    c.args(["-j", &format!("{}:{}:{}", th, th, th)]);
                }
                c
            }
            UpscalerKind::RealCugan => {
                let mut c = Command::new(&self.path);
                c.args([
                    "-i",
                    input_pattern.to_string_lossy().as_ref(),
                    "-o",
                    output_pattern.to_string_lossy().as_ref(),
                    "-g",
                ])
                .arg(gpu.to_string());
                if let Some(t) = tile_size {
                    c.args(["-t", &t.to_string()]);
                }
                if let Some(level) = model.denoise_level {
                    c.args(["-n", &level.to_string()]);
                }
                c
            }
            UpscalerKind::Waifu2x => {
                let mut c = Command::new(&self.path);
                c.args([
                    "-i",
                    input_pattern.to_string_lossy().as_ref(),
                    "-o",
                    output_pattern.to_string_lossy().as_ref(),
                    "-g",
                ])
                .arg(gpu.to_string());
                if let Some(t) = tile_size {
                    c.args(["-t", &t.to_string()]);
                }
                if let Some(level) = model.denoise_level {
                    c.args(["-n", &level.to_string()]);
                }
                c
            }
        };

        let status = cmd.status()?;
        if !status.success() {
            return Err(PixyError::ProcessFailed {
                cmd: format!("{:?}", cmd),
                code: status.code(),
                stderr: String::new(),
            });
        }
        Ok(())
    }
}

fn sequence_dir_arg(path: &Path) -> Cow<'_, str> {
    let as_str = path.to_string_lossy();
    if as_str.contains('%') {
        if let Some(parent) = path.parent() {
            return Cow::Owned(parent.to_string_lossy().to_string());
        }
    }
    as_str
}

/// Finds the executable path for a given upscaler kind using platform-aware resolution.
/// Why: Bundled resources and user PATHs vary across OSes and install methods.
pub fn find_upscaler_binary(kind: UpscalerKind) -> Result<UpscalerBinary, PixyError> {
    let name = match kind {
        UpscalerKind::RealEsrgan => "realesrgan-ncnn-vulkan",
        UpscalerKind::RealCugan => "realcugan-ncnn-vulkan",
        UpscalerKind::Waifu2x => "waifu2x-ncnn-vulkan",
    };
    let path = resolve_tool(name)?;
    Ok(UpscalerBinary { kind, path })
}
