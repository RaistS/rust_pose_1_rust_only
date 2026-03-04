param(
    [int]$CameraIndex = 0,
    [switch]$ShowWindow = $true,
    [string]$DnnTarget = "cuda-fp16"
)

$ErrorActionPreference = "Stop"

$basePath = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
$env:Path = "C:\opencv-cuda\deps\cudnn-windows-x86_64-9.19.1.2_cuda13-archive\bin\x64;C:\opencv-cuda\build\bin\Release;C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.1\bin\x64;C:\Program Files\LLVM\bin;$basePath"

# Avoid inheriting stale OpenCV build vars that can trigger fragile re-probes
Remove-Item Env:OPENCV_INCLUDE_PATHS -ErrorAction SilentlyContinue
Remove-Item Env:OPENCV_LINK_PATHS -ErrorAction SilentlyContinue
Remove-Item Env:OPENCV_LINK_LIBS -ErrorAction SilentlyContinue
Remove-Item Env:OPENCV_DISABLE_PROBES -ErrorAction SilentlyContinue
Remove-Item Env:OpenCV_DIR -ErrorAction SilentlyContinue

$exe = Join-Path $PSScriptRoot "target\release\rust_pose_1_rust_only.exe"
if (-not (Test-Path $exe)) {
    throw "No existe $exe. Compila primero una vez con: cargo build --release --features camera"
}

$args = @(
    "--mode", "camera",
    "--estimator", "onnx",
    "--dnn-backend", "cuda",
    "--dnn-target", $DnnTarget,
    "--camera-index", "$CameraIndex"
)

if ($ShowWindow) {
    $args += "--show-window"
}

Write-Host "Running: $exe $($args -join ' ')" -ForegroundColor Cyan
& $exe @args
