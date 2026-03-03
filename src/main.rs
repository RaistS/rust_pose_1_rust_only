mod domain;
mod math;
mod pipeline;

use anyhow::Context;
use clap::Parser;
use pipeline::{build_frame, MockEstimator, PoseEstimator};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Parser)]
#[command(author, version, about = "Rust-only pose pipeline starter")]
struct Cli {
    #[arg(long, default_value_t = 30)]
    fps_target: u32,
    #[arg(long, default_value_t = 60)]
    report_every: u32,
    #[arg(long, default_value_t = 0)]
    max_frames: u64,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let frame_budget = Duration::from_secs_f64(1.0 / (cli.fps_target.max(1) as f64));
    let report_every = cli.report_every.max(1) as u64;

    let mut estimator = MockEstimator::new();
    let mut frame_id = 0u64;

    let mut fps_count = 0u64;
    let mut fps_start = Instant::now();

    println!(
        "Rust-only pipeline running (mock mode). target_fps={} report_every={}",
        cli.fps_target, cli.report_every
    );

    loop {
        let tick_start = Instant::now();

        let people = estimator.estimate(frame_id);
        let frame = build_frame(frame_id, people);

        if frame_id % report_every == 0 {
            let first_angle = frame
                .people
                .first()
                .and_then(|p| p.right_elbow_deg)
                .map(|v| format!("{v:.2}"))
                .unwrap_or_else(|| "None".to_string());

            let json_preview = serde_json::to_string(&frame)
                .context("No se pudo serializar PoseFrame")?;
            println!(
                "frame={} angle={} json_bytes={}",
                frame.frame_id,
                first_angle,
                json_preview.len()
            );
        }

        fps_count += 1;
        if fps_count >= report_every {
            let elapsed = fps_start.elapsed().as_secs_f64().max(1e-9);
            let fps = fps_count as f64 / elapsed;
            println!("[FPS] {:.2} (window={} frames)", fps, fps_count);
            fps_count = 0;
            fps_start = Instant::now();
        }

        frame_id += 1;
        if cli.max_frames > 0 && frame_id >= cli.max_frames {
            break;
        }

        let tick_elapsed = tick_start.elapsed();
        if tick_elapsed < frame_budget {
            thread::sleep(frame_budget - tick_elapsed);
        }
    }

    Ok(())
}
