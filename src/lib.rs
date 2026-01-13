mod border_scaling;
mod error;
mod slices;

use blittle::{ClippedRect, PositionI, Size, blit};
pub use border_scaling::BorderScaling;
pub use error::Error;
pub use fast_image_resize::{PixelType, images::Image};
use fast_image_resize::{ResizeOptions, Resizer};
pub use slices::*;

pub struct NineSlicedSprite<'s> {
    image: Image<'s>,
    slices: Slices,
    border_scaling: BorderScaling,
    resizer: Resizer,
}

impl<'s> NineSlicedSprite<'s> {
    pub fn new_from_image(image: Image<'s>, slices: Slices, border_scaling: BorderScaling) -> Self {
        Self {
            image,
            slices,
            border_scaling,
            resizer: Resizer::new(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<Image<'_>, Error> {
        let pixel_type = self.image.pixel_type();

        let src = self.image.buffer();

        // Create a new empty image.
        let mut dst = Image::new(width, height, pixel_type);
        let dst_buffer = dst.buffer_mut();

        let clipped_rect = ClippedRect::new(
            PositionI::default(),
            Size {
                w: width as usize,
                h: height as usize,
            },
            Size {
                w: self.image.width() as usize,
                h: self.image.height() as usize,
            },
        )
        .ok_or(Error::InvalidClippedRect)?;

        // Convert to a blittle pixel type.
        let pixel_type_blittle = Self::get_blittle_pixel_type(pixel_type)?;
        // Blit corners.
        Self::blit(
            src,
            dst_buffer,
            &self.slices.top_left,
            &clipped_rect,
            &pixel_type_blittle,
        );
        Self::blit(
            src,
            dst_buffer,
            &self.slices.top_right,
            &clipped_rect,
            &pixel_type_blittle,
        );
        Self::blit(
            src,
            dst_buffer,
            &self.slices.bottom_right,
            &clipped_rect,
            &pixel_type_blittle,
        );
        Self::blit(
            src,
            dst_buffer,
            &self.slices.bottom_left,
            &clipped_rect,
            &pixel_type_blittle,
        );

        // Resize the inner area.
        let options = ResizeOptions::new().crop(
            self.slices.left.size.w as f64,
            self.slices.top.size.h as f64,
            self.slices.right.size.w as f64,
            self.slices.bottom.size.h as f64,
        );
        let mut resized_inner = Image::new(
            width - (self.slices.left.size.w + self.slices.right.size.w) as u32,
            height - (self.slices.top.size.h + self.slices.bottom.size.h) as u32,
            pixel_type,
        );
        self.resizer
            .resize(&self.image, &mut resized_inner, Some(&options))
            .map_err(Error::ResizeInner)?;

        // Blit the inner area.
        let mut clipped_rect = clipped_rect;
        clipped_rect.set_src_rect(
            self.slices.inner.position,
            Size {
                w: resized_inner.width() as usize,
                h: resized_inner.height() as usize,
            },
        );
        blit(
            resized_inner.buffer(),
            dst_buffer,
            &clipped_rect,
            &pixel_type_blittle,
        );

        match &self.border_scaling {
            BorderScaling::Repeat => todo!(),
            BorderScaling::Stretch => self.stretch_edges(dst_buffer)?
        }

        Ok(dst)
    }

    pub const fn get_blittle_pixel_type(
        pixel_type: PixelType,
    ) -> Result<blittle::PixelType, Error> {
        match pixel_type {
            PixelType::U8 => Ok(blittle::PixelType::Gr8),
            PixelType::U8x2 => Ok(blittle::PixelType::GrA8),
            PixelType::U8x3 => Ok(blittle::PixelType::Rgb8),
            PixelType::U8x4 => Ok(blittle::PixelType::Rgba8),
            PixelType::F32 => Ok(blittle::PixelType::Gr32),
            PixelType::F32x3 => Ok(blittle::PixelType::Rgb32),
            PixelType::F32x4 => Ok(blittle::PixelType::Rgba32),
            other => Err(Error::UnsupportedPixelType(other)),
        }
    }

    fn blit(
        src: &[u8],
        dst: &mut [u8],
        rect: &Rect,
        clipped_rect: &ClippedRect,
        pixel_type: &blittle::PixelType,
    ) {
        let mut clipped_rect = *clipped_rect;
        clipped_rect.set_src_rect(rect.position, rect.size);
        blit(src, dst, &clipped_rect, pixel_type);
    }

    fn stretch_edges(&self, dst: &mut [u8]) -> Result<(), Error> {

    }

    fn stretch_edge(&self, edge: &Rect, dst: &mut [u8]) -> Result<(), Error> {
        // Resize the inner area.
        let options = ResizeOptions::new().crop(
            edge.size.w as f64,
            edge.size.h as f64,
            edge.size.w as f64,
            edge.size.h as f64,
        );
        let mut resized_inner = Image::new(
            width - (self.slices.left.size.w + self.slices.right.size.w) as u32,
            height - (self.slices.top.size.h + self.slices.bottom.size.h) as u32,
            pixel_type,
        );
        self.resizer
            .resize(&self.image, &mut resized_inner, Some(&options))
            .map_err(Error::ResizeInner)?;
    }
}
