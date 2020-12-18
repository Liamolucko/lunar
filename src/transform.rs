use std::path::Path;

use anyhow::Result;

pub enum TransformError {
    UnsupportedType,
    IoError(std::io::Error),
}

impl From<std::io::Error> for TransformError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

pub async fn transform(path: &Path) -> Result<String, TransformError> {
    if let Some("js") = path.extension().map(|ext| ext.to_str()).flatten() {
        Ok(tokio::fs::read_to_string(path).await?)
    } else {
        Err(TransformError::UnsupportedType)
    }
}
