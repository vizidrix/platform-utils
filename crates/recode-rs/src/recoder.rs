use crate::{ColorType, Error, Format, Outcome};

use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::codecs::webp::WebPEncoder;
use image::{
    // guess_format, load_from_memory, EncodableLayout, ImageEncoder
    guess_format, load_from_memory, ImageEncoder
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Recoder {
    format: Format,
    width: u32,
    height: u32,
    color: ColorType,
    data: Vec<u8>,
}

impl Recoder {
    /// Some formats available from the image library may not be supported natively but could be
    /// pre-processed into one of the supported formats via a separate process.
    ///
    /// Known supported formats that aren't implemented here are:
    /// ["avif", "bmp", "dds", "ff"/"farbfeld", "gif", "hdr", "ico", "jpeg", "exr"/"openexr", "png", "pnm", "qoi", "tga", "tiff", "webp"]
    pub fn new(buffer: &[u8]) -> Result<Self, Error> {
        // Try to get the image format
        let format = guess_format(&buffer)
            .map_err(|_| Error::UnsupportedFormat)?;
        // Try to load an unknown blob of image data
        let dynamic_image = load_from_memory(buffer).map_err(|_| Error::LoadError)?;
        let (width, height) = (dynamic_image.width(), dynamic_image.height());
        let color = dynamic_image.color();
        let data =  dynamic_image.as_bytes().to_vec();

        Ok(Recoder {
            format: format.try_into()?,
            width,
            height,
            color: color.into(),
            data,
        })
    }

    pub fn to_outcome(&self, new_format: Format, new_data: Vec<u8>) -> Outcome {
        Outcome {
            src: self.format,
            width: self.width,
            height: self.height,
            dest: new_format,
            data: new_data,
        }
    }

    pub fn to_png(&self) -> Result<Outcome, Error> {
        // Make a buffer to write into
        let mut out_buffer = Vec::<u8>::new();
        // Setup the encoder with fast and no filter to try and avoid any compression or other data loss
        let png_encoder = PngEncoder::new_with_quality(
            &mut out_buffer,
            CompressionType::Best,
            FilterType::NoFilter,
        );
        // Try to write the image as a PNG to the buffer
        // png_encoder.write_image(image_16bit.as_bytes(), width, height, ColorType::Rgba16)?;
        // png_encoder.write_image(image_16bit.as_bytes(), width, height, ExtendedColorType::Rgba16)?;
        png_encoder.write_image(&self.data, self.width, self.height, self.color.into())?;

        Ok(self.to_outcome(Format::Png, out_buffer))
    }

    pub fn to_webp(&self) -> Result<Outcome, Error> {
        // Make a buffer to write into
        let mut out_buffer = Vec::<u8>::new();
        let webp_encoder = WebPEncoder::new_lossless(&mut out_buffer);
        // Try to write the image as a WebP to the buffer
        webp_encoder.write_image(&self.data, self.width, self.height, self.color.into())?;

        Ok(self.to_outcome(Format::WebP, out_buffer))
    }
}

// pub fn to_webp_enhanced(&self, buffer: &[u8]) -> Result<Outcome, Error> {
//     let base_webp = self.to_webp(buffer)?;
//     // let options = Options {
//     //     fix_errors: false,
//     //     force: false,
//     //     filter: indexset! {RowFilter::None, RowFilter::Sub, RowFilter::Entropy, RowFilter::Bigrams},
//     //     interlace: Some(Interlacing::None),
//     //     optimize_alpha: false,
//     //     bit_depth_reduction: true,
//     //     color_type_reduction: true,
//     //     palette_reduction: true,
//     //     grayscale_reduction: true,
//     //     idat_recoding: true,
//     //     scale_16: false,
//     //     strip: StripChunks::None,
//     //     deflate: Deflaters::Libdeflater { compression: 11 },
//     //     fast_evaluation: true,
//     //     timeout: None,
//     // };
//     // Create custom options for Oxipng
//     let options = Options {
//         // strip: Some(oxipng::Metadata::All),
//         strip: StripChunks::All,
//         // interlace: Some(1),
//         interlace: Some(Interlacing::Adam7),
//         // compression: Deflate::new(9),
//         deflate: Deflaters::Libdeflater { compression: 9 },
//         // filter: vec![FilterType::Paeth],
//         filter: indexset! {RowFilter::None, RowFilter::Sub, RowFilter::Entropy, RowFilter::Bigrams},
//         ..Options::default()
//     };
//     let png_data = PngData::from_slice(&base_webp.data, &options)?;
//     let output = png_data.output();
//     Ok(Outcome::new(
//         base_webp.src,
//         base_webp.dest,
//         output,
//     ))
// }
