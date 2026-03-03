use crate::domain::{Keypoint, PoseFrame};

#[cfg(feature = "camera")]
use opencv::core::{Point, Scalar};
#[cfg(feature = "camera")]
use opencv::highgui;
#[cfg(feature = "camera")]
use opencv::imgproc;
#[cfg(feature = "camera")]
use opencv::prelude::*;

#[cfg(feature = "camera")]
const COCO_EDGES: &[(u8, u8)] = &[
    (0, 1),
    (0, 2),
    (1, 3),
    (2, 4),
    (5, 6),
    (5, 7),
    (7, 9),
    (6, 8),
    (8, 10),
    (5, 11),
    (6, 12),
    (11, 12),
    (11, 13),
    (13, 15),
    (12, 14),
    (14, 16),
];

#[cfg(feature = "camera")]
pub fn draw_and_show(
    window_name: &str,
    image: &mut Mat,
    frame: &PoseFrame,
    fps: Option<f64>,
) -> anyhow::Result<bool> {
    if let Some(person) = frame.people.first() {
        draw_full_skeleton(image, &person.keypoints)?;

        let angle_txt = person
            .right_elbow_deg
            .map(|v| format!("Codo: {:.1} deg", v))
            .unwrap_or_else(|| "Codo: n/a".to_string());

        imgproc::put_text(
            image,
            &angle_txt,
            Point::new(20, 35),
            imgproc::FONT_HERSHEY_SIMPLEX,
            0.8,
            Scalar::new(0.0, 255.0, 0.0, 0.0),
            2,
            imgproc::LINE_AA,
            false,
        )?;
    } else {
        imgproc::put_text(
            image,
            "Sin persona detectada",
            Point::new(20, 35),
            imgproc::FONT_HERSHEY_SIMPLEX,
            0.8,
            Scalar::new(0.0, 180.0, 255.0, 0.0),
            2,
            imgproc::LINE_AA,
            false,
        )?;
    }

    let fps_txt = match fps {
        Some(v) => format!("FPS: {:.1}", v),
        None => "FPS: --".to_string(),
    };

    imgproc::put_text(
        image,
        &fps_txt,
        Point::new(20, 70),
        imgproc::FONT_HERSHEY_SIMPLEX,
        0.8,
        Scalar::new(255.0, 255.0, 0.0, 0.0),
        2,
        imgproc::LINE_AA,
        false,
    )?;

    let src_txt = format!("source: {}", frame.source);
    imgproc::put_text(
        image,
        &src_txt,
        Point::new(20, 105),
        imgproc::FONT_HERSHEY_SIMPLEX,
        0.7,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
        2,
        imgproc::LINE_AA,
        false,
    )?;

    highgui::imshow(window_name, image)?;
    let key = highgui::wait_key(1)?;

    Ok(key == 113 || key == 81)
}

#[cfg(feature = "camera")]
fn draw_full_skeleton(image: &mut Mat, keypoints: &[Keypoint]) -> anyhow::Result<()> {
    for &(a, b) in COCO_EDGES {
        let ka = keypoints.iter().find(|k| k.idx == a && k.score > 0.0);
        let kb = keypoints.iter().find(|k| k.idx == b && k.score > 0.0);
        if let (Some(ka), Some(kb)) = (ka, kb) {
            imgproc::line(
                image,
                Point::new(ka.x as i32, ka.y as i32),
                Point::new(kb.x as i32, kb.y as i32),
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                imgproc::LINE_AA,
                0,
            )?;
        }
    }

    for kp in keypoints.iter().filter(|k| k.score > 0.0) {
        imgproc::circle(
            image,
            Point::new(kp.x as i32, kp.y as i32),
            4,
            Scalar::new(0.0, 0.0, 255.0, 0.0),
            -1,
            imgproc::LINE_AA,
            0,
        )?;
    }

    Ok(())
}

#[cfg(not(feature = "camera"))]
#[allow(dead_code)]
pub fn draw_and_show(
    _window_name: &str,
    _image: &mut (),
    _frame: &PoseFrame,
    _fps: Option<f64>,
) -> anyhow::Result<bool> {
    Ok(false)
}
