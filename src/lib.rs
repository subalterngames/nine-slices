#![doc = include_str!("../doc/README-rust.md")]
//!
#![cfg_attr(all(),
    doc = embed_doc_image::embed_image!("src", "doc/images/src.png"),
    doc = embed_doc_image::embed_image!("sliced_and_scaled", "doc/images/sliced_and_scaled.png"),
    doc = embed_doc_image::embed_image!("scaled", "doc/images/scaled.png"))]

mod border_offsets;
mod border_scaling;
mod error;
mod nine_slices;
mod resizable;
mod resize_method;

use crate::resize_method::ResizeMethod;
use blittle::*;
pub use border_offsets::BorderOffsets;
pub use border_scaling::BorderScaling;
use bytemuck::{Pod, Zeroable};
pub use error::Error;
pub use fast_image_resize;
use fast_image_resize::images::CroppedImageMut;
use fast_image_resize::{ResizeAlg, ResizeOptions, Resizer, images::Image};
use nine_slices::NineSlices;
pub use resizable::ResizablePixel;
use resize_method::ResizeMethods;

/// A sprite sliced into a 3x3 grid that can be resized without distorting the corners.
pub struct NineSlicedSprite<
    's,
    S: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default + PartialEq + Zeroable + Pod + ResizablePixel,
> {
    surface: Surface<'s, S, P>,
    slices: NineSlices,
    border_scaling: BorderScaling,
    resizer: Resizer,
    resize_algorithm: ResizeAlg,
    resize_methods: ResizeMethods<P>,
}

impl<
    's,
    S: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default + PartialEq + Zeroable + Pod + ResizablePixel,
