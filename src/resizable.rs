use fast_image_resize::PixelType;

/// Used internal to get a `PixelType` from a pixel.
pub trait ResizablePixel {
    fn get_pixel_type() -> PixelType;
}

macro_rules! impl_single {
    ($t:path, $p:ident) => {
        impl ResizablePixel for $t {
            fn get_pixel_type() -> PixelType {
                PixelType::$p
            }
        }
    };
}

macro_rules! impl_array {
    ($t:ty, $len:literal, $p:ident) => {
        impl ResizablePixel for [$t; $len] {
            fn get_pixel_type() -> PixelType {
                PixelType::$p
            }
        }
    };
}

impl_single!(u8, U8);
impl_single!(f32, F32);

impl_array!(u8, 2, U8x2);
impl_array!(u8, 3, U8x3);
impl_array!(u8, 4, U8x4);
impl_array!(f32, 2, F32x2);
impl_array!(f32, 4, F32x4);

#[cfg(feature = "softbuffer")]
impl_single!(blittle::sb::Zrgb, U8x4);
