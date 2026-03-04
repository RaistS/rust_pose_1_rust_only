# rust_pose_1_rust_only

Implementacion Rust-only para pipeline de pose, sin runtime Python en produccion.

## Estado actual

- Runtime en Rust con modos `mock` y `camera`.
- Estimador configurable: `mock` o `onnx`.
- Inferencia real YOLO pose ONNX (OpenCV DNN) con `--estimator onnx`.
- Ventana en vivo opcional con overlay de keypoints, angulo y FPS.
- Salida opcional NDJSON para analisis offline.

## Rama dev (GPU experimental)

La rama `dev` incluye selector de backend/target DNN:

- `--dnn-backend opencv|cuda`
- `--dnn-target cpu|cuda|cuda-fp16`

Si backend/target no esta disponible, hay fallback automatico a CPU con warning.

## Requisitos para CUDA en este entorno

Este proyecto asume este layout local (ajustado a tu maquina actual):

- OpenCV compilado con CUDA en `C:\opencv-cuda\build`
- cuDNN en `C:\opencv-cuda\deps\cudnn-windows-x86_64-9.19.1.2_cuda13-archive\bin\x64`
- CUDA Toolkit 13.1 en `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.1\bin\x64`
- LLVM en `C:\Program Files\LLVM\bin`

## Ejecucion recomendada (sin recompilar)

Si ya tienes binario compilado, ejecuta el `.exe` directo (evita reprobes fragiles del crate `opencv`):

```powershell
cd E:\Programacion\RUST\rust_pose_1_rust_only

$basePath = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
$env:Path = "C:\opencv-cuda\deps\cudnn-windows-x86_64-9.19.1.2_cuda13-archive\bin\x64;C:\opencv-cuda\build\bin\Release;C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.1\bin\x64;C:\Program Files\LLVM\bin;$basePath"

.\target\debug\rust_pose_1_rust_only.exe --mode camera --estimator onnx --dnn-backend cuda --dnn-target cuda-fp16 --camera-index 0 --show-window
```

## Lanzadores rapidos (PowerShell)

- `./run_cuda.ps1`: ejecuta `target\debug\rust_pose_1_rust_only.exe`
- `./run_cuda_release.ps1`: ejecuta `target\release\rust_pose_1_rust_only.exe`

Uso:

```powershell
.\run_cuda.ps1
.\run_cuda_release.ps1
```

Con parametros:

```powershell
.\run_cuda.ps1 -CameraIndex 0 -DnnTarget cuda-fp16 -ShowWindow
```

Si no existe el binario, compila una vez:

```powershell
cargo build --features camera
cargo build --release --features camera
```

## Ejecucion via cargo (solo si entorno estable)

CPU:

```powershell
cargo run --features camera -- --mode camera --estimator onnx --dnn-backend opencv --dnn-target cpu --camera-index 0 --show-window
```

CUDA:

```powershell
cargo run --features camera -- --mode camera --estimator onnx --dnn-backend cuda --dnn-target cuda-fp16 --camera-index 0 --show-window
```

## Perfil rapido (CPU)

```powershell
cargo run --features camera -- --mode camera --estimator onnx --dnn-backend opencv --dnn-target cpu --camera-index 0 --camera-width 480 --camera-height 360 --infer-every 2 --show-window --conf-thres 0.15 --kpt-thres 0.10
```

## Nota sobre input-size

- El ONNX incluido (`models/yolov8n-pose.onnx`) esta exportado a `640`.
- Usa `--input-size 640` para evitar errores de `Reshape` en OpenCV DNN.

## CLI principal

- `--mode mock|camera`
- `--estimator mock|onnx`
- `--dnn-backend opencv|cuda`
- `--dnn-target cpu|cuda|cuda-fp16`
- `--fps-target <u32>`
- `--report-every <u32>`
- `--max-frames <u64>`
- `--camera-index <i32>`
- `--camera-width <i32>`
- `--camera-height <i32>`
- `--show-window`
- `--model-path <PATH>`
- `--conf-thres <f32>`
- `--kpt-thres <f32>`
- `--input-size <i32>`
- `--infer-every <u32>`
- `--out-ndjson <PATH>`
- `--source-tag <STRING>`

## Estructura

- `src/main.rs`: runtime y CLI.
- `src/onnx_pose.rs`: inferencia ONNX + decode de keypoints.
- `src/camera.rs`: captura de camara.
- `src/render.rs`: overlay y ventana en vivo.
- `src/pipeline.rs`: contratos de frame y mock estimator.
- `src/domain.rs`: modelos serializables.
- `src/math.rs`: calculo angular.
- `src/metrics.rs`: FPS window.
- `src/output.rs`: NDJSON sink.
