# rust_pose_1_rust_only

Starter Rust-only project to migrate pose processing out of Python.

Current status:
- 100% Rust runtime (no Python dependency).
- Real-time loop with target FPS and FPS reporting.
- Pose domain model + right elbow angle computation.
- Mock estimator that simulates pose keypoints to validate architecture/performance.

## Run

```powershell
cargo run -- --fps-target 30 --report-every 60
```

Optional stop condition:

```powershell
cargo run -- --fps-target 30 --report-every 30 --max-frames 300
```

## Why this starter exists

Before integrating camera + inference engines, this gives you a clean Rust baseline for:
- loop timing,
- serialization cost,
- post-processing cost,
- logging and diagnostics.

## Next steps to make it real vision

1. Camera capture in Rust (`opencv` crate or `nokhwa`).
2. Pose inference in Rust via ONNX Runtime.
3. Decode model output into keypoints.
4. Replace `MockEstimator` with real estimator.

## Suggested architecture

- `src/domain.rs`: frame/keypoint/person structs.
- `src/math.rs`: angle and geometric utilities.
- `src/pipeline.rs`: estimator abstraction and frame builder.
- `src/main.rs`: runtime loop, FPS and reporting.
