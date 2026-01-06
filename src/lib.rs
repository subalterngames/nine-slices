mod border_scaling;
mod error;
mod rect;
mod scale;
mod sprite;
mod bitmap;
mod borders;
mod corners;

use std::io::Read;
pub use bitmap::Bitmap;
pub use blittle;
use blittle::*;
pub use border_scaling::BorderScaling;
use bytemuck::cast_slice;
use fast_image_resize::{CropBoxError, ImageViewMut, IntoImageView, IntoImageViewMut, PixelTrait};
use fast_image_resize::images::CroppedImage;
pub use error::Error;
pub use fast_image_resize::images::Image;
pub use glam::UVec2;
pub use rect::Rect;
pub use scale::Scale;
pub use sprite::Sprite;
pub use borders::Borders;
use corners::*;

pub struct NineSlicedSprite<'s> {
    image: Image<'s>,
    rect: Rect,
    borders: Borders,
    corners: Corners,
}


impl<'s> NineSlicedSprite<'s> {
    pub fn new_from_vec(buffer: Vec<u8>, borders: Borders, rect: Rect, pixel_type: PixelType) -> Result<Self, Error> {
        let size = rect.image_size();
        let image = Image::from_vec_u8(size.x, size.y, buffer, Self::convert_pixel_type(&pixel_type)).map_err(Error::FromVec)?;
        let corners = Corners::new(&borders, &rect);
        Ok(Self {
            image,
            rect,
            borders,
            corners
        })
    }

    pub const fn new_from_image(image: Image<'s>, borders: Borders, rect: Rect) -> Self {
        let corners = Corners::new(&borders, &rect);
        Self {
            image,
            borders,
            rect,
            corners
        }
    }

    pub fn resize(&self, width: u32, height: u32) -> Result<Image<'_>, Error> {
        let mut dst = Image::new(width, height, self.image.pixel_type());
    }

    fn blit_corner<I: ImageViewMut>(src: &Image, dst: &mut Image, corner: Corner) -> Result<(), Error> {
        let src = CroppedImage::new(src, corner.left, corner.top, corner.width, corner.height).map_err(Error::Corner)?;
        src.borrow().
        Ok(())
    }

   /* pub fn resize(&self, size: Size) -> Result<Sprite, Error> {
        let mut dst = self.get_dst(&size);
        let outer_size = *self.params.outer.size();

        if size.w < outer_size.w || size.h < outer_size.h {
            return Err(Error::TooSmall {
                from: outer_size,
                to: size,
            });
        }

        let rect = ClippedRect::new(PositionI::default(), size, outer_size)
            .ok_or(Error::ClippedRect(size))?;

        // Top-left corner.
        let mut top_left = rect.clone();
        top_left.set_src_rect(
            PositionU::default(),
            Size {
                w: self.params.inner.position.x,
                h: self.params.inner.position.y,
            },
        );
        blit(&self.bitmap, &mut dst, &top_left, &self.pixel_type);

        // Top-right corner.
        let mut top_right = rect.clone();
        let w1 = outer_size.w - (self.params.inner.size.w - self.params.inner.position.x);
        let top_right_size = Size {
            w: w1,
            h: self.params.inner.position.y,
        };
        let x1 = size.w - top_right_size.w;
        top_right.set_src_rect(PositionU { x: x1, y: 0 }, top_right_size);
        blit(&self.bitmap, &mut dst, &top_right, &self.pixel_type);

        // Bottom-right corner.
        let mut bottom_right = rect.clone();
        let h1 = outer_size.h - (self.params.inner.size.h - self.params.inner.position.y);
        let bottom_right_size = Size { w: w1, h: h1 };
        let y1 = size.h - top_right_size.h;
        bottom_right.set_src_rect(PositionU { x: x1, y: y1 }, bottom_right_size);
        blit(&self.bitmap, &mut dst, &bottom_right, &self.pixel_type);

        // Bottom-left corner.
        let mut bottom_left = rect.clone();
        let bottom_left_size = Size {
            w: self.params.inner.position.x,
            h: h1,
        };
        bottom_right.set_src_rect(PositionU { x: 0, y: y1 }, bottom_left_size);
        blit(&self.bitmap, &mut dst, &bottom_left, &self.pixel_type);

        // Resize the entire sprite., accounting for the inner offset.
        let resize_to = Size {
            w: size.w + self.params.inner.position.x,
            h: size.h + self.params.inner.position.y,
        };
        let pixel_type = match &self.pixel_type {
            PixelType::Gr8 => fast_image_resize::PixelType::U8,
            PixelType::GrA8 => fast_image_resize::PixelType::U8x2,
            PixelType::Rgb8 => fast_image_resize::PixelType::U8x3,
            PixelType::Rgba8 => fast_image_resize::PixelType::U8x4,
            PixelType::Gr32 => fast_image_resize::PixelType::F32,
            PixelType::Rgb32 => fast_image_resize::PixelType::F32x3,
            PixelType::Rgba32 => fast_image_resize::PixelType::F32x4,
        };
        let resize_src = Image::from_slice_u8(outer_size.w as u32, resize_to.h as u32, pixel_type);
    }

    fn get_dst(&self, size: &Size) -> Vec<u8> {
        fn empty_f32(size: &Size, channels: usize) -> Vec<u8> {
            let v = vec![0.; size.w * size.h * channels];
            cast_slice::<f32, u8>(&v).to_vec()
        }

        match &self.pixel_type {
            PixelType::Gr8 | PixelType::GrA8 | PixelType::Rgb8 | PixelType::Rgba8 => {
                vec![0; size.w * size.h * self.pixel_type.stride()]
            }
            PixelType::Gr32 => empty_f32(size, 1),
            PixelType::Rgb32 => empty_f32(size, 3),
            PixelType::Rgba32 => empty_f32(size, 4),
        }
    }*/

    const fn convert_pixel_type(pixel_type: &PixelType) -> fast_image_resize::PixelType {
        match pixel_type {
            PixelType::Gr8 => fast_image_resize::PixelType::U8,
            PixelType::GrA8 => fast_image_resize::PixelType::U8x2,
            PixelType::Rgb8 => fast_image_resize::PixelType::U8x3,
            PixelType::Rgba8 => fast_image_resize::PixelType::U8x4,
            PixelType::Gr32 => fast_image_resize::PixelType::F32,
            PixelType::Rgb32 => fast_image_resize::PixelType::F32x3,
            PixelType::Rgba32 => fast_image_resize::PixelType::F32x4,
        }
    }
}