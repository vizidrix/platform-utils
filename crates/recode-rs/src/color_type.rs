use image;

/// An enumeration over supported color types and bit depths
#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum ColorType {
    /// Pixel is 8-bit luminance
    L8,
    /// Pixel is 8-bit luminance with an alpha channel
    La8,
    /// Pixel contains 8-bit R, G and B channels
    Rgb8,
    /// Pixel is 8-bit RGB with an alpha channel
    Rgba8,

    /// Pixel is 16-bit luminance
    L16,
    /// Pixel is 16-bit luminance with an alpha channel
    La16,
    /// Pixel is 16-bit RGB
    Rgb16,
    /// Pixel is 16-bit RGBA
    Rgba16,

    /// Pixel is 32-bit float RGB
    Rgb32F,
    /// Pixel is 32-bit float RGBA
    Rgba32F,
}

impl ColorType {
    /// Returns the number of bytes contained in a pixel of `ColorType` ```c```
    #[must_use]
    pub fn bytes_per_pixel(self) -> u8 {
        match self {
            ColorType::L8 => 1,
            ColorType::L16 | ColorType::La8 => 2,
            ColorType::Rgb8 => 3,
            ColorType::Rgba8 | ColorType::La16 => 4,
            ColorType::Rgb16 => 6,
            ColorType::Rgba16 => 8,
            ColorType::Rgb32F => 3 * 4,
            ColorType::Rgba32F => 4 * 4,
        }
    }

    /// Returns if there is an alpha channel.
    #[must_use]
    pub fn has_alpha(self) -> bool {
        use ColorType::*;
        match self {
            L8 | L16 | Rgb8 | Rgb16 | Rgb32F => false,
            La8 | Rgba8 | La16 | Rgba16 | Rgba32F => true,
        }
    }

    /// Returns false if the color scheme is grayscale, true otherwise.
    #[must_use]
    pub fn has_color(self) -> bool {
        use ColorType::*;
        match self {
            L8 | L16 | La8 | La16 => false,
            Rgb8 | Rgb16 | Rgba8 | Rgba16 | Rgb32F | Rgba32F => true,
        }
    }

    /// Returns the number of bits contained in a pixel of `ColorType` ```c``` (which will always be
    /// a multiple of 8).
    #[must_use]
    pub fn bits_per_pixel(self) -> u16 {
        <u16 as From<u8>>::from(self.bytes_per_pixel()) * 8
    }

    /// Returns the number of color channels that make up this pixel
    #[must_use]
    pub fn channel_count(self) -> u8 {
        let e: ExtendedColorType = self.into();
        e.channel_count()
    }
}

/// An enumeration of color types encountered in image formats.
///
/// This is not exhaustive over all existing image formats but should be granular enough to allow
/// round tripping of decoding and encoding as much as possible. The variants will be extended as
/// necessary to enable this.
///
/// Another purpose is to advise users of a rough estimate of the accuracy and effort of the
/// decoding from and encoding to such an image format.
#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash)]
#[non_exhaustive]
pub enum ExtendedColorType {
    /// Pixel is 8-bit alpha
    A8,
    /// Pixel is 1-bit luminance
    L1,
    /// Pixel is 1-bit luminance with an alpha channel
    La1,
    /// Pixel contains 1-bit R, G and B channels
    Rgb1,
    /// Pixel is 1-bit RGB with an alpha channel
    Rgba1,
    /// Pixel is 2-bit luminance
    L2,
    /// Pixel is 2-bit luminance with an alpha channel
    La2,
    /// Pixel contains 2-bit R, G and B channels
    Rgb2,
    /// Pixel is 2-bit RGB with an alpha channel
    Rgba2,
    /// Pixel is 4-bit luminance
    L4,
    /// Pixel is 4-bit luminance with an alpha channel
    La4,
    /// Pixel contains 4-bit R, G and B channels
    Rgb4,
    /// Pixel is 4-bit RGB with an alpha channel
    Rgba4,
    /// Pixel is 8-bit luminance
    L8,
    /// Pixel is 8-bit luminance with an alpha channel
    La8,
    /// Pixel contains 8-bit R, G and B channels
    Rgb8,
    /// Pixel is 8-bit RGB with an alpha channel
    Rgba8,
    /// Pixel is 16-bit luminance
    L16,
    /// Pixel is 16-bit luminance with an alpha channel
    La16,
    /// Pixel contains 16-bit R, G and B channels
    Rgb16,
    /// Pixel is 16-bit RGB with an alpha channel
    Rgba16,
    /// Pixel contains 8-bit B, G and R channels
    Bgr8,
    /// Pixel is 8-bit BGR with an alpha channel
    Bgra8,

