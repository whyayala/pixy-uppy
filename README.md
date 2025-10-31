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

Quick setup using helper scripts:
- Linux: `bash packaging/scripts/fetch_binaries_linux.sh && bash packaging/scripts/verify_binaries_linux.sh`
- Windows (PowerShell): `./packaging/scripts/fetch_binaries_windows.ps1; ./packaging/scripts/verify_binaries_windows.ps1`

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

## Testing end-to-end

1) Smoke check: tools and devices
```
./target/release/pixy-uppy devices
./third_party/bin/<platform>/ffmpeg -version
./third_party/bin/<platform>/realesrgan-ncnn-vulkan -h
```

2) Generate a short test clip (uses ffmpeg)
- Windows (PowerShell):
```
./third_party/bin/win64/ffmpeg.exe -y -f lavfi -i testsrc2=size=640x360:rate=24 -t 5 testsrc_360p.mp4
```
- Linux:
```
third_party/bin/linux64/ffmpeg -y -f lavfi -i testsrc2=size=640x360:rate=24 -t 5 testsrc_360p.mp4
```

3) Upscale with Real-ESRGAN
```
./target/release/pixy-uppy upscale -i testsrc_360p.mp4 -o testsrc_1440p.mkv -m general_x4v3 --gpu 0 --encoder hevc_nvenc --preset p4 --crf 20
```

4) Verify streams and resolution
```
ffprobe -v error -select_streams v:0 -show_entries stream=width,height,avg_frame_rate,pix_fmt -of json testsrc_1440p.mkv
ffprobe -v error -select_streams a? -show_entries stream=index,codec_name -of json testsrc_1440p.mkv
```

5) Spot-check quality (visual)
- Extract a frame before/after and compare:
```
ffmpeg -y -i testsrc_360p.mp4 -vframes 1 src.png
ffmpeg -y -i testsrc_1440p.mkv -vframes 1 up.png
```

6) Approximate objective check
- Downscale the upscaled output back to source size and compute SSIM/PSNR (higher is closer):
```
ffmpeg -i testsrc_1440p.mkv -vf scale=640:360:flags=spline -an ds.mkv
ffmpeg -i testsrc_360p.mp4 -i ds.mkv -lavfi "ssim;[0:v][1:v]psnr" -f null -
```

7) Real content test
- Try a short scene (10–30s) from a live-action and an animation source. Adjust `--tile-size` and `--threads` if VRAM errors occur. For 1080p→4K, either `-m general_x4v3` with post-scale, or a `x2` model twice.

Acceptance checklist
- Output resolution matches requested (via ffprobe)
- Audio/subtitle streams are copied (stream counts match input)
- Encoding plays in VLC/MPV and GPU usage spikes during upscaling
- Visual inspection shows improved detail without excessive artifacts

## License
MIT
