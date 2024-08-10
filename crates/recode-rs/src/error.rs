use image::ImageError;
use oxipng::PngError;

#[derive(Clone, Debug)]
pub enum Error {
    LoadError,
    ImageError(String),
    PngError(PngError),
    UnsupportedFormat,
}

impl From<ImageError> for Error {
    fn from(src: ImageError) -> Self {
        Error::ImageError(src.to_string())
    }
}

impl From<PngError> for Error {
    fn from(src: PngError) -> Self {
        Error::PngError(src)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Error::LoadError => "load error".to_owned(),
            Error::ImageError(msg) => format!("image error: {msg}"),
            Error::PngError(err) => format!("png error: {err}"),
            Error::UnsupportedFormat => "unsupported format".to_owned(),
        };
        write!(f, "Error ( {message} )")
    }
}
