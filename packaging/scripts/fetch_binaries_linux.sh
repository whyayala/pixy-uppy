#!/usr/bin/env bash
set -euo pipefail

# This script downloads Linux x86_64 binaries for FFmpeg and NCNN Vulkan upscalers
# and places them under third_party/bin/linux64. Adjust versions/URLs as needed.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && cd ../.. && pwd)"
BIN_DIR="$ROOT_DIR/third_party/bin/linux64"
TMP_DIR="$(mktemp -d)"
mkdir -p "$BIN_DIR"

echo "Using BIN_DIR=$BIN_DIR"

# FFmpeg (static build)
FFMPEG_TAR_URL="https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz"
echo "Downloading FFmpeg static build..."
curl -L "$FFMPEG_TAR_URL" -o "$TMP_DIR/ffmpeg.tar.xz"
tar -C "$TMP_DIR" -xf "$TMP_DIR/ffmpeg.tar.xz"
FFMPEG_EXTRACT_DIR="$(find "$TMP_DIR" -maxdepth 1 -type d -name 'ffmpeg-*' | head -n1)"
install -m 0755 "$FFMPEG_EXTRACT_DIR/ffmpeg" "$BIN_DIR/ffmpeg"
install -m 0755 "$FFMPEG_EXTRACT_DIR/ffprobe" "$BIN_DIR/ffprobe"

# Real-ESRGAN NCNN Vulkan
# Pick a suitable asset from releases if this  URL changes
ESRGAN_ZIP_URL="https://github.com/xinntao/Real-ESRGAN-ncnn-vulkan/releases/latest/download/realesrgan-ncnn-vulkan-20220424-ubuntu.zip"
echo "Downloading Real-ESRGAN NCNN Vulkan..."
curl -L "$ESRGAN_ZIP_URL" -o "$TMP_DIR/realesrgan.zip"
unzip -q "$TMP_DIR/realesrgan.zip" -d "$TMP_DIR/realesrgan"
install -m 0755 "$TMP_DIR/realesrgan"/*/realesrgan-ncnn-vulkan "$BIN_DIR/realesrgan-ncnn-vulkan" || install -m 0755 "$TMP_DIR/realesrgan"/realesrgan-ncnn-vulkan "$BIN_DIR/realesrgan-ncnn-vulkan"

# Waifu2x NCNN Vulkan
WAIFU2X_ZIP_URL="https://github.com/nihui/waifu2x-ncnn-vulkan/releases/latest/download/waifu2x-ncnn-vulkan-20220728-ubuntu.zip"
echo "Downloading Waifu2x NCNN Vulkan..."
curl -L "$WAIFU2X_ZIP_URL" -o "$TMP_DIR/waifu2x.zip"
unzip -q "$TMP_DIR/waifu2x.zip" -d "$TMP_DIR/waifu2x"
install -m 0755 "$TMP_DIR/waifu2x"/*/waifu2x-ncnn-vulkan "$BIN_DIR/waifu2x-ncnn-vulkan" || install -m 0755 "$TMP_DIR/waifu2x"/waifu2x-ncnn-vulkan "$BIN_DIR/waifu2x-ncnn-vulkan"

# Real-CUGAN NCNN Vulkan
REALCUGAN_ZIP_URL="https://github.com/nihui/realcugan-ncnn-vulkan/releases/latest/download/realcugan-ncnn-vulkan-20220728-ubuntu.zip"
echo "Downloading Real-CUGAN NCNN Vulkan..."
curl -L "$REALCUGAN_ZIP_URL" -o "$TMP_DIR/realcugan.zip"
unzip -q "$TMP_DIR/realcugan.zip" -d "$TMP_DIR/realcugan"
install -m 0755 "$TMP_DIR/realcugan"/*/realcugan-ncnn-vulkan "$BIN_DIR/realcugan-ncnn-vulkan" || install -m 0755 "$TMP_DIR/realcugan"/realcugan-ncnn-vulkan "$BIN_DIR/realcugan-ncnn-vulkan"

echo "Making sure binaries are executable..."
chmod +x "$BIN_DIR/ffmpeg" "$BIN_DIR/ffprobe" "$BIN_DIR/realesrgan-ncnn-vulkan" "$BIN_DIR/waifu2x-ncnn-vulkan" "$BIN_DIR/realcugan-ncnn-vulkan"

echo "Done. Binaries installed to $BIN_DIR"


