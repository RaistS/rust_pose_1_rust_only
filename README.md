# rust_pose_1_rust_only

Implementacion Rust-only para pipeline de pose, sin runtime Python.

Estado actual:
- Runtime en Rust con modos `mock` y `camera`.
- Medicion de FPS por ventana de N frames.
- Calculo de angulo de codo derecho desde keypoints.
- Salida opcional a archivo NDJSON para analisis offline.
- Arquitectura modular lista para conectar inferencia ONNX real.

## Ejecutar (mock)

```powershell
cargo run -- --mode mock --fps-target 60 --report-every 120
```

Con limite de frames:

```powershell
cargo run -- --mode mock --max-frames 300
```

## Ejecutar (camera real)

Modo camera requiere compilar con feature `camera` (OpenCV):

```powershell
cargo run --features camera -- --mode camera --camera-index 0 --camera-width 640 --camera-height 480
```

## Guardar salida NDJSON

```powershell
cargo run -- --mode mock --out-ndjson .\out_pose.ndjson --max-frames 1000
```

Cada linea del archivo es un `PoseFrame` JSON.

## CLI principal

- `--mode mock|camera`
- `--fps-target <u32>`
- `--report-every <u32>`
- `--max-frames <u64>` (0 = infinito)
- `--camera-index <i32>`
- `--camera-width <i32>`
- `--camera-height <i32>`
- `--out-ndjson <PATH>`
- `--source-tag <STRING>`

## Siguiente fase (inferencia real)

1. Exportar modelo pose a ONNX una sola vez (fuera de runtime).
2. Crear `OnnxEstimator` que implemente `PoseEstimator`.
3. Decodificar tensor de salida a 17 keypoints COCO.
4. Sustituir `MockEstimator` por `OnnxEstimator` en `main.rs`.

## Estructura

- `src/main.rs`: runtime, CLI, control de modos y FPS.
- `src/camera.rs`: captura de camara (feature-gated).
- `src/pipeline.rs`: contratos del pipeline y frame builder.
- `src/domain.rs`: structs serializables de salida.
- `src/math.rs`: utilidades geometricas.
- `src/metrics.rs`: ventana de FPS.
- `src/output.rs`: sink NDJSON.
