use crate::{Error, Format, Meta, Outcome};

use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::codecs::webp::WebPEncoder;
use image::{
    guess_format, load_from_memory, EncodableLayout, ImageEncoder, ImageFormat
};
// use internal_tests::PngData;
// use oxipng::*;

/// Some formats available from the image library aren't supported natively but could be
/// pre-processed into one of the supported formats via a separate process.
///
/// Known supported formats that aren't implemented here are:
/// [ "avif", "bmp", "dds", "farbfeld", "hdr", "ico", "openexr", "pnm", "tga", "tiff" ]
pub fn guess_image_format(buffer: &[u8]) -> Result<Format, Error> {
    Ok(
        match guess_format(buffer).map_err(|_| Error::UnsupportedFormat)? {
            ImageFormat::Gif => Format::Gif,
            ImageFormat::Jpeg => Format::Jpeg,
            ImageFormat::Png => Format::Png,
            ImageFormat::WebP => Format::WebP,
            _ => return Err(Error::UnsupportedFormat),
        },
    )
}

#[derive(Default)]
pub struct Recoder {}

impl Recoder {
    pub fn new() -> Self {
        Recoder {}
    }

    pub fn to_png(&self, buffer: &[u8]) -> Result<Outcome, Error> {
        // Try to get the image format
        let format = guess_image_format(buffer)?;
        // Try to load an unknown blob of image data
        let dynamic_image = load_from_memory(buffer).map_err(|_| Error::LoadError)?;
        let (width, height) = (dynamic_image.width(), dynamic_image.height());
        // Capture the input image metadata
        let src_meta = Meta::new(format, width, height);
        // Convert the imported photo data into the desired 16 bit depth
        // let image_16bit = dynamic_image.into_rgba16();
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
        png_encoder.write_image(dynamic_image.as_bytes(), width, height, dynamic_image.color().into())?;
        // Describe the output image metadata
        let dest_meta = Meta::new(Format::Png, width, height);
        // Return the result of the recoding process
        Ok(Outcome::new(
            src_meta,
            dest_meta,
            out_buffer.as_bytes().to_vec(),
        ))
    }

    pub fn to_webp(&self, buffer: &[u8]) -> Result<Outcome, Error> {
        // Try to get the image format
        let format = guess_image_format(buffer)?;
        // Try to load an unknown blob of image data
        let dynamic_image = load_from_memory(buffer).map_err(|_| Error::LoadError)?;
        let (width, height) = (dynamic_image.width(), dynamic_image.height());
        // Capture the input image metadata
        let src_meta = Meta::new(format, width, height);
        // Make a buffer to write into
        let mut out_buffer = Vec::<u8>::new();
        let webp_encoder = WebPEncoder::new_lossless(&mut out_buffer);
        // Try to write the image as a WebP to the buffer
        webp_encoder.write_image(dynamic_image.as_bytes(), width, height, dynamic_image.color().into())?;
        // Describe the output image metadata
        let dest_meta = Meta::new(Format::WebP, width, height);
        // Return the result of the recoding process
        Ok(Outcome::new(
            src_meta,
            dest_meta,
            out_buffer.as_bytes().to_vec(),
        ))
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
}
