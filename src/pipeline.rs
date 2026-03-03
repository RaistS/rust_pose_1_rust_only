use crate::domain::{Keypoint, PersonPose, PoseFrame};
use crate::math::compute_right_elbow_deg;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct PipelineInput {
    pub frame_id: u64,
    pub ts_ms: u64,
    pub image_width: u32,
    pub image_height: u32,
}

pub trait PoseEstimator {
    fn estimate(&mut self, input: &PipelineInput) -> Vec<PersonPose>;
}

pub struct MockEstimator {
    phase: f32,
}

impl MockEstimator {
    pub fn new() -> Self {
        Self { phase: 0.0 }
    }
}

impl PoseEstimator for MockEstimator {
    fn estimate(&mut self, input: &PipelineInput) -> Vec<PersonPose> {
        self.phase += 0.08;
        let t = self.phase;

        let cx = (input.image_width as f32 * 0.5).max(160.0);
        let cy = (input.image_height as f32 * 0.45).max(120.0);

        let shoulder = Keypoint {
            idx: 6,
            x: cx,
            y: cy,
            score: 0.95,
        };
        let elbow = Keypoint {
            idx: 8,
            x: cx + 45.0 + 10.0 * t.sin(),
            y: cy + 55.0,
            score: 0.95,
        };
        let wrist = Keypoint {
            idx: 10,
            x: cx + 95.0 + 40.0 * (t + PI / 4.0).sin(),
            y: cy + 100.0 + 20.0 * (t * 0.5).cos(),
            score: 0.95,
        };

        let mut keypoints = vec![shoulder, elbow, wrist];
        for idx in 0..17u8 {
            if idx == 6 || idx == 8 || idx == 10 {
                continue;
            }
            keypoints.push(Keypoint {
                idx,
                x: 0.0,
                y: 0.0,
                score: 0.0,
            });
        }

        let right_elbow_deg = compute_right_elbow_deg(&keypoints);

        vec![PersonPose {
            id: (input.frame_id % 4) as u32,
            right_elbow_deg,
            keypoints,
        }]
    }
}

pub fn build_frame(source: &str, input: &PipelineInput, people: Vec<PersonPose>) -> PoseFrame {
    PoseFrame {
        event: "pose_frame".to_string(),
        source: source.to_string(),
        frame_id: input.frame_id,
        ts_ms: input.ts_ms,
        image_width: input.image_width,
        image_height: input.image_height,
        people,
    }
}

pub fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
