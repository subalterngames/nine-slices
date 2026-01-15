use crate::Error;
use fast_image_resize::images::Image;

#[cfg(feature = "png")]
pub struct PngPixelType {
    pub color_type: png::ColorType,
    pub bit_depth: png::BitDepth,
}

/// Pixel types as expressed in different crates.
pub struct PixelType {
    pub blittle: blittle::PixelType,
    pub fast_image_resize: fast_image_resize::PixelType,
    #[cfg(feature = "png")]
    pub png: PngPixelType,
}

impl PixelType {
    pub fn new(image: &Image) -> Result<Self, Error> {
        let fast_image_resize = image.pixel_type();
        let blittle = Self::get_blittle_pixel_type(fast_image_resize)?;

        #[cfg(feature = "png")]
        {
            let png = Self::get_png_pixel_type(&blittle).map_err(Error::Png)?;
            Ok(Self {
                blittle,
                fast_image_resize,
                png,
            })
        }
        #[cfg(not(feature = "png"))]
        {
            Ok(Self {
                blittle,
                fast_image_resize,
            })
        }
    }

    /// Derive the pixel type from a png `ColorType` and `BitDepth`.
    /// Some combinations of `ColorType` and `BitDepth` are not allowed.
    #[cfg(feature = "png")]
    pub fn from_png(color_type: &png::ColorType, bit_depth: &png::BitDepth) -> Result<Self, Error> {
        const fn get_pixel_type(
            color_type: &png::ColorType,
            bit_depth: &png::BitDepth,
            eight: fast_image_resize::PixelType,
            sixteen: fast_image_resize::PixelType,
        ) -> Result<fast_image_resize::PixelType, crate::PngError> {
            match bit_depth {
                png::BitDepth::Eight => Ok(eight),
                png::BitDepth::Sixteen => Ok(sixteen),
                _ => Err(crate::PngError::InvalidColorBitDepth(
                    *color_type,
                    *bit_depth,
                )),
            }
        }

        let fast_image_resize = match color_type {
            png::ColorType::Grayscale => get_pixel_type(
                color_type,
                bit_depth,
                fast_image_resize::PixelType::U8,
                fast_image_resize::PixelType::U16,
            ),
            png::ColorType::GrayscaleAlpha => get_pixel_type(
                color_type,
                bit_depth,
                fast_image_resize::PixelType::U8x2,
                fast_image_resize::PixelType::U16x2,
            ),
            png::ColorType::Rgb => get_pixel_type(
                color_type,
                bit_depth,
                fast_image_resize::PixelType::U8x3,
                fast_image_resize::PixelType::U16x3,
            ),
            png::ColorType::Rgba => get_pixel_type(
                color_type,
                bit_depth,
                fast_image_resize::PixelType::U8x4,
                fast_image_resize::PixelType::U16x4,
            ),
            png::ColorType::Indexed => Err(crate::PngError::IndexedColorType),
        }
        .map_err(Error::Png)?;
        let blittle = Self::get_blittle_pixel_type(fast_image_resize)?;
        Ok(Self {
            fast_image_resize,
            blittle,
            png: PngPixelType {
                color_type: *color_type,
                bit_depth: *bit_depth,
            },
        })
    }

    const fn get_blittle_pixel_type(
        pixel_type: fast_image_resize::PixelType,
    ) -> Result<blittle::PixelType, Error> {
        match pixel_type {
            fast_image_resize::PixelType::U8 => Ok(blittle::PixelType::Gr8),
            fast_image_resize::PixelType::U8x2 => Ok(blittle::PixelType::GrA8),
            fast_image_resize::PixelType::U8x3 => Ok(blittle::PixelType::Rgb8),
            fast_image_resize::PixelType::U8x4 => Ok(blittle::PixelType::Rgba8),
            fast_image_resize::PixelType::F32 => Ok(blittle::PixelType::Gr32),
            fast_image_resize::PixelType::F32x3 => Ok(blittle::PixelType::Rgb32),
            fast_image_resize::PixelType::F32x4 => Ok(blittle::PixelType::Rgba32),
            other => Err(Error::UnsupportedPixelType(other)),
        }
    }

    /// Get a `PngPixelType` from a `blittle::PixelType`.
    /// Some blittle pixel types are not allowed.
    #[cfg(feature = "png")]
    const fn get_png_pixel_type(
        pixel_type: &blittle::PixelType,
    ) -> Result<PngPixelType, crate::PngError> {
        match pixel_type {
            blittle::PixelType::Gr8 => Ok(PngPixelType {
                color_type: png::ColorType::Grayscale,
                bit_depth: png::BitDepth::Eight,
            }),
            blittle::PixelType::GrA8 => Ok(PngPixelType {
                color_type: png::ColorType::GrayscaleAlpha,
                bit_depth: png::BitDepth::Eight,
            }),
            blittle::PixelType::Rgb8 => Ok(PngPixelType {
                color_type: png::ColorType::Rgb,
                bit_depth: png::BitDepth::Eight,
            }),
            blittle::PixelType::Rgba8 => Ok(PngPixelType {
                color_type: png::ColorType::Rgba,
                bit_depth: png::BitDepth::Eight,
            }),
            other => Err(crate::PngError::BlittlePixelType(*other)),
        }
    }
}

#[cfg(test)]
mod tests {}