    // TODO f16 types?
    /// Pixel is 32-bit float RGB
    Rgb32F,
    /// Pixel is 32-bit float RGBA
    Rgba32F,

    /// Pixel is 8-bit CMYK
    Cmyk8,

    /// Pixel is of unknown color type with the specified bits per pixel. This can apply to pixels
    /// which are associated with an external palette. In that case, the pixel value is an index
    /// into the palette.
    Unknown(u8),
}

impl ExtendedColorType {
    /// Get the number of channels for colors of this type.
    ///
    /// Note that the `Unknown` variant returns a value of `1` since pixels can only be treated as
    /// an opaque datum by the library.
    #[must_use]
    pub fn channel_count(self) -> u8 {
        match self {
            ExtendedColorType::A8
            | ExtendedColorType::L1
            | ExtendedColorType::L2
            | ExtendedColorType::L4
            | ExtendedColorType::L8
            | ExtendedColorType::L16
            | ExtendedColorType::Unknown(_) => 1,
            ExtendedColorType::La1
            | ExtendedColorType::La2
            | ExtendedColorType::La4
            | ExtendedColorType::La8
            | ExtendedColorType::La16 => 2,
            ExtendedColorType::Rgb1
            | ExtendedColorType::Rgb2
            | ExtendedColorType::Rgb4
            | ExtendedColorType::Rgb8
            | ExtendedColorType::Rgb16
            | ExtendedColorType::Rgb32F
            | ExtendedColorType::Bgr8 => 3,
            ExtendedColorType::Rgba1
            | ExtendedColorType::Rgba2
            | ExtendedColorType::Rgba4
            | ExtendedColorType::Rgba8
            | ExtendedColorType::Rgba16
            | ExtendedColorType::Rgba32F
            | ExtendedColorType::Bgra8
            | ExtendedColorType::Cmyk8 => 4,
        }
    }

    /// Returns the number of bits per pixel for this color type.
    #[must_use]
    pub fn bits_per_pixel(&self) -> u16 {
        match *self {
            ExtendedColorType::A8 => 8,
            ExtendedColorType::L1 => 1,
            ExtendedColorType::La1 => 2,
            ExtendedColorType::Rgb1 => 3,
            ExtendedColorType::Rgba1 => 4,
            ExtendedColorType::L2 => 2,
            ExtendedColorType::La2 => 4,
            ExtendedColorType::Rgb2 => 6,
            ExtendedColorType::Rgba2 => 8,
            ExtendedColorType::L4 => 4,
            ExtendedColorType::La4 => 8,
            ExtendedColorType::Rgb4 => 12,
            ExtendedColorType::Rgba4 => 16,
            ExtendedColorType::L8 => 8,
            ExtendedColorType::La8 => 16,
            ExtendedColorType::Rgb8 => 24,
            ExtendedColorType::Rgba8 => 32,
            ExtendedColorType::L16 => 16,
            ExtendedColorType::La16 => 32,
            ExtendedColorType::Rgb16 => 48,
            ExtendedColorType::Rgba16 => 64,
            ExtendedColorType::Rgb32F => 96,
            ExtendedColorType::Rgba32F => 128,
            ExtendedColorType::Bgr8 => 24,
            ExtendedColorType::Bgra8 => 32,
            ExtendedColorType::Cmyk8 => 32,
            ExtendedColorType::Unknown(bpp) => bpp as u16,
        }
    }

    /// Returns the number of bytes required to hold a width x height image of this color type.
    pub fn buffer_size(self, width: u32, height: u32) -> u64 {
        let bpp = self.bits_per_pixel() as u64;
        let row_pitch = (width as u64 * bpp + 7) / 8;
        row_pitch.saturating_mul(height as u64)
    }
}

// Mapping to and from image types

impl From<ColorType> for image::ColorType {
    fn from(value: ColorType) -> Self {
        match value {
            ColorType::L8 => image::ColorType::L8,
            ColorType::La8 => image::ColorType::La8,
            ColorType::Rgb8 => image::ColorType::Rgb8,
            ColorType::Rgba8 => image::ColorType::Rgba8,
            ColorType::L16 => image::ColorType::L16,
            ColorType::La16 => image::ColorType::La16,
            ColorType::Rgb16 => image::ColorType::Rgb16,
            ColorType::Rgba16 => image::ColorType::Rgba16,
            ColorType::Rgb32F => image::ColorType::Rgb32F,
            ColorType::Rgba32F => image::ColorType::Rgba32F,
            // _ => image::ColorType::Rgba32F,
        }
    }
}

