use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::PixyError;
use crate::models::ModelSpec;

#[derive(Debug, Clone, Copy)]
pub enum UpscalerKind {
    RealEsrgan,
    RealCugan,
    Waifu2x,
}

#[derive(Debug, Clone)]
pub struct UpscalerBinary {
    pub kind: UpscalerKind,
    pub path: PathBuf,
}

impl UpscalerBinary {
    pub fn run(&self, input_pattern: &Path, output_pattern: &Path, gpu: usize, tile_size: Option<u32>, threads: Option<u32>, model: &ModelSpec) -> Result<(), PixyError> {
        let mut cmd = match self.kind {
            UpscalerKind::RealEsrgan => {
                let mut c = Command::new(&self.path);
                c.args(["-i", input_pattern.to_string_lossy().as_ref(), "-o", output_pattern.to_string_lossy().as_ref(), "-n", &model.name, "-g"]).arg(gpu.to_string());
                if let Some(t) = tile_size { c.args(["-t", &t.to_string()]); }
                if let Some(th) = threads { c.args(["-j", &format!("{}:{}:{}", th, th, th)]); }
                c
            }
            UpscalerKind::RealCugan => {
                let mut c = Command::new(&self.path);
                c.args(["-i", input_pattern.to_string_lossy().as_ref(), "-o", output_pattern.to_string_lossy().as_ref(), "-g"]).arg(gpu.to_string());
                if let Some(t) = tile_size { c.args(["-t", &t.to_string()]); }
                if let Some(level) = model.denoise_level { c.args(["-n", &level.to_string()]); }
                c
            }
            UpscalerKind::Waifu2x => {
                let mut c = Command::new(&self.path);
                c.args(["-i", input_pattern.to_string_lossy().as_ref(), "-o", output_pattern.to_string_lossy().as_ref(), "-g"]).arg(gpu.to_string());
                if let Some(t) = tile_size { c.args(["-t", &t.to_string()]); }
                if let Some(level) = model.denoise_level { c.args(["-n", &level.to_string()]); }
                c
            }
        };

        let status = cmd.status()?;
        if !status.success() {
            return Err(PixyError::ProcessFailed { cmd: format!("{:?}", cmd), code: status.code(), stderr: String::new() });
        }
        Ok(())
    }
}


