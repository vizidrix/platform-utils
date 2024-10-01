use std::io::Cursor;
use serde::{Serialize, Deserialize};

use image::{ ImageBuffer, DynamicImage, ImageFormat, ImageError };//, ImageOutputFormat };
use image::imageops::resize;
use q_rs::*;

#[derive(Debug)]
pub enum QrPngError {
    ImageError(ImageError),
    QrError(q_rs::QrError),
}

impl std::error::Error for QrPngError {}

impl From<ImageError> for QrPngError {
    fn from(src: ImageError) -> Self {
        QrPngError::ImageError(src)
    }
}

impl From<QrError> for QrPngError {
    fn from(value: QrError) -> Self {
        QrPngError::QrError(value)
    }
}

impl std::fmt::Display for QrPngError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ImageError(err) => {
                write!(f, "{:?}", err)
            },
            Self::QrError(err) => {
                write!(f, "{:?}", err)
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ColorTemplate {
    BlackOnWhite,
    BlackOnTransparant,
    WhiteOnTransparant,
    // LumaA Byte, Alpha Byte
    CustomGrayOnTransparant {
        gray: u8,
        alpha: u8,
    },
}

impl ColorTemplate {
    pub fn into_colors(&self) -> (image::LumaA<u8>, image::LumaA<u8>) {
        match self {
            ColorTemplate::BlackOnWhite => (image::LumaA([0u8, 255u8]), image::LumaA([255u8, 255u8])),
            ColorTemplate::BlackOnTransparant => (image::LumaA([0u8, 255u8]), image::LumaA([0u8, 0u8])),
            ColorTemplate::WhiteOnTransparant => (image::LumaA([255u8, 255u8]), image::LumaA([0u8, 0u8])),
            ColorTemplate::CustomGrayOnTransparant { gray, alpha } => (image::LumaA([*gray, *alpha]), image::LumaA([0u8, 0u8])),
        }
    }
}

impl Default for ColorTemplate {
    fn default() -> Self {
        ColorTemplate::BlackOnWhite
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DensityVersion(u8);

impl DensityVersion {
    pub fn new(value: u8) -> Self {
        DensityVersion(value)
    }
}

impl Default for DensityVersion {
    fn default() -> Self {
        DensityVersion(1)
    }
}

impl From<DensityVersion> for Version {
    fn from(value: DensityVersion) -> Self {
        let DensityVersion(data) = value;
        Version::new(data)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCorrection {
    /// The QR Code can tolerate about  7% erroneous codewords.
    Low,
    /// The QR Code can tolerate about 15% erroneous codewords.
    Medium,
    /// The QR Code can tolerate about 25% erroneous codewords.
    Quartile,
    /// The QR Code can tolerate about 30% erroneous codewords.
    High,
}

impl Default for ErrorCorrection {
    fn default() -> Self {
        ErrorCorrection::Medium
    }
}

impl From<ErrorCorrection> for CodeEcc {
    fn from(value: ErrorCorrection) -> Self {
        match value {
            ErrorCorrection::Low => CodeEcc::Low,
            ErrorCorrection::Medium => CodeEcc::Medium,
            ErrorCorrection::Quartile => CodeEcc::Quartile,
            ErrorCorrection::High => CodeEcc::High,
        }
    }
}

/// Returns a QR Code representing the given segments with the given encoding parameters.
///
/// The smallest possible QR Code version within the given range is automatically
/// chosen for the output. Iff boostecl is `true`, then the ECC level of the result
/// may be higher than the ecl argument if it can be done without increasing the
/// version. The mask number is either between 0 to 7 (inclusive) to force that
/// mask, or `None` to automatically choose an appropriate mask (which may be slow).
///
/// This function allows the user to create a custom sequence of segments that switches
/// between modes (such as alphanumeric and byte) to encode text in less space.
/// This is a mid-level API; the high-level API is `encode_text()` and `encode_binary()`.
///
/// Returns a wrapped `QrCode` if successful, or `Err` if the data is too
/// long to fit in any version in the given range at the given ECC level.
#[derive(Debug, Serialize, Deserialize)]
pub struct QROptions {
    // Sets the colors used for the foreground and background
    pub color_template: Option<ColorTemplate>,
    // Sets the minimum block density of the QR
    pub min_version: Option<DensityVersion>,
    // Sets the maximum density of the QR
    pub max_version: Option<DensityVersion>,
    // Determines the number of failing blocks for error correction
    pub error_correction: Option<ErrorCorrection>,
    // Defines the default size of each block in the QR
    pub scale: Option<u8>,
    // Specify the mask if desired
    pub mask: Option<u8>,
    // True automatically optimizes the error correction within version bounds if possible
    pub boost_ecl: bool,
}

impl Default for QROptions {
    fn default() -> Self {
        QROptions {
            color_template: None,
            min_version: None,
            max_version: None,
            error_correction: None,
            scale: None,
            mask: None,
            boost_ecl: true,
        }
    }
}

pub async fn generate_qr_image(
    data: &str,
    options: Option<QROptions>,
) -> Result<Vec<u8>, QrPngError> {
    let segments = Segment::make_segments(data);
    let options = options.unwrap_or_default();
    let color_template = options.color_template.unwrap_or_default();
    let min_version = options.min_version.unwrap_or(DensityVersion(1));
    let max_version = options.max_version.unwrap_or(DensityVersion(10));
    let error_correction = options.error_correction.unwrap_or_default();
    let scale = options.scale.unwrap_or(8) as i32;
    let mask = options.mask.map(|v| Mask::new(v));
    let boost_ecl = options.boost_ecl;

    // let qr = QrCode::encode_segments_advanced(&segments, CodeEcc::Medium,
    //     Version::new(5), Version::new(5), Some(Mask::new(2)), false).unwrap();
    // let qr = QrCode::encode_segments_advanced(&segments, error_correction.into(), min_version.into(), max_version.into(), mask, boost_ecl).unwrap();
    let qr = QrCode::encode_segments_advanced(&segments, error_correction.into(), min_version.into(), max_version.into(), mask, boost_ecl)?;
    // let png: ImageBuffer<Luma<u8>, Vec<u8>> = qr.render::<Luma<u8>>().build();
    let size = qr.size;
    
    let (on, off) = color_template.into_colors();
    let png = ImageBuffer::from_fn(size as u32, size as u32, |x, y| {
        if qr.get_module(x as i32, y as i32) {
            // image::LumaA([0u8, 0u8])
            // image::Luma([0u8])
            on
        } else {
            off
            // image::LumaA([255u8, 255u8])
            // image::Luma([255u8])
        }
    });
    // let scale = 8;
    let resized = resize(&png, (size * scale) as u32, (size * scale) as u32, image::imageops::FilterType::Nearest);
    let mut w = Cursor::new(Vec::new());
    // DynamicImage::ImageLuma8(resized)
    DynamicImage::ImageLumaA8(resized)
        // .write_to(&mut w, ImageOutputFormat::Png)
        .write_to(&mut w, ImageFormat::Png)?;
    let vec: Vec<_> = w.into_inner();
    Ok(vec)
}