impl From<ColorType> for ExtendedColorType {
    fn from(value: ColorType) -> Self {
        match value {
            ColorType::L8 => ExtendedColorType::L8,
            ColorType::La8 => ExtendedColorType::La8,
            ColorType::Rgb8 => ExtendedColorType::Rgb8,
            ColorType::Rgba8 => ExtendedColorType::Rgba8,
            ColorType::L16 => ExtendedColorType::L16,
            ColorType::La16 => ExtendedColorType::La16,
            ColorType::Rgb16 => ExtendedColorType::Rgb16,
            ColorType::Rgba16 => ExtendedColorType::Rgba16,
            ColorType::Rgb32F => ExtendedColorType::Rgb32F,
            ColorType::Rgba32F => ExtendedColorType::Rgba32F,
        }
    }
}

impl From<ColorType> for image::ExtendedColorType {
    fn from(value: ColorType) -> Self {
        let ext: ExtendedColorType = value.into();
        ext.into()
    }
}

impl From<image::ColorType> for ColorType {
    fn from(value: image::ColorType) -> Self {
        match value {
            image::ColorType::L8 => ColorType::L8,
            image::ColorType::La8 => ColorType::La8,
            image::ColorType::Rgb8 => ColorType::Rgb8,
            image::ColorType::Rgba8 => ColorType::Rgba8,
            image::ColorType::L16 => ColorType::L16,
            image::ColorType::La16 => ColorType::La16,
            image::ColorType::Rgb16 => ColorType::Rgb16,
            image::ColorType::Rgba16 => ColorType::Rgba16,
            image::ColorType::Rgb32F => ColorType::Rgb32F,
            image::ColorType::Rgba32F => ColorType::Rgba32F,
            _ => ColorType::Rgba32F,
        }
    }
}

impl From<image::ColorType> for ExtendedColorType {
    fn from(value: image::ColorType) -> Self {
        match value {
            image::ColorType::L8 => ExtendedColorType::L8,
            image::ColorType::La8 => ExtendedColorType::La8,
            image::ColorType::Rgb8 => ExtendedColorType::Rgb8,
            image::ColorType::Rgba8 => ExtendedColorType::Rgba8,
            image::ColorType::L16 => ExtendedColorType::L16,
            image::ColorType::La16 => ExtendedColorType::La16,
            image::ColorType::Rgb16 => ExtendedColorType::Rgb16,
            image::ColorType::Rgba16 => ExtendedColorType::Rgba16,
            image::ColorType::Rgb32F => ExtendedColorType::Rgb32F,
            image::ColorType::Rgba32F => ExtendedColorType::Rgba32F,
            _ => ExtendedColorType::Rgba32F,
        }
    }
}

impl From<image::ExtendedColorType> for ExtendedColorType {
    fn from(value: image::ExtendedColorType) -> Self {
        match value {
            image::ExtendedColorType::L8 => ExtendedColorType::L8,
            image::ExtendedColorType::La8 => ExtendedColorType::La8,
            image::ExtendedColorType::Rgb8 => ExtendedColorType::Rgb8,
            image::ExtendedColorType::Rgba8 => ExtendedColorType::Rgba8,
            image::ExtendedColorType::L16 => ExtendedColorType::L16,
            image::ExtendedColorType::La16 => ExtendedColorType::La16,
            image::ExtendedColorType::Rgb16 => ExtendedColorType::Rgb16,
            image::ExtendedColorType::Rgba16 => ExtendedColorType::Rgba16,
            image::ExtendedColorType::Rgb32F => ExtendedColorType::Rgb32F,
            image::ExtendedColorType::Rgba32F => ExtendedColorType::Rgba32F,
            _ => ExtendedColorType::Rgba32F,
        }
    }
}

impl From<ExtendedColorType> for image::ExtendedColorType {
    fn from(value: ExtendedColorType) -> Self {
        match value {
            ExtendedColorType::L8 => image::ExtendedColorType::L8,
            ExtendedColorType::La8 => image::ExtendedColorType::La8,
            ExtendedColorType::Rgb8 => image::ExtendedColorType::Rgb8,
            ExtendedColorType::Rgba8 => image::ExtendedColorType::Rgba8,
            ExtendedColorType::L16 => image::ExtendedColorType::L16,
            ExtendedColorType::La16 => image::ExtendedColorType::La16,
            ExtendedColorType::Rgb16 => image::ExtendedColorType::Rgb16,
            ExtendedColorType::Rgba16 => image::ExtendedColorType::Rgba16,
            ExtendedColorType::Rgb32F => image::ExtendedColorType::Rgb32F,
            ExtendedColorType::Rgba32F => image::ExtendedColorType::Rgba32F,
            _ => image::ExtendedColorType::Rgba32F,
        }
    }
}

// impl From<image::ColorType> for image::ExtendedColorType {
//     fn from(value: image::ColorType) -> Self {
//         let v = value.into().into()
//     }
// }