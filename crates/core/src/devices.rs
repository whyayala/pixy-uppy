use std::ffi::{CStr, CString};

use ash::{vk, Entry, LoadingError};

use crate::error::PixyError;

/// A Vulkan device detected via the Vulkan API.
#[derive(Debug, Clone)]
pub struct VulkanDevice {
    pub index: usize,
    pub name: String,
}

/// Detects available Vulkan devices using the Vulkan loader. When the loader is
/// missing or errors, a warning is emitted and an empty list is returned.
pub fn detect_vulkan_devices() -> Result<Vec<VulkanDevice>, PixyError> {
    match detect_vulkan_devices_native() {
        Ok(devices) => Ok(devices),
        Err(NativeDetectError::LoaderUnavailable(reason)) => {
            eprintln!(
                "warning: Vulkan loader not available ({reason}). Install the Vulkan runtime to enable device detection."
            );
            Ok(Vec::new())
        }
        Err(NativeDetectError::Other(reason)) => {
            eprintln!("warning: Native Vulkan device enumeration failed ({reason}).");
            Ok(Vec::new())
        }
    }
}

fn detect_vulkan_devices_native() -> Result<Vec<VulkanDevice>, NativeDetectError> {
    let entry = unsafe {
        Entry::load().map_err(|err| match err {
            LoadingError::LibraryLoadFailure(e) => {
                NativeDetectError::LoaderUnavailable(e.to_string())
            }
            LoadingError::MissingEntryPoint(e) => {
                NativeDetectError::Other(format!("missing entry point: {e}"))
            }
        })?
    };

    let app_name = CString::new("pixy-uppy").expect("static string has no interior nulls");
    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(0)
        .engine_name(&app_name)
        .engine_version(0)
        .api_version(vk::API_VERSION_1_0);

    #[allow(unused_mut)]
    let mut extension_names: Vec<*const i8> = Vec::new();
    #[cfg(target_os = "macos")]
    {
        extension_names.push(vk::KhrPortabilityEnumerationFn::name().as_ptr());
    }

    #[allow(unused_mut)]
    let mut flags = vk::InstanceCreateFlags::empty();
    #[cfg(target_os = "macos")]
    {
        flags |= vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
    }

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names)
        .flags(flags);

    let instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .map_err(|err| NativeDetectError::Other(format!("create instance: {err}")))?
    };

    let devices = unsafe {
        match instance.enumerate_physical_devices() {
            Ok(handles) => {
                let list = handles
                    .iter()
                    .enumerate()
                    .map(|(idx, physical)| {
                        let props = instance.get_physical_device_properties(*physical);
                        let name = CStr::from_ptr(props.device_name.as_ptr())
                            .to_string_lossy()
                            .trim()
                            .to_string();
                        VulkanDevice { index: idx, name }
                    })
                    .collect::<Vec<_>>();
                Ok(list)
            }
            Err(err) => Err(NativeDetectError::Other(format!(
                "enumerate physical devices: {err}"
            ))),
        }
    };

    unsafe {
        instance.destroy_instance(None);
    }

    devices
}

#[derive(Debug)]
enum NativeDetectError {
    LoaderUnavailable(String),
    Other(String),
}
