#[cfg(feature = "camera")]
use anyhow::Context;
#[cfg(feature = "camera")]
use opencv::core::{self, Mat, Scalar, Size};
#[cfg(feature = "camera")]
use opencv::dnn;
#[cfg(feature = "camera")]
use opencv::prelude::*;

use crate::domain::{Keypoint, PersonPose};
use crate::math::compute_right_elbow_deg;

#[cfg(feature = "camera")]
pub struct OnnxPoseEstimator {
    net: dnn::Net,
    input_size: i32,
    conf_thres: f32,
    kpt_thres: f32,
}

#[cfg(feature = "camera")]
impl OnnxPoseEstimator {
    pub fn new(model_path: &str, input_size: i32, conf_thres: f32, kpt_thres: f32) -> anyhow::Result<Self> {
        let mut net = dnn::read_net_from_onnx(model_path)
            .with_context(|| format!("No se pudo cargar modelo ONNX: {model_path}"))?;
        net.set_preferable_backend(dnn::DNN_BACKEND_OPENCV)?;
        net.set_preferable_target(dnn::DNN_TARGET_CPU)?;

        Ok(Self {
            net,
            input_size,
            conf_thres,
            kpt_thres,
        })
    }

    pub fn estimate(&mut self, frame_bgr: &Mat) -> anyhow::Result<Vec<PersonPose>> {
        let (padded, scale, pad_x, pad_y) = letterbox(frame_bgr, self.input_size)?;

        let blob = dnn::blob_from_image(
            &padded,
            1.0 / 255.0,
            Size::new(self.input_size, self.input_size),
            Scalar::default(),
            true,
            false,
            core::CV_32F,
        )?;

        self.net.set_input_def(&blob)?;
        let out = self.net.forward_single_def()?;

        let (channels, preds, channel_first) = detect_layout(&out)?;
        if channels < 56 || preds == 0 {
            return Ok(vec![]);
        }

        let data = out.data_typed::<f32>()?;

        let mut best_i = None;
        let mut best_conf = 0.0f32;

        for i in 0..preds {
            let conf = sigmoid(at(data, channels, preds, i, 4, channel_first));
            if conf > self.conf_thres && conf > best_conf {
                best_conf = conf;
                best_i = Some(i);
            }
        }

        let Some(i) = best_i else {
            return Ok(vec![]);
        };

        let mut keypoints = Vec::with_capacity(17);
        for k in 0..17usize {
            let base = 5 + k * 3;
            let x = at(data, channels, preds, i, base, channel_first);
            let y = at(data, channels, preds, i, base + 1, channel_first);
            let s = sigmoid(at(data, channels, preds, i, base + 2, channel_first));

            let ox = ((x - pad_x) / scale).clamp(0.0, (frame_bgr.cols() - 1).max(0) as f32);
            let oy = ((y - pad_y) / scale).clamp(0.0, (frame_bgr.rows() - 1).max(0) as f32);

            keypoints.push(Keypoint {
                idx: k as u8,
                x: ox,
                y: oy,
                score: if s >= self.kpt_thres { s } else { 0.0 },
            });
        }

        let right_elbow_deg = compute_right_elbow_deg(&keypoints);

        Ok(vec![PersonPose {
            id: 0,
            right_elbow_deg,
            keypoints,
        }])
    }
}

#[cfg(feature = "camera")]
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

#[cfg(feature = "camera")]
fn at(data: &[f32], channels: usize, preds: usize, pred_idx: usize, ch_idx: usize, channel_first: bool) -> f32 {
    if channel_first {
        data[ch_idx * preds + pred_idx]
    } else {
        data[pred_idx * channels + ch_idx]
    }
}

#[cfg(feature = "camera")]
fn detect_layout(out: &Mat) -> anyhow::Result<(usize, usize, bool)> {
    let dims = out.mat_size().dims();

    if dims >= 3 {
        let d1 = out.mat_size().get(1)? as usize;
        let d2 = out.mat_size().get(2)? as usize;

        if d1 == 56 {
            return Ok((56, d2, true));
        }
        if d2 == 56 {
            return Ok((56, d1, false));
        }
    }

    let rows = out.rows().max(0) as usize;
    let cols = out.cols().max(0) as usize;

    if rows == 56 {
        return Ok((56, cols, true));
    }
    if cols == 56 {
        return Ok((56, rows, false));
    }

    anyhow::bail!("Salida ONNX con shape no soportada para YOLO pose")
}

#[cfg(feature = "camera")]
fn letterbox(src: &Mat, input_size: i32) -> anyhow::Result<(Mat, f32, f32, f32)> {
    let w = src.cols().max(1) as f32;
    let h = src.rows().max(1) as f32;
    let target = input_size as f32;

    let scale = (target / w).min(target / h);
    let new_w = (w * scale).round() as i32;
    let new_h = (h * scale).round() as i32;

    let mut resized = Mat::default();
    opencv::imgproc::resize(
        src,
        &mut resized,
        Size::new(new_w, new_h),
        0.0,
        0.0,
        opencv::imgproc::INTER_LINEAR,
    )?;

    let dw = input_size - new_w;
    let dh = input_size - new_h;

    let left = dw / 2;
    let right = dw - left;
    let top = dh / 2;
    let bottom = dh - top;

    let mut padded = Mat::default();
    core::copy_make_border(
        &resized,
        &mut padded,
        top,
        bottom,
        left,
        right,
        core::BORDER_CONSTANT,
        Scalar::new(114.0, 114.0, 114.0, 0.0),
    )?;

    Ok((padded, scale, left as f32, top as f32))
}
