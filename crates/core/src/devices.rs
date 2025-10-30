use std::process::Command;

use regex::Regex;

use crate::error::PixyError;

#[derive(Debug, Clone)]
pub struct VulkanDevice {
    pub index: usize,
    pub name: String,
}

pub fn detect_vulkan_devices(binary: &str) -> Result<Vec<VulkanDevice>, PixyError> {
    let output = Command::new(binary).arg("-l").output()?;
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


