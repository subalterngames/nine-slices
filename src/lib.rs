mod border_offsets;
mod border_scaling;
mod error;
mod nine_slices;
mod pixel_type;
mod rect;

use blittle::{ClippedRect, PositionU, Size, blit};
pub use border_offsets::BorderOffsets;
pub use border_scaling::BorderScaling;
pub use error::Error;
#[cfg(feature = "png")]
pub use error::PngError;
pub use fast_image_resize;
use fast_image_resize::images::Image;
use fast_image_resize::{ResizeOptions, Resizer};
use nine_slices::NineSlices;
use pixel_type::PixelType;
pub use rect::Rect;
#[cfg(feature = "png")]
use std::io::{BufRead, Seek, Write};

pub struct NineSlicedSprite<'s> {
    image: Image<'s>,
    pixel_type: PixelType,
    slices: NineSlices,
    border_scaling: BorderScaling,
    resizer: Resizer,
}

impl<'s> NineSlicedSprite<'s> {
    /// Slice an in-memory `image` using `offsets`.
    /// `border_scaling` defines how borders are scaled.
    pub fn new(
        image: Image<'s>,
        offsets: BorderOffsets,
        border_scaling: BorderScaling,
    ) -> Result<Self, Error> {
        let pixel_type = PixelType::new(&image)?;
        let slices = offsets.into_slices(Size {
            w: image.width() as usize,
            h: image.height() as usize,
        })?;
        Ok(Self {
            image,
            pixel_type,
            slices,
            border_scaling,
            resizer: Resizer::new(),
        })
    }

    /// Load a .png file and then slice it using `offsets`.
    /// `border_scaling` defines how borders are scaled.
    #[cfg(feature = "png")]
    pub fn from_png<R: BufRead + Seek>(
        r: R,
        offsets: BorderOffsets,
        border_scaling: BorderScaling,
    ) -> Result<Self, Error> {
        // Boilerplate .png decoding.
        let decoder = png::Decoder::new(r);
        let mut reader = decoder
            .read_info()
            .map_err(|e| Error::Png(PngError::Info(e)))?;
        let mut buf = vec![
            0;
            reader
                .output_buffer_size()
                .ok_or(Error::Png(PngError::OutputBufferSize))?
        ];
        let info = reader
            .next_frame(&mut buf)
            .map_err(|e| Error::Png(PngError::Frame(e)))?;
        let bytes = buf[..info.buffer_size()].to_vec();

        // Derive the pixel type.
        let pixel_type = PixelType::from_png(&info.color_type, &info.bit_depth)?;

        // Convert to `Image`.
        let image =
            Image::from_vec_u8(info.width, info.height, bytes, pixel_type.fast_image_resize)
                .map_err(Error::FromVec)?;

        // Convert border offsets to slices.
        let slices = offsets.into_slices(Size {
            w: image.width() as usize,
            h: image.height() as usize,
        })?;

        Ok(Self {
            image,
            pixel_type,
            slices,
            border_scaling,
            resizer: Resizer::new(),
        })
    }

    /// Resize the sprite to dimensions `(width, height)`.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<Image<'_>, Error> {
        let src = self.image.buffer();

        // Create a new empty image.
        let mut dst = Image::new(width, height, self.pixel_type.fast_image_resize);
        let dst_buffer = dst.buffer_mut();

        let dst_size = Size {
            w: width as usize,
            h: height as usize,
        };

        // Blit corners.
        self.blit(
            src,
            &self.slices.top_left,
            dst_buffer,
            Rect {
                position: self.slices.top_left.position,
                size: dst_size,
            },
        )?;
        self.blit(
            src,
            &self.slices.top_right,
            dst_buffer,
            Rect {
                position: PositionU {
                    x: dst_size.w - self.slices.top_right.size.w,
                    y: 0,
                },
                size: dst_size,
            },
        )?;
        self.blit(
            src,
            &self.slices.bottom_right,
            dst_buffer,
            Rect {
                position: PositionU {
                    x: dst_size.w - self.slices.bottom_right.size.w,
                    y: dst_size.h - self.slices.bottom_right.size.h,
                },
                size: dst_size,
            },
        )?;
        self.blit(
            src,
            &self.slices.bottom_left,
            dst_buffer,
            Rect {
                position: PositionU {
                    x: 0,
                    y: dst_size.h - self.slices.bottom_right.size.h,
                },
                size: dst_size,
            },
        )?;

        // Resize and blit the inner area.
        self.resize_and_blit(
            self.slices.inner,
            Size {
                w: dst_size.w - (self.slices.top_left.size.w + self.slices.top_right.size.w),
                h: dst_size.h - (self.slices.top_left.size.h + self.slices.bottom_left.size.h),
            },
            Rect {
                position: PositionU {
                    x: self.slices.top_left.size.w,
                    y: self.slices.top_right.size.h,
                },
                size: dst_size,
            },
            dst_buffer,
        )?;

