use crate::domain::PoseFrame;
use anyhow::Context;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct FrameSink {
    file: Option<BufWriter<File>>,
}

impl FrameSink {
    pub fn new(path: Option<&Path>) -> anyhow::Result<Self> {
        let file = if let Some(path) = path {
            let file = File::create(path)
                .with_context(|| format!("No se pudo crear archivo de salida: {}", path.display()))?;
            Some(BufWriter::new(file))
        } else {
            None
        };

        Ok(Self { file })
    }

    pub fn write(&mut self, frame: &PoseFrame) -> anyhow::Result<usize> {
        let json = serde_json::to_string(frame).context("No se pudo serializar PoseFrame")?;

        if let Some(file) = self.file.as_mut() {
            file.write_all(json.as_bytes())?;
            file.write_all(b"\n")?;
            file.flush()?;
        }

        Ok(json.len())
    }
}
