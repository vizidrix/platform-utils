use image::ImageFormat;

pub static AVIF: &str = "avif";
pub static BMP: &str = "bmp";
pub static DDS: &str = "dds";
pub static OPENEXR: &str = "exr";
pub static FARBFELD: &str = "ff";
pub static GIF: &str = "gif";
pub static HDR: &str = "hdr";
pub static ICO: &str = "ico";
pub static JPEG: &str = "jpeg";
pub static PNG: &str = "png";
pub static PNM: &str = "pnm";
pub static QOI: &str = "qoi";
pub static TGA: &str = "tga";
pub static TIFF: &str = "tiff";
pub static WEBP: &str = "webp";

// ["avif", "bmp", "dds", "exr", "ff", "gif", "hdr", "ico", "jpeg", "png", "pnm", "qoi", "tga", "tiff", "webp"]

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Format {
    Avif,
    Bmp,
    Dds,
    Farbfeld,
    Gif,
    Hdr,
    Ico,
    Jpeg,
    OpenExr,
    Png,
    Pnm,
    Qoi,
    Tga,
    Tiff,
    WebP,
}

impl Format {
    pub fn from_extension<S>(ext: S) -> Option<Format>
    where
        S: AsRef<std::ffi::OsStr>
    {
        image::ImageFormat::from_extension(ext)
            .map(|o| o.into())
    }

    pub fn from_mime_type<M>(mime_type: M) -> Option<Format>
    where
        M: AsRef<str>
    {
        image::ImageFormat::from_mime_type(mime_type)
            .map(|o| o.into())
    }
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Avif => AVIF.to_owned(),
            Format::Bmp => BMP.to_owned(),
            Format::Dds => DDS.to_owned(),
            Format::Farbfeld => FARBFELD.to_owned(),
            Format::Gif => GIF.to_owned(),
            Format::Hdr => HDR.to_owned(),
            Format::Ico => ICO.to_owned(),
            Format::Jpeg => JPEG.to_owned(),
            Format::OpenExr => OPENEXR.to_owned(),
            Format::Png => PNG.to_owned(),
            Format::Pnm => PNM.to_owned(),
            Format::Qoi => QOI.to_owned(),
            Format::Tga => TGA.to_owned(),
            Format::Tiff => TIFF.to_owned(),
            Format::WebP => WEBP.to_owned(),
        }
    }
}

impl From<ImageFormat> for Format {
    fn from(value: ImageFormat) -> Self {
        match value {
            ImageFormat::Avif => Format::Avif,
            ImageFormat::Bmp => Format::Bmp,
            ImageFormat::Dds => Format::Dds,
            ImageFormat::Farbfeld => Format::Farbfeld,
            ImageFormat::Gif => Format::Gif,
            ImageFormat::Hdr => Format::Hdr,
            ImageFormat::Ico => Format::Ico,
            ImageFormat::Jpeg => Format::Jpeg,
            ImageFormat::OpenExr => Format::OpenExr,
            ImageFormat::Png => Format::Png,
            ImageFormat::Pnm => Format::Pnm,
            ImageFormat::Qoi => Format::Qoi,
            ImageFormat::Tga => Format::Tga,
            ImageFormat::Tiff => Format::Tiff,
            ImageFormat::WebP => Format::WebP,
            _ => Format::WebP,
        }
    }
}

// impl TryFrom<ImageFormat> for Format {
//     type Error = Error;

//     fn try_from(value: ImageFormat) -> Result<Self, Self::Error> {
//         match value {
//             ImageFormat::Avif => Ok(Format::Avif),
//             ImageFormat::Bmp => Ok(Format::Bmp),
//             ImageFormat::Dds => Ok(Format::Dds),
//             ImageFormat::Farbfeld => Ok(Format::Farbfeld),
//             ImageFormat::Gif => Ok(Format::Gif),
//             ImageFormat::Hdr => Ok(Format::Hdr),
//             ImageFormat::Ico => Ok(Format::Ico),
//             ImageFormat::Jpeg => Ok(Format::Jpeg),
//             ImageFormat::OpenExr => Ok(Format::OpenExr),
//             ImageFormat::Png => Ok(Format::Png),
//             ImageFormat::Pnm => Ok(Format::Pnm),
//             ImageFormat::Qoi => Ok(Format::Qoi),
//             ImageFormat::Tga => Ok(Format::Tga),
//             ImageFormat::Tiff => Ok(Format::Tiff),
//             ImageFormat::WebP => Ok(Format::WebP),
//             _ => Err(Error::UnsupportedFormat)
//         }
//     }
// }