        // Resize the borders.
        match &self.border_scaling {
            BorderScaling::Stretch => {
                self.stretch_edges(width, height, dst_buffer)?;
            }
            BorderScaling::Repeat => todo!(),
        }

        Ok(dst)
    }

    /// Write a resized image as a .png
    #[cfg(feature = "png")]
    pub fn write<W: Write>(image: &Image<'_>, w: W) -> Result<(), Error> {
        let pixel_type = PixelType::new(image)?;
        let mut encoder = png::Encoder::new(w, image.width(), image.height());
        encoder.set_color(pixel_type.png.color_type);
        encoder.set_depth(pixel_type.png.bit_depth);
        let mut writer = encoder
            .write_header()
            .map_err(|e| Error::Png(PngError::WriteHeader(e)))?;
        writer
            .write_image_data(image.buffer())
            .map_err(|e| Error::Png(PngError::WritePng(e)))
    }

    /// Blit an area of `src` defined by `src_rect` to `dst`.
    fn blit(
        &self,
        src: &[u8],
        src_rect: &Rect,
        dst: &mut [u8],
        dst_rect: Rect,
    ) -> Result<(), Error> {
        let mut clipped_rect =
            ClippedRect::new(dst_rect.position.into(), dst_rect.size, self.slices.size)
                .ok_or(Error::InvalidClippedRect)?;
        clipped_rect.src_size_clipped = src_rect.size;
        clipped_rect.set_src_rect(src_rect.position, src_rect.size);
        blit(src, dst, &clipped_rect, &self.pixel_type.blittle);
        Ok(())
    }

    fn resize_and_blit(
        &mut self,
        src_rect: Rect,
        resize_to: Size,
        dst_rect: Rect,
        dst: &mut [u8],
    ) -> Result<(), Error> {
        // Resize.
        let options = ResizeOptions::new().crop(
            src_rect.position.x as f64,
            src_rect.position.y as f64,
            src_rect.size.w as f64,
            src_rect.size.h as f64,
        );
        let mut resized = Image::new(
            resize_to.w as u32,
            resize_to.h as u32,
            self.pixel_type.fast_image_resize,
        );
        self.resizer
            .resize(&self.image, &mut resized, Some(&options))
            .map_err(Error::Resize)?;
        // Blit.
        let clipped_rect = ClippedRect::new(dst_rect.position.into(), dst_rect.size, resize_to)
            .ok_or(Error::InvalidClippedRect)?;
        blit(
            resized.buffer(),
            dst,
            &clipped_rect,
            &self.pixel_type.blittle,
        );
        Ok(())
    }

    /// Resize the edges.
    /// `width` and `height` are the dimensions of `dst`.
    fn stretch_edges(&mut self, width: u32, height: u32, dst: &mut [u8]) -> Result<(), Error> {
        let total_dst_size = Size {
            w: width as usize,
            h: height as usize,
        };

        let w = total_dst_size.w - (self.slices.left.size.w + self.slices.right.size.w);
        let h = total_dst_size.h - (self.slices.top.size.h + self.slices.bottom.size.h);

        self.resize_and_blit(
            self.slices.top,
            Size {
                w,
                h: self.slices.top.size.h,
            },
            Rect {
                position: self.slices.top.position,
                size: total_dst_size,
            },
            dst,
        )?;

        self.resize_and_blit(
            self.slices.right,
            Size {
                w: self.slices.right.size.w,
                h,
            },
            Rect {
                position: PositionU {
                    x: total_dst_size.w - self.slices.right.size.w,
                    y: self.slices.top.size.h,
                },
                size: total_dst_size,
            },
            dst,
        )?;

        self.resize_and_blit(
            self.slices.bottom,
            Size {
                w,
                h: self.slices.bottom.size.h,
            },
            Rect {
                position: PositionU {
                    x: self.slices.bottom.position.x,
                    y: total_dst_size.h - self.slices.bottom.size.h,
                },
                size: total_dst_size,
            },
            dst,
        )?;

        self.resize_and_blit(
            self.slices.left,
            Size {
                w: self.slices.left.size.w,
                h,
            },
            Rect {
                position: self.slices.left.position,
                size: total_dst_size,
            },
            dst,
        )?;

        Ok(())
    }
}

#[cfg(feature = "png")]
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufWriter, Cursor};

    #[test]
    fn test_resize() {
        let slices = BorderOffsets {
            left: 32,
            top: 32,
            right: 32,
            bottom: 32,
        };
        let mut n = NineSlicedSprite::from_png(
            Cursor::new(include_bytes!("../test_files/test_image.png")),
            slices,
            BorderScaling::Stretch,
        )
        .unwrap();

        let width = 1024;
        let height = 768;
        let image = n.resize(width, height).unwrap();

        NineSlicedSprite::write(
            &image,
            BufWriter::new(File::create("test_files/resized.png").unwrap()),
        )
        .unwrap();
    }
}
