use crate::pixel_type::PixelType;
use blittle::{PositionU, Size, get_index};
use std::slice::from_raw_parts_mut;

macro_rules! fill8 {
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

macro_rules! fill32 {
    ($color:ident, $size:ident, $channels:literal) => {{
        let mut bitmap = bitmap($size, $channels * 4);
        let ptr = bitmap.as_mut_ptr().cast::<[f32; $channels]>();
        let len = bitmap.len() / ($channels * 4);
        unsafe {
            from_raw_parts_mut(ptr, len).fill(*$color);
        }
        bitmap
    }};
}

pub enum FillColor {
    Gr8(u8),
    GrA8([u8; 2]),
    Rgb8([u8; 3]),
    Rgba8([u8; 4]),
    Gr32(f32),
    Rgb32([f32; 3]),
    Rgba32([f32; 4]),
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
            blittle::PixelType::Gr32 => Self::Gr32(Self::float(src, i)),
            blittle::PixelType::Rgb32 => {
                let r = Self::float(src, i);
                let g = Self::float(src, i + 4);
                let b = Self::float(src, i + 8);
                Self::Rgb32([r, g, b])
            }
            blittle::PixelType::Rgba32 => {
                let r = Self::float(src, i);
                let g = Self::float(src, i + 4);
                let b = Self::float(src, i + 8);
                let a = Self::float(src, i + 12);
                Self::Rgba32([r, g, b, a])
            }
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
            Self::GrA8(color) => fill8!(color, size, 2),
            Self::Rgb8(color) => fill8!(color, size, 3),
            Self::Rgba8(color) => fill8!(color, size, 4),
            Self::Gr32(color) => {
                let mut bitmap = bitmap(size, 4);
                let ptr = bitmap.as_mut_ptr().cast::<f32>();
                let len = bitmap.len() / 4;
                unsafe {
                    from_raw_parts_mut(ptr, len).fill(*color);
                }
                bitmap
            }
            Self::Rgb32(color) => fill32!(color, size, 3),
            Self::Rgba32(color) => fill32!(color, size, 4),
        }
    }

    /// Get an f32 in `src` that starts at index `i`.
    const fn float(src: &[u8], i: usize) -> f32 {
        f32::from_le_bytes([src[i], src[i + 1], src[i + 2], src[i + 3]])
    }

    const fn rgb32(src: &[u8], i: usize) -> (f32, f32, f32) {
        let r = Self::float(src, i);
        let g = Self::float(src, i + 4);
        let b = Self::float(src, i + 8);
        (r, g, b)
    }
}
