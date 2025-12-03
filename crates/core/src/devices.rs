use std::process::Command;

use regex::Regex;

use crate::error::PixyError;
use crate::paths::resolve_tool;

/// A Vulkan device detected by NCNN Vulkan upscaler binaries using `-l`.
#[derive(Debug, Clone)]
pub struct VulkanDevice {
    pub index: usize,
    pub name: String,
}

/// Detects available Vulkan devices by invoking an upscaler binary with `-l`.
/// Why: Device selection is per-upscaler; we parse its own device listing.
pub fn detect_vulkan_devices(binary: &str) -> Result<Vec<VulkanDevice>, PixyError> {
    let bin = resolve_tool(binary).unwrap_or_else(|_| std::path::PathBuf::from(binary));
    let output = Command::new(bin).arg("-l").output()?;
    if !output.status.success() {
        return Ok(Vec::new());
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();
    let re = Regex::new(r"(?m)^(\d+):\s+(.+)$").unwrap();
    for cap in re.captures_iter(&text) {
        let idx: usize = cap[1].parse().unwrap_or(0);
        let name = cap[2].trim().to_string();
        devices.push(VulkanDevice { index: idx, name });
    }
    Ok(devices)
}
