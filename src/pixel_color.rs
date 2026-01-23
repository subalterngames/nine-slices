use crate::pixel_type::PixelType;
use blittle::{PositionU, Size, get_index};
use std::slice::from_raw_parts_mut;

macro_rules! fill {
    ($color:ident, $size:ident, $len:literal) => {{
        let mut bitmap = vec![0; $size.w * $size.h * $len];
        let ptr = bitmap.as_mut_ptr().cast::<[u8; $len]>();
        let len = bitmap.len() / $len;
        // Technically it's unsafe, but it's used only internally, so it's probably fine.
        unsafe {
            from_raw_parts_mut(ptr, len).fill(*$color);
        }
        bitmap
    }};
}

/// A pixel type and a color.
/// All values are u8 arrays because for the purposes of this crate,
/// we don't need to cast to the correct type.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PixelColor {
    One(u8),
    Two([u8; 2]),
    Three([u8; 3]),
    Four([u8; 4]),
    Twelve([u8; 12]),
    Sixteen([u8; 16]),
}

impl PixelColor {
    /// Returns the pixel color at `position` in `src`.
    pub const fn at_position(
        position: PositionU,
        width: usize,
        pixel_type: &PixelType,
        src: &[u8],
    ) -> Self {
        let stride = pixel_type.blittle.stride();
        let i = get_index(position.x, position.y, width, stride);
        match &pixel_type.blittle {
            blittle::PixelType::Gr8 => Self::One(src[i]),
            blittle::PixelType::GrA8 => Self::Two([src[i], src[i + 1]]),
            blittle::PixelType::Rgb8 => Self::Three([src[i], src[i + 1], src[i + 2]]),
            blittle::PixelType::Rgba8 | blittle::PixelType::Gr32 => {
                Self::Four([src[i], src[i + 1], src[i + 2], src[i + 3]])
            }
            blittle::PixelType::Rgb32 => Self::Twelve([
                src[i],
                src[i + 1],
                src[i + 2],
                src[i + 3],
                src[i + 4],
                src[i + 5],
                src[i + 6],
                src[i + 7],
                src[i + 8],
                src[i + 9],
                src[i + 10],
                src[i + 11],
            ]),
            blittle::PixelType::Rgba32 => Self::Sixteen([
                src[i],
                src[i + 1],
                src[i + 2],
                src[i + 3],
                src[i + 4],
                src[i + 5],
                src[i + 6],
                src[i + 7],
                src[i + 8],
                src[i + 9],
                src[i + 10],
                src[i + 11],
                src[i + 12],
                src[i + 13],
                src[i + 14],
                src[i + 15],
            ]),
        }
    }

    pub fn get_filled(&self, size: Size) -> Vec<u8> {
        let stride = match self {
            Self::One(_) => 1,
            Self::Two(_) => 2,
            Self::Three(_) => 3,
            Self::Four(_) => 4,
            Self::Twelve(_) => 12,
            Self::Sixteen(_) => 16,
        };
        match self {
            Self::One(color) => vec![*color; size.w * size.h * stride],
            Self::Two(color) => fill!(color, size, 2),
            Self::Three(color) => fill!(color, size, 3),
            Self::Four(color) => fill!(color, size, 4),
            Self::Twelve(color) => fill!(color, size, 12),
            Self::Sixteen(color) => fill!(color, size, 16),
        }
    }
}
