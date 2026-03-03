use crate::domain::PoseFrame;

#[cfg(feature = "camera")]
use opencv::core::{Point, Scalar};
#[cfg(feature = "camera")]
use opencv::highgui;
#[cfg(feature = "camera")]
use opencv::imgproc;
#[cfg(feature = "camera")]
use opencv::prelude::*;

#[cfg(feature = "camera")]
pub fn draw_and_show(
    window_name: &str,
    image: &mut Mat,
    frame: &PoseFrame,
    fps: Option<f64>,
) -> anyhow::Result<bool> {
    if let Some(person) = frame.people.first() {
        let kp6 = person.keypoints.iter().find(|k| k.idx == 6 && k.score > 0.0);
        let kp8 = person.keypoints.iter().find(|k| k.idx == 8 && k.score > 0.0);
        let kp10 = person.keypoints.iter().find(|k| k.idx == 10 && k.score > 0.0);

        if let (Some(a), Some(b), Some(c)) = (kp6, kp8, kp10) {
            let p1 = Point::new(a.x as i32, a.y as i32);
            let p2 = Point::new(b.x as i32, b.y as i32);
            let p3 = Point::new(c.x as i32, c.y as i32);

            imgproc::line(
                image,
                p1,
                p2,
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                imgproc::LINE_AA,
                0,
            )?;
            imgproc::line(
                image,
                p2,
                p3,
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                imgproc::LINE_AA,
                0,
            )?;

            for p in [p1, p2, p3] {
                imgproc::circle(
                    image,
                    p,
                    6,
                    Scalar::new(0.0, 0.0, 255.0, 0.0),
                    -1,
                    imgproc::LINE_AA,
                    0,
                )?;
            }
        }

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
