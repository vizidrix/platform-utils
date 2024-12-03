use thiserror::Error;
use image::ImageError;
use q_rs::QrError;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum QrPngError {
    #[error("image error: {0}")]
    ImageError(#[from] ImageError),
    
    #[error("qr error: {0}")]
    QrError(#[from] QrError),
    
    #[error("invalid data: {0}")]
    InvalidData(#[from] Box<ErrorPayload>),
}

#[derive(Debug, Clone, Error)]
pub enum ErrorPayload {
    #[error("payload too large: size {size} exceeds max {max}")]
    PayloadTooLarge { size: usize, max: usize },

    #[error("invalid dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },

    #[error("invalid payload format")]
    InvalidPayload,

    #[error("metadata missing")]
    MissingMetadata,

    #[error("verification failed: {0}")]
    VerificationFailed(String),
}

