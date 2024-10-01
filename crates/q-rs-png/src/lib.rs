use std::io::Cursor;

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

pub enum ColorTemplate {
    BlackOnWhite,
    BlackOnTransparant,
    WhiteOnTransparant,
}

impl ColorTemplate {
    pub fn into_colors(&self) -> (image::LumaA<u8>, image::LumaA<u8>) {
        match self {
            ColorTemplate::BlackOnWhite => (image::LumaA([0u8, 255u8]), image::LumaA([255u8, 255u8])),
            ColorTemplate::BlackOnTransparant => (image::LumaA([0u8, 255u8]), image::LumaA([0u8, 0u8])),
            ColorTemplate::WhiteOnTransparant => (image::LumaA([255u8, 255u8]), image::LumaA([0u8, 0u8])),
        }
    }
}

pub async fn generate_qr_image(
    data: &str,
    color_template: ColorTemplate,
) -> Result<Vec<u8>, QrPngError> {
    let segments = Segment::make_segments(data);
    let qr = QrCode::encode_segments_advanced(&segments, CodeEcc::Medium,
        Version::new(5), Version::new(5), Some(Mask::new(2)), false).unwrap();
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
    let scale = 8;
    let resized = resize(&png, (size * scale) as u32, (size * scale) as u32, image::imageops::FilterType::Nearest);
    let mut w = Cursor::new(Vec::new());
    // DynamicImage::ImageLuma8(resized)
    DynamicImage::ImageLumaA8(resized)
        // .write_to(&mut w, ImageOutputFormat::Png)
        .write_to(&mut w, ImageFormat::Png)?;
    let vec: Vec<_> = w.into_inner();
    Ok(vec)
}