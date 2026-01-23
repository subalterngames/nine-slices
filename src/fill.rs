use crate::pixel_type::PixelType;
use blittle::{PositionU, Size, get_index};
use std::slice::from_raw_parts_mut;

macro_rules! fill {
    ($color:ident, $size:ident, $channels:literal) => {{
        let mut bitmap = bitmap($size, $channels);
        let ptr = bitmap.as_mut_ptr().cast::<[u8; $channels]>();
        let len = bitmap.len() / $channels;
        unsafe {
            from_raw_parts_mut(ptr, len).fill(*$color);
        }
        bitmap
    }};
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FillColor {
    Gr8(u8),
    GrA8([u8; 2]),
    Rgb8([u8; 3]),
    Rgba8([u8; 4]),
    Gr32([u8; 4]),
    Rgb32([u8; 12]),
    Rgba32([u8; 16]),
}

impl FillColor {
    pub const fn from_pixel(
        position: PositionU,
        width: usize,
        pixel_type: &PixelType,
        src: &[u8],
    ) -> Self {
        let stride = pixel_type.blittle.stride();
        let i = get_index(position.x, position.y, width, stride);
        match &pixel_type.blittle {
            blittle::PixelType::Gr8 => Self::Gr8(src[i]),
            blittle::PixelType::GrA8 => Self::GrA8([src[i], src[i + 1]]),
            blittle::PixelType::Rgb8 => Self::Rgb8([src[i], src[i + 1], src[i + 2]]),
            blittle::PixelType::Rgba8 => Self::Rgba8([src[i], src[i + 1], src[i + 2], src[i + 3]]),
            blittle::PixelType::Gr32 => Self::Gr32([src[i], src[i + 1], src[i + 2], src[i + 3]]),
            blittle::PixelType::Rgb32 => Self::Rgb32([
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
            blittle::PixelType::Rgba32 => Self::Rgba32([
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
        fn bitmap(size: Size, stride: usize) -> Vec<u8> {
            vec![0; size.w * size.h * stride]
        }

        let stride = match self {
            Self::Gr8(_) => 1,
            Self::GrA8(_) => 2,
            Self::Rgb8(_) => 3,
            Self::Rgba8(_) | Self::Gr32(_) => 4,
            Self::Rgb32(_) => 12,
            Self::Rgba32(_) => 16,
        };
        match self {
            Self::Gr8(color) => vec![*color; size.w * size.h * stride],
            Self::GrA8(color) => fill!(color, size, 2),
            Self::Rgb8(color) => fill!(color, size, 3),
            Self::Rgba8(color) | Self::Gr32(color) => fill!(color, size, 4),
            Self::Rgb32(color) => fill!(color, size, 12),
            Self::Rgba32(color) => fill!(color, size, 16),
        }
    }
}
