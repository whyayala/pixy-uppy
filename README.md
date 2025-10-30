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

## Requirements
- Windows 10+ or Linux (x86_64)
- Rust (stable) with `rustfmt` and `clippy`
- Node.js 20+ for GUI
- Vulkan-capable GPU and drivers (AMD/NVIDIA/Intel)

## Binaries and Models
Pixy-Uppy looks for tools in this order:
1. System `PATH`
2. `PIXY_UPPY_BIN_DIR` environment variable
3. `third_party/bin/<win64|linux64>/` relative to CWD or executable

Place the following executables in `third_party/bin/<platform>/` or ensure they’re on `PATH`:
- `ffmpeg`, `ffprobe`
- `realesrgan-ncnn-vulkan`, `waifu2x-ncnn-vulkan`, `realcugan-ncnn-vulkan`

Curated models may be shipped under `assets/models/` (or use models bundled with the binaries).

## Build (CLI)
```
cargo build -p pixy_cli --release
```

Run examples:
```
./target/release/pixy-uppy devices
./target/release/pixy-uppy models
./target/release/pixy-uppy upscale \
  -i input.mkv -o output.mkv -m general_x4v3 \
  --gpu 0 --encoder hevc_nvenc --preset p4 --crf 20
```

Common flags:
- `--model` one of `pixy-uppy models`
- `--scale` or `--width/--height` for target resolution
- `--prefilter` `yadif|hqdn3d|deband|none`
- `--encoder` `h264_nvenc|hevc_nvenc|h264_amf|hevc_amf|h264_qsv|hevc_qsv|h264_vaapi|hevc_vaapi|libx264|libx265`
- `--preset`, `--tune`, `--crf`

## Build (GUI)
Install JS deps, then build and run the Tauri app:
```
cd apps/desktop
npm install
npm run build
cd src-tauri
cargo tauri dev
```

Or package installers:
```
cd apps/desktop/src-tauri
cargo tauri build
```

Note: The GUI delegates to the same core pipeline; ensure binaries are discoverable as above.

## License
MIT
