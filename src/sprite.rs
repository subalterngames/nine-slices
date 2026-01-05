use blittle::*;

/// A raw bitmap and its properties.
pub struct Sprite {
    pub bitmap: Vec<u8>,
    pub pixel_type: PixelType,
    pub size: Size,
}
