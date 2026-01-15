use crate::Error;
use fast_image_resize::images::Image;

/// Pixel types as expressed in different crates.
pub struct PixelType {
    pub blittle: blittle::PixelType,
    pub fast_image_resize: fast_image_resize::PixelType,
}

impl PixelType {
    pub fn new(image: &Image) -> Result<Self, Error> {
        let fast_image_resize = image.pixel_type();
        let blittle = Self::get_blittle_pixel_type(fast_image_resize)?;
        Ok(Self {
            blittle,
            fast_image_resize,
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
}
