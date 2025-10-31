Param()
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Downloads Windows x64 binaries for FFmpeg and NCNN Vulkan upscalers
# and places them under third_party/bin/win64. Adjust versions/URLs as needed.

$Root = (Resolve-Path (Join-Path $PSScriptRoot '..' '..')).Path
$Bin = Join-Path $Root 'third_party/bin/win64'
New-Item -ItemType Directory -Force -Path $Bin | Out-Null

Write-Host "Using BIN_DIR=$Bin"

# FFmpeg (essentials build)
$ffmpegZip = Join-Path $env:TEMP 'ffmpeg.zip'
$ffmpegUrl = 'https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip'
Write-Host 'Downloading FFmpeg...'
Invoke-WebRequest -Uri $ffmpegUrl -OutFile $ffmpegZip
Expand-Archive -Path $ffmpegZip -DestinationPath $env:TEMP -Force
$ffmpegDir = Get-ChildItem -Directory -Path $env:TEMP | Where-Object { $_.Name -like 'ffmpeg-*' } | Select-Object -First 1
Copy-Item (Join-Path $ffmpegDir.FullName 'bin/ffmpeg.exe') $Bin -Force
Copy-Item (Join-Path $ffmpegDir.FullName 'bin/ffprobe.exe') $Bin -Force

# Real-ESRGAN NCNN Vulkan
$esrZip = Join-Path $env:TEMP 'realesrgan.zip'
$esrUrl = 'https://github.com/xinntao/Real-ESRGAN-ncnn-vulkan/releases/latest/download/realesrgan-ncnn-vulkan-20220424-windows.zip'
Write-Host 'Downloading Real-ESRGAN...'
Invoke-WebRequest -Uri $esrUrl -OutFile $esrZip
Expand-Archive -Path $esrZip -DestinationPath $env:TEMP -Force
$esrExe = Get-ChildItem -Recurse -Path $env:TEMP -Filter 'realesrgan-ncnn-vulkan.exe' | Select-Object -First 1
Copy-Item $esrExe.FullName (Join-Path $Bin 'realesrgan-ncnn-vulkan.exe') -Force

# Waifu2x NCNN Vulkan
$w2xZip = Join-Path $env:TEMP 'waifu2x.zip'
$w2xUrl = 'https://github.com/nihui/waifu2x-ncnn-vulkan/releases/latest/download/waifu2x-ncnn-vulkan-20220728-windows.zip'
Write-Host 'Downloading Waifu2x...'
Invoke-WebRequest -Uri $w2xUrl -OutFile $w2xZip
Expand-Archive -Path $w2xZip -DestinationPath $env:TEMP -Force
$w2xExe = Get-ChildItem -Recurse -Path $env:TEMP -Filter 'waifu2x-ncnn-vulkan.exe' | Select-Object -First 1
Copy-Item $w2xExe.FullName (Join-Path $Bin 'waifu2x-ncnn-vulkan.exe') -Force

# Real-CUGAN NCNN Vulkan
$rcgZip = Join-Path $env:TEMP 'realcugan.zip'
$rcgUrl = 'https://github.com/nihui/realcugan-ncnn-vulkan/releases/latest/download/realcugan-ncnn-vulkan-20220728-windows.zip'
Write-Host 'Downloading Real-CUGAN...'
Invoke-WebRequest -Uri $rcgUrl -OutFile $rcgZip
Expand-Archive -Path $rcgZip -DestinationPath $env:TEMP -Force
$rcgExe = Get-ChildItem -Recurse -Path $env:TEMP -Filter 'realcugan-ncnn-vulkan.exe' | Select-Object -First 1
Copy-Item $rcgExe.FullName (Join-Path $Bin 'realcugan-ncnn-vulkan.exe') -Force

Write-Host "Done. Binaries installed to $Bin"


