use crate::domain::Keypoint;

pub fn angle_deg(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> Option<f32> {
    let ba = (a.0 - b.0, a.1 - b.1);
    let bc = (c.0 - b.0, c.1 - b.1);

    let norm_ba = (ba.0 * ba.0 + ba.1 * ba.1).sqrt();
    let norm_bc = (bc.0 * bc.0 + bc.1 * bc.1).sqrt();

    if norm_ba == 0.0 || norm_bc == 0.0 {
        return None;
    }

    let mut cos_angle = (ba.0 * bc.0 + ba.1 * bc.1) / (norm_ba * norm_bc);
    cos_angle = cos_angle.clamp(-1.0, 1.0);

    Some(cos_angle.acos().to_degrees())
}

pub fn compute_right_elbow_deg(keypoints: &[Keypoint]) -> Option<f32> {
    let shoulder = keypoints.iter().find(|k| k.idx == 6)?;
    let elbow = keypoints.iter().find(|k| k.idx == 8)?;
    let wrist = keypoints.iter().find(|k| k.idx == 10)?;

    if shoulder.score <= 0.0 || elbow.score <= 0.0 || wrist.score <= 0.0 {
        return None;
    }

    angle_deg(
        (shoulder.x, shoulder.y),
        (elbow.x, elbow.y),
        (wrist.x, wrist.y),
    )
}

#[cfg(test)]
mod tests {
    use super::angle_deg;

    #[test]
    fn right_angle() {
        let deg = angle_deg((0.0, 1.0), (0.0, 0.0), (1.0, 0.0)).unwrap();
        assert!((deg - 90.0).abs() < 0.001);
    }
}
