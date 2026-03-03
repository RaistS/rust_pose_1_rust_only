# rust_pose_1_rust_only

Implementacion Rust-only para pipeline de pose, sin runtime Python en produccion.

Estado actual:
- Runtime en Rust con modos `mock` y `camera`.
- Estimador configurable: `mock` o `onnx`.
- Inferencia real YOLO pose ONNX (OpenCV DNN) en modo `--estimator onnx`.
- Ventana en vivo opcional con overlay de keypoints, angulo y FPS.
- Salida opcional NDJSON para analisis offline.

## Ejecutar (mock)

```powershell
cargo run --features camera -- --mode camera --estimator mock --camera-index 0 --show-window
```

## Ejecutar (inferencia real ONNX)

Modelo por defecto: `models/yolov8n-pose.onnx`

```powershell
cargo run --features camera -- --mode camera --estimator onnx --camera-index 0 --camera-width 640 --camera-height 480 --show-window
```

## Perfil rapido recomendado (mas FPS)

```powershell
cargo run --features camera -- --mode camera --estimator onnx --camera-index 0 --camera-width 480 --camera-height 360 --infer-every 2 --show-window --conf-thres 0.15 --kpt-thres 0.10
```

- `--infer-every`: ejecuta inferencia cada N frames y reutiliza ultimo esqueleto en frames intermedios.
- Con `infer-every 2` en esta maquina la ventana subio a ~15 FPS efectivos tras warm-up.

## Nota sobre input-size

- El ONNX incluido (`models/yolov8n-pose.onnx`) esta exportado a `640`, por eso el valor seguro es `--input-size 640`.
- Si usas `320` o `416` con este ONNX concreto, OpenCV DNN puede fallar por `Reshape`.

## CLI principal

- `--mode mock|camera`
- `--estimator mock|onnx`
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

## Notas

- En `estimator=onnx`, el esqueleto y el angulo salen de inferencia real del modelo ONNX.
- Si no detecta persona, no dibuja keypoints y el angulo aparece `n/a`.

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
