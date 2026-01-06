use blittle::*;
use crate::bitmap::Bitmap;

/// A bitmap and a pixel type.
pub struct Sprite {
    pub bitmap: Bitmap,
    pub size: Size,
}
