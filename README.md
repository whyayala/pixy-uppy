# Pixy-Uppy

Cross-platform Rust CLI + Tauri GUI video upscaler using NCNN/Vulkan upscalers (Real-ESRGAN, Waifu2x, Real-CUGAN) and FFmpeg. Targets AMD/NVIDIA/Intel via Vulkan, preserves audio/subtitles, and exposes encoder controls.

## Status
Work in progress. See `pix.plan.md` for scope.

## Workspace
- `crates/core` – pipeline orchestration and helpers
- `crates/cli` – command line interface
- `apps/desktop` – Tauri desktop GUI
- `third_party/bin` – bundled binaries (per-platform)
- `assets/models` – curated models

## License
MIT
