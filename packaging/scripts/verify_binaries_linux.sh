#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && cd ../.. && pwd)"
BIN_DIR="$ROOT_DIR/third_party/bin/linux64"

echo "Checking $BIN_DIR"
for exe in ffmpeg ffprobe realesrgan-ncnn-vulkan waifu2x-ncnn-vulkan realcugan-ncnn-vulkan; do
  if [[ ! -x "$BIN_DIR/$exe" ]]; then
    echo "Missing or not executable: $exe" >&2
    exit 1
  fi
done

"$BIN_DIR/ffmpeg" -version | head -n1
"$BIN_DIR/ffprobe" -version | head -n1
"$BIN_DIR/realesrgan-ncnn-vulkan" -h | head -n1 || true
"$BIN_DIR/waifu2x-ncnn-vulkan" -h | head -n1 || true
"$BIN_DIR/realcugan-ncnn-vulkan" -h | head -n1 || true

echo "All binaries present."


