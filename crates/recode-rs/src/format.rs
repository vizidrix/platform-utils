pub static GIF: &str = "gif";
pub static JPEG: &str = "jpeg";
pub static PNG: &str = "png";
pub static WEBP: &str = "webp";

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Gif,
    Jpeg,
    Png,
    WebP,
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Gif => GIF.to_owned(),
            Format::Jpeg => JPEG.to_owned(),
            Format::Png => PNG.to_owned(),
            Format::WebP => WEBP.to_owned(),
        }
    }
}
