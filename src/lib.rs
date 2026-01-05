pub mod params;
mod sprite;

use crate::params::{Rect, Scale, ScaleMethod};
pub use blittle;
use blittle::{PixelType, PositionU};
use params::Params;
pub use sprite::Sprite;

pub struct NineSlicedSprite {
    bitmap: Vec<u8>,
    params: Params,
    pixel_type: PixelType,
}

impl NineSlicedSprite {
    pub fn new(bitmap: Vec<u8>, pixel_type: PixelType, params: Params) -> Self {
        Self {
            bitmap,
            pixel_type,
            params,
        }
    }
}
