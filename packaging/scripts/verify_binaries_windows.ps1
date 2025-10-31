Param()
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$Root = (Resolve-Path (Join-Path $PSScriptRoot '..' '..')).Path
$Bin = Join-Path $Root 'third_party/bin/win64'
Write-Host "Checking $Bin"

$expected = @('ffmpeg.exe','ffprobe.exe','realesrgan-ncnn-vulkan.exe','waifu2x-ncnn-vulkan.exe','realcugan-ncnn-vulkan.exe')
foreach ($e in $expected) {
  $p = Join-Path $Bin $e
  if (!(Test-Path $p)) { throw "Missing: $e" }
}

& (Join-Path $Bin 'ffmpeg.exe') -version | Select-Object -First 1
& (Join-Path $Bin 'ffprobe.exe') -version | Select-Object -First 1
& (Join-Path $Bin 'realesrgan-ncnn-vulkan.exe') -h | Select-Object -First 1
& (Join-Path $Bin 'waifu2x-ncnn-vulkan.exe') -h | Select-Object -First 1
& (Join-Path $Bin 'realcugan-ncnn-vulkan.exe') -h | Select-Object -First 1

Write-Host "All binaries present."


