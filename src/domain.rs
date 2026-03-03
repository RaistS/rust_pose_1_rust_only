use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keypoint {
    pub idx: u8,
    pub x: f32,
    pub y: f32,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonPose {
    pub id: u32,
    pub right_elbow_deg: Option<f32>,
    pub keypoints: Vec<Keypoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoseFrame {
    pub event: &'static str,
    pub source: &'static str,
    pub frame_id: u64,
    pub ts_ms: u64,
    pub people: Vec<PersonPose>,
}
