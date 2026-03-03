use crate::domain::{Keypoint, PersonPose, PoseFrame};
use crate::math::compute_right_elbow_deg;
use std::f32::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait PoseEstimator {
    fn estimate(&mut self, frame_id: u64) -> Vec<PersonPose>;
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
    fn estimate(&mut self, frame_id: u64) -> Vec<PersonPose> {
        self.phase += 0.08;
        let t = self.phase;

        let shoulder = Keypoint {
            idx: 6,
            x: 320.0,
            y: 180.0,
            score: 0.95,
        };
        let elbow = Keypoint {
            idx: 8,
            x: 360.0 + 10.0 * t.sin(),
            y: 230.0,
            score: 0.95,
        };
        let wrist = Keypoint {
            idx: 10,
            x: 405.0 + 40.0 * (t + PI / 4.0).sin(),
            y: 275.0 + 20.0 * (t * 0.5).cos(),
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
            id: (frame_id % 4) as u32,
            right_elbow_deg,
            keypoints,
        }]
    }
}

pub fn build_frame(frame_id: u64, people: Vec<PersonPose>) -> PoseFrame {
    PoseFrame {
        event: "pose_frame",
        source: "rust-only-mock",
        frame_id,
        ts_ms: now_ms(),
        people,
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
