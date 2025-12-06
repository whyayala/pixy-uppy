# Fetching Binaries (Windows/Linux)

Pixy-Uppy expects the following executables to be available:

- ffmpeg, ffprobe
- realesrgan-ncnn-vulkan
- waifu2x-ncnn-vulkan
- realcugan-ncnn-vulkan

Resolution order:
1. PATH
2. PIXY_UPPY_BIN_DIR environment variable
3. third_party/bin/<win64|linux64>/ relative to CWD or executable

## Windows (x64)

1) FFmpeg
- Download a static build from `https://www.gyan.dev/ffmpeg/builds/` (ffmpeg-release-essentials.zip)
- Copy `ffmpeg.exe` and `ffprobe.exe` to `third_party/bin/win64/`

2) Real-ESRGAN (NCNN Vulkan)
- Releases: `https://github.com/xinntao/Real-ESRGAN-ncnn-vulkan/releases`
- Extract `realesrgan-ncnn-vulkan.exe` to `third_party/bin/win64/`

3) Waifu2x (NCNN Vulkan)
- Releases: `https://github.com/nihui/waifu2x-ncnn-vulkan/releases`
- Extract `waifu2x-ncnn-vulkan.exe` to `third_party/bin/win64/`

4) Real-CUGAN (NCNN Vulkan)
- Releases: `https://github.com/nihui/realcugan-ncnn-vulkan/releases`
- Extract `realcugan-ncnn-vulkan.exe` to `third_party/bin/win64/`

5) (Optional) Models
- Many NCNN binaries include default models; if you have separate `.bin/.param` models, keep them alongside the exe or in `assets/models/`.

Verify:
- In PowerShell: `Get-ChildItem third_party/bin/win64`
- Check versions: `third_party\bin\win64\ffmpeg.exe -version`

## Linux (x86_64)

1) FFmpeg
- Use distro packages or static build (e.g., `https://johnvansickle.com/ffmpeg/`)
- Place `ffmpeg` and `ffprobe` into `third_party/bin/linux64/` or ensure on PATH

2) Real-ESRGAN (portable NCNN Vulkan bundle)
- Download `realesrgan-ncnn-vulkan-20220424-ubuntu.zip` from the Real-ESRGAN v0.2.5.0 release: [portable binary](https://github.com/xinntao/Real-ESRGAN/releases/download/v0.2.5.0/realesrgan-ncnn-vulkan-20220424-ubuntu.zip)
- Extract `realesrgan-ncnn-vulkan` to `third_party/bin/linux64/` and `chmod +x` it
- Copy the bundled `models/` directory into `third_party/bin/linux64/models/` so the `.param/.bin` files sit next to the binary (the `fetch_binaries_linux.sh` script performs this automatically)

3) Waifu2x / Real-CUGAN (NCNN Vulkan)
- Releases: `https://github.com/nihui/waifu2x-ncnn-vulkan/releases`
- Releases: `https://github.com/nihui/realcugan-ncnn-vulkan/releases`
- `chmod +x` all binaries

4) Vulkan runtime/drivers
- Install Vulkan drivers for your GPU vendor (mesa-vulkan-drivers, nvidia-driver, amdgpu-pro as applicable)

Verify:
- `third_party/bin/linux64/ffmpeg -version`
- `third_party/bin/linux64/realesrgan-ncnn-vulkan -h`

## Checksums

- Windows (PowerShell): `Get-FileHash .\third_party\bin\win64\realesrgan-ncnn-vulkan.exe -Algorithm SHA256`
- Linux/macOS: `shasum -a 256 third_party/bin/linux64/realesrgan-ncnn-vulkan`

## Environment Override

To store binaries elsewhere, set:
- Windows: `$env:PIXY_UPPY_BIN_DIR = "C:\\tools\\pixy-bins"`
- Linux: `export PIXY_UPPY_BIN_DIR=$HOME/tools/pixy-bins`

Ensure the directory contains the expected executable names.
