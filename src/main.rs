mod camera;
mod domain;
mod math;
mod metrics;
mod output;
mod pipeline;
mod render;

use anyhow::Context;
use camera::CameraCapture;
use clap::{Parser, ValueEnum};
use metrics::FpsWindow;
use output::FrameSink;
use pipeline::{build_frame, now_ms, MockEstimator, PipelineInput, PoseEstimator};
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    Mock,
    Camera,
}

#[derive(Debug, Parser)]
#[command(author, version, about = "Rust-only pose pipeline runtime")]
struct Cli {
    #[arg(long, value_enum, default_value_t = Mode::Mock)]
    mode: Mode,

    #[arg(long, default_value_t = 30)]
    fps_target: u32,

    #[arg(long, default_value_t = 60)]
    report_every: u32,

    #[arg(long, default_value_t = 0)]
    max_frames: u64,

    #[arg(long, default_value_t = 0)]
    camera_index: i32,

    #[arg(long, default_value_t = 640)]
    camera_width: i32,

    #[arg(long, default_value_t = 480)]
    camera_height: i32,

    #[arg(long)]
    out_ndjson: Option<PathBuf>,

    #[arg(long, default_value = "rust-only-mock")]
    source_tag: String,

    #[arg(long, default_value_t = false)]
    show_window: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let report_every = cli.report_every.max(1) as u64;
    let frame_budget = Duration::from_secs_f64(1.0 / (cli.fps_target.max(1) as f64));

    let mut sink = FrameSink::new(cli.out_ndjson.as_deref())?;
    let mut estimator = MockEstimator::new();

    let mut fps_window = FpsWindow::new(report_every);
    let mut frame_id = 0u64;
    #[cfg(feature = "camera")]
    let mut last_fps = None;

    println!(
        "Runtime start mode={:?} fps_target={} report_every={} source_tag={}",
        cli.mode, cli.fps_target, cli.report_every, cli.source_tag
    );
    println!(
        "Estimator activo: MOCK (angulos sinteticos desde keypoints simulados, no inferencia ONNX todavia)."
    );

    let mut camera = match cli.mode {
        Mode::Mock => None,
        Mode::Camera => Some(
            CameraCapture::open(cli.camera_index, cli.camera_width, cli.camera_height)
                .context("No se pudo iniciar modo camera")?,
        ),
    };

    loop {
        let loop_start = Instant::now();

        #[cfg(feature = "camera")]
        let mut maybe_image = None;

        let input = match camera.as_mut() {
            Some(camera) => {
                let frame = camera.next_frame().context("Fallo leyendo frame de camara")?;

                #[cfg(feature = "camera")]
                {
                    maybe_image = Some(frame.mat);
                }

                PipelineInput {
                    frame_id,
                    ts_ms: frame.ts_ms,
                    image_width: frame.width,
                    image_height: frame.height,
                }
            }
            None => PipelineInput {
                frame_id,
                ts_ms: now_ms(),
                image_width: cli.camera_width.max(1) as u32,
                image_height: cli.camera_height.max(1) as u32,
            },
        };

        let people = estimator.estimate(&input);
        let frame = build_frame(&cli.source_tag, &input, people);
        let json_bytes = sink.write(&frame)?;

        if frame_id % report_every == 0 {
            let angle = frame
                .people
                .first()
                .and_then(|p| p.right_elbow_deg)
                .map(|v| format!("{v:.2}"))
                .unwrap_or_else(|| "None".to_string());

            println!(
                "frame={} angle={} size={}x{} json_bytes={} estimator=mock",
                frame.frame_id,
                angle,
                frame.image_width,
                frame.image_height,
                json_bytes
            );
        }

        #[cfg(feature = "camera")]
        if cli.show_window {
            if let Some(mut image) = maybe_image {
                let should_exit = render::draw_and_show("rust_pose_1_rust_only", &mut image, &frame, last_fps)?;
                if should_exit {
                    break;
                }
            }
        }

        if let Some((fps, frames, elapsed)) = fps_window.tick() {
            println!(
                "[FPS] {:.2} (window={} frames, {:.2}s)",
                fps,
                frames,
                elapsed.as_secs_f64()
            );

            #[cfg(feature = "camera")]
            {
                last_fps = Some(fps);
            }
        }

        frame_id += 1;
        if cli.max_frames > 0 && frame_id >= cli.max_frames {
            break;
        }

        let elapsed = loop_start.elapsed();
        if elapsed < frame_budget {
            thread::sleep(frame_budget - elapsed);
        }
    }

    Ok(())
}