> NineSlicedSprite<'s, S, P>
{
    /// Slice `image` into a 3x3 grid using `offsets`.
    pub fn new(
        surface: Surface<'s, S, P>,
        offsets: BorderOffsets,
        border_scaling: BorderScaling,
    ) -> Result<Self, Error> {
        let slices = offsets.into_slices(surface.get_size())?;
        let resize_methods = ResizeMethods::new(&slices, &surface);
        Ok(Self {
            surface,
            slices,
            border_scaling,
            resizer: Resizer::new(),
            resize_algorithm: ResizeAlg::default(),
            resize_methods,
        })
    }

    /// Set the resize algorithm.
    ///
    /// The default algorithm is Lanczos3, which is slow but results in a high-quality resized image.
    /// The fastest option, with the worst quality, is `ResizeAlg::Nearest`.
    pub const fn set_resize_algorithm(&mut self, resize_algorithm: ResizeAlg) {
        self.resize_algorithm = resize_algorithm;
    }

    /// Resize the sprite to dimensions `(width, height)`.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<Surface<'s, Vec<P>, P>, Error> {
        let dst_size = Size {
            width: width as usize,
            height: height as usize,
        };

        // Create a new empty image.
        let mut dst = Surface::new_from_slice(dst_size);

        // Blit corners.
        self.blit_corners(&mut dst).map_err(Error::Blittle)?;

        // Resize and blit the inner area.
        self.resize_and_blit(
            PositionU {
                x: self.slices.top_left.size.width,
                y: self.slices.top_right.size.height,
            },
            self.slices.inner,
            self.resize_methods.inner,
            Size {
                width: dst_size.width
                    - (self.slices.top_left.size.width + self.slices.top_right.size.width),
                height: dst_size.height
                    - (self.slices.top_left.size.height + self.slices.bottom_left.size.height),
            },
            &mut dst,
        )?;

        // Resize the borders.
        match &self.border_scaling {
            BorderScaling::Stretch => self.stretch_borders(width, height, &mut dst)?,
            BorderScaling::Repeat => self.repeat_borders(width, height, &mut dst),
        }

        Ok(dst)
    }

    pub fn finish(self) -> Surface<'s, S, P> {
        self.surface
    }

    /// Blit an area of `src` defined by `src_rect` to `dst`.
    fn blit(
        &mut self,
        position: PositionU,
        area: RectU,
        dst: &mut Surface<'_, Vec<P>, P>,
    ) -> Result<(), blittle::Error> {
        // Set the blit area and destination rect.
        self.surface.set_position(position.into(), dst)?;
        self.surface.set_area(Some(area.into_recti()))?;
        // Blit.
        self.surface.blit(dst)
    }

    fn blit_corners(&mut self, dst: &mut Surface<'s, Vec<P>, P>) -> Result<(), blittle::Error> {
        let dst_size = dst.get_size();
        self.blit(self.slices.top_left.position, self.slices.top_left, dst)?;
        self.blit(
            PositionU {
                x: dst_size.width - self.slices.top_right.size.width,
                y: 0,
            },
            self.slices.top_right,
            dst,
        )?;
        self.blit(
            PositionU {
                x: dst_size.width - self.slices.bottom_right.size.width,
                y: dst_size.height - self.slices.bottom_right.size.height,
            },
            self.slices.bottom_right,
            dst,
        )?;
        self.blit(
            PositionU {
                x: 0,
                y: dst_size.height - self.slices.bottom_right.size.height,
            },
            self.slices.bottom_left,
            dst,
        )
    }

    fn resize_and_blit(
        &mut self,
        position: PositionU,
        area: RectU,
        method: ResizeMethod<P>,
        resize_to: Size,
        dst: &mut Surface<'s, Vec<P>, P>,
    ) -> Result<(), Error> {
        self.surface
            .set_position(position.into(), dst)
            .map_err(Error::Blittle)?;
        match method {
            // An obvious optimization!
            // The slice will sometimes contain a single color.
            // So, let's just create a new bitmap with that color.
            ResizeMethod::Fill(color) => {
                let dst_size = dst.get_size();
                let x0 = position.x;
                let x1 = (x0 + area.size.width).min(dst_size.width);
                let y0 = position.y;
                let y1 = (y0 + area.size.height).min(dst_size.height);
                for y in y0..y1 {
                    let i0 = dst.get_index(x0, y);
                    let i1 = dst.get_index(x1, y);
                    dst.buffer_mut()[i0..i1].fill(color);
                }
                Ok(())
            }
            ResizeMethod::Resize => {
                self.fast_resize(area, resize_to, dst)?;
                Ok(())
            }
        }
    }

    fn fast_resize(
        &mut self,
        area: RectU,
        resize_to: Size,
        dst: &mut Surface<'s, Vec<P>, P>,
    ) -> Result<(), Error> {
        // Get the position.
        let position = self
            .surface
            .get_position()
            .ok_or(Error::Blittle(blittle::Error::NoDestinationRect))?;
        let dst_size = dst.get_size();
        let src_size = self.surface.get_size();
        let pixel_type = P::get_pixel_type();
        let src = Image::from_slice_u8(
            src_size.width as u32,
            src_size.height as u32,
            self.surface.bytes_mut(),
            pixel_type,
        )
        .map_err(Error::FromSlice)?;
        // Get an image referencing the buffer.
        let mut dst = Image::from_slice_u8(
            dst_size.width as u32,
            dst_size.height as u32,
            dst.bytes_mut(),
            pixel_type,
        )
        .map_err(Error::FromSlice)?;
        // The resized image is a cropped view of the destination image.
        let mut resized = CroppedImageMut::new(
            &mut dst,
            position.x as u32,
            position.y as u32,
            resize_to.width as u32,
            resize_to.height as u32,
        )
        .map_err(Error::Crop)?;
        // Crop the source image.
        let options = ResizeOptions::new()
            .crop(
                area.position.x as f64,
                area.position.y as f64,
                area.size.width as f64,
                area.size.height as f64,
            )
            .resize_alg(self.resize_algorithm);
        // Resize the cropped image into `resized`.
        self.resizer
            .resize(&src, &mut resized, Some(&options))
            .map_err(Error::Resize)
    }

    /// Resize the borders by stretching them.
    /// `width` and `height` are the dimensions of `dst`.
    fn stretch_borders(
        &mut self,
        width: u32,
        height: u32,
        dst: &mut Surface<'s, Vec<P>, P>,
    ) -> Result<(), Error> {
        let total_dst_size = Size {
            width: width as usize,
            height: height as usize,
        };

        let width =
            total_dst_size.width - (self.slices.left.size.width + self.slices.right.size.width);
        let height =
            total_dst_size.height - (self.slices.top.size.height + self.slices.bottom.size.height);

        self.resize_and_blit(
            self.slices.top.position,
            self.slices.top,
            self.resize_methods.top,
            Size {
                width,
                height: self.slices.top.size.height,
            },
            dst,
        )?;

        self.resize_and_blit(
            PositionU {
                x: total_dst_size.width - self.slices.right.size.width,
                y: self.slices.top.size.height,
            },
            self.slices.right,
            self.resize_methods.right,
            Size {
                width: self.slices.right.size.width,
                height,
            },
            dst,
        )?;

        self.resize_and_blit(
            PositionU {
                x: self.slices.bottom.position.x,
                y: total_dst_size.height - self.slices.bottom.size.height,
            },
            self.slices.bottom,
            self.resize_methods.bottom,
            Size {
                width,
                height: self.slices.bottom.size.height,
            },
            dst,
        )?;

        self.resize_and_blit(
            self.slices.left.position,
            self.slices.left,
            self.resize_methods.left,
            Size {
                width: self.slices.left.size.width,
                height,
            },
            dst,
        )?;

        Ok(())
    }

    // Resize by repeatedly blitting the source bitmap's borders.
    // `width` and `height` are the dimensions of `dst`.
    fn repeat_borders(&mut self, width: u32, height: u32, dst: &mut Surface<'s, Vec<P>, P>) {
        let dst_w = width as usize;
        let dst_h = height as usize;
        let border_w = dst_w - (self.slices.top_left.size.width + self.slices.top_right.size.width);
        let border_h =
            dst_h - (self.slices.top_left.size.height + self.slices.top_right.size.height);
        // Left.
        self.repeat_vertical(self.slices.left.position, self.slices.left, border_h, dst);
        // Top.
        self.repeat_horizontal(self.slices.top.position, self.slices.top, border_w, dst);
        // Right.
        self.repeat_vertical(
            PositionU {
                x: dst_w - self.slices.right.size.width,
                y: self.slices.right.position.y,
            },
            self.slices.right,
            border_h,
            dst,
        );
        // Bottom.
        self.repeat_horizontal(
            PositionU {
                x: self.slices.bottom.position.x,
                y: dst_h - self.slices.bottom.size.height,
            },
            self.slices.bottom,
            border_w,
            dst,
        );
    }

    /// Resize a horizontal edge by repeating it.
    ///
    /// - `src_rect` is the size of the source slice.
    /// - `dst_position` is the position of the edge on the `dst` bitmap.
    /// - `border_w` is the width of the resized border.
    /// - `dst_w` is the width of `dst`.
    fn repeat_horizontal(
        &self,
        position: PositionU,
        src_rect: RectU,
        edge_w: usize,
        dst: &mut Surface<'s, Vec<P>, P>,
    ) {
        // Rows.
        for y in 0..src_rect.size.height {
            // The y coordinate on the source bitmap.
            let src_y = src_rect.position.y + y;
            // The starting index in the source slice.
            let s0 = self.surface.get_index(src_rect.position.x, src_y);
            // The y coordinate on the destination bitmap.
            let dst_y = position.y + y;
            // The destination slice's start and end indices.
            let d0 = dst.get_index(position.x, dst_y);
            let d1 = dst.get_index(position.x + edge_w, dst_y);
            // Blit slices of `src` onto chunks of `dst`.
            dst.buffer_mut()[d0..d1]
                .chunks_mut(src_rect.size.width)
                .for_each(|chunk| {
                    chunk.copy_from_slice(&self.surface.buffer()[s0..s0 + chunk.len()]);
                });
        }
    }

    /// Resize a vertical edge by repeating it.
    ///
    /// - `src_rect` is the size of the source slice.
    /// - `dst_position` is the position of the edge on the `dst` bitmap.
    /// - `border_h` is the width of the resized border.
    /// - `dst_w` is the width of `dst`.
    fn repeat_vertical(
        &self,
        position: PositionU,
        src_rect: RectU,
        border_height: usize,
        dst: &mut Surface<'s, Vec<P>, P>,
    ) {
        for y in 0..border_height {
            // The y coordinate on the destination bitmap.
            let dst_y = position.y + y;
            // The y coordinate on the source bitmap. Use modulus division to repeat the blit.
            let src_y = src_rect.position.y + dst_y % src_rect.size.height;
            // Source horizontal slice.
            let s0 = self.surface.get_index(src_rect.position.x, src_y);
            let s1 = self
                .surface
                .get_index(src_rect.position.x + src_rect.size.width, src_y);
            // Destination horizontal slice.
            let d0 = dst.get_index(position.x, dst_y);
            let d1 = dst.get_index(position.x + src_rect.size.width, dst_y);
            // Blit.
            dst.buffer_mut()[d0..d1].copy_from_slice(&self.surface.buffer()[s0..s1]);
        }
    }
}

#[cfg(feature = "png")]
#[cfg(test)]
mod tests {
    use super::*;
    use blittle::png::Png;
    use std::io::Cursor;

    macro_rules! resize {
        ($filename:literal, $scaling:ident) => {{
            let slices = BorderOffsets {
                left: 32,
                top: 32,
                right: 32,
                bottom: 32,
            };
            let src = Rgba8Surface::read_png(Cursor::new(include_bytes!(concat!(
                "../test_files/src/",
                $filename,
                ".png"
            ))))
            .unwrap();
            let mut sprite = NineSlicedSprite::new(src, slices, BorderScaling::$scaling).unwrap();

            let width = 1024;
            let height = 768;
            let dst = sprite.resize(width, height).unwrap();
            Rgba8Surface::write_png(&dst, format!("test_files/dst/{}.png", $filename)).unwrap();
        }};
    }

    #[test]
    fn test_stretch() {
        resize!("stretch", Stretch);
    }

    #[test]
    fn test_repeat() {
        resize!("repeat", Repeat);
    }
}
