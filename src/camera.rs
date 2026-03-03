#[cfg(feature = "camera")]
use anyhow::Context;

#[cfg(feature = "camera")]
use opencv::core::Mat;
#[cfg(feature = "camera")]
use opencv::prelude::*;
#[cfg(feature = "camera")]
use opencv::videoio::{VideoCapture, CAP_ANY, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct CaptureFrame {
    pub width: u32,
    pub height: u32,
    pub ts_ms: u64,
}

#[cfg(feature = "camera")]
pub struct CameraCapture {
    cap: VideoCapture,
}

#[cfg(feature = "camera")]
impl CameraCapture {
    pub fn open(index: i32, width: i32, height: i32) -> anyhow::Result<Self> {
        let mut cap = VideoCapture::new(index, CAP_ANY)
            .with_context(|| format!("No se pudo abrir camara index={index}"))?;

        cap.set(CAP_PROP_FRAME_WIDTH, width as f64)
            .context("No se pudo configurar width")?;
        cap.set(CAP_PROP_FRAME_HEIGHT, height as f64)
            .context("No se pudo configurar height")?;

        let opened = cap.is_opened().context("Error validando estado de camara")?;
        if !opened {
            anyhow::bail!("Camara no disponible en index={index}");
        }

        Ok(Self { cap })
    }

    pub fn next_frame(&mut self) -> anyhow::Result<CaptureFrame> {
        let mut mat = Mat::default();
        self.cap.read(&mut mat).context("No se pudo leer frame de camara")?;

        if mat.empty() {
            anyhow::bail!("Frame vacio recibido desde camara");
        }

        let width = mat.cols().max(0) as u32;
        let height = mat.rows().max(0) as u32;

        Ok(CaptureFrame {
            width,
            height,
            ts_ms: now_ms(),
        })
    }
}

#[cfg(not(feature = "camera"))]
pub struct CameraCapture;

#[cfg(not(feature = "camera"))]
impl CameraCapture {
    pub fn open(_index: i32, _width: i32, _height: i32) -> anyhow::Result<Self> {
        anyhow::bail!(
            "Modo camera requiere compilar con --features camera (dependencia opencv)"
        );
    }

    pub fn next_frame(&mut self) -> anyhow::Result<CaptureFrame> {
        anyhow::bail!(
            "Modo camera requiere compilar con --features camera (dependencia opencv)"
        );
    }
}

#[cfg(feature = "camera")]
fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
