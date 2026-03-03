# rust_pose_1_rust_only

Implementacion Rust-only para pipeline de pose, sin runtime Python en produccion.

Estado actual:
- Runtime en Rust con modos `mock` y `camera`.
- Estimador configurable: `mock` o `onnx`.
- Inferencia real YOLO pose ONNX (OpenCV DNN) en modo `--estimator onnx`.
- Ventana en vivo opcional con overlay de keypoints, angulo y FPS.
- Salida opcional NDJSON para analisis offline.

## Rama dev (GPU experimental)

La rama `dev` incluye selector de backend/target DNN para probar GPU:
- `--dnn-backend opencv|cuda`
- `--dnn-target cpu|cuda|cuda-fp16`

Si el backend/target no esta disponible, hace fallback automatico a CPU con warning.

## Ejecutar (inferencia real ONNX CPU)

```powershell
cargo run --features camera -- --mode camera --estimator onnx --dnn-backend opencv --dnn-target cpu --camera-index 0 --show-window
```

## Ejecutar (intento GPU CUDA)

```powershell
cargo run --features camera -- --mode camera --estimator onnx --dnn-backend cuda --dnn-target cuda-fp16 --camera-index 0 --show-window
```

## Perfil rapido recomendado (mas FPS en CPU)

```powershell
cargo run --features camera -- --mode camera --estimator onnx --dnn-backend opencv --dnn-target cpu --camera-index 0 --camera-width 480 --camera-height 360 --infer-every 2 --show-window --conf-thres 0.15 --kpt-thres 0.10
```

## Nota sobre input-size

- El ONNX incluido (`models/yolov8n-pose.onnx`) esta exportado a `640`, por eso el valor seguro es `--input-size 640`.
- Si usas `320` o `416` con este ONNX concreto, OpenCV DNN puede fallar por `Reshape`.

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
