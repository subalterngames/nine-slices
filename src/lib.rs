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
use png::ColorType;
pub use rect::Rect;
use std::io::{BufRead, Cursor, Seek};

pub struct NineSlicedSprite<'s> {
    image: Image<'s>,
    pixel_type: PixelType,
    slices: NineSlices,
    border_scaling: BorderScaling,
    resizer: Resizer,
}

impl<'s> NineSlicedSprite<'s> {
    pub fn new(
        image: Image<'s>,
        offsets: BorderOffsets,
        border_scaling: BorderScaling,
    ) -> Result<Self, Error> {
        let pixel_type = PixelType::new(&image)?;
        let slices = offsets.into_internal(Size {
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

    pub fn new_from_png<R: BufRead + Seek>(
        r: R,
        offsets: BorderOffsets,
        border_scaling: BorderScaling,
    ) -> Result<Self, Error> {
        let decoder = png::Decoder::new(r);
        let mut reader = decoder
            .read_info()
            .map_err(|e| Error::Png(PngError::PngInfo(e)))?;
        let mut buf = vec![
            0;
            reader
                .output_buffer_size()
                .ok_or(Error::Png(PngError::OutputBufferSize))?
        ];
        let info = reader
            .next_frame(&mut buf)
            .map_err(|e| Error::Png(PngError::PngFrame(e)))?;
        let bytes = buf[..info.buffer_size()].to_vec();
        let pixel_type = PixelType::from_png(&info.color_type, &info.bit_depth)?;
        let image =
            Image::from_vec_u8(info.width, info.height, bytes, pixel_type.fast_image_resize)
                .map_err(Error::FromVec)?;
        let slices = offsets.into_internal(Size {
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
        let top_left = self.slices.top_left();
        self.blit(
            src,
            &top_left,
            dst_buffer,
            Rect {
                position: top_left.position,
                size: dst_size,
            },
        )?;
        let top_right = self.slices.top_right();
        self.blit(
            src,
            &top_right,
            dst_buffer,
            Rect {
                position: PositionU {
                    x: dst_size.w - top_right.size.w,
                    y: 0,
                },
                size: dst_size,
            },
        )?;
        let bottom_right = self.slices.bottom_right();
        self.blit(
            src,
            &bottom_right,
            dst_buffer,
            Rect {
                position: PositionU {
                    x: dst_size.w - bottom_right.size.w,
                    y: dst_size.h - bottom_right.size.h,
                },
                size: dst_size,
            },
        )?;
        let bottom_left = self.slices.bottom_left();
        self.blit(
            src,
            &bottom_left,
            dst_buffer,
            Rect {
                position: PositionU {
                    x: 0,
                    y: dst_size.h - bottom_right.size.h,
                },
                size: dst_size,
            },
        )?;

        // Resize and blit the inner area.
        self.resize_and_blit(
            &self.slices.inner(),
            Size {
                w: dst_size.w - (top_left.size.w + top_right.size.w),
                h: dst_size.h - (top_left.size.h + bottom_left.size.h),
            },
            Rect {
                position: PositionU {
                    x: top_left.size.w,
                    y: top_right.size.h,
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

    fn stretch_edges(&mut self, width: u32, height: u32, dst: &mut [u8]) -> Result<(), Error> {
        let top = self.slices.top();
        let right = self.slices.right();
        let bottom = self.slices.bottom();
        let left = self.slices.left();

        let total_dst_size = Size {
            w: width as usize,
            h: height as usize,
        };

        let w = total_dst_size.w - (left.size.w + right.size.w);
        let h = total_dst_size.h - (top.size.h + bottom.size.h);

        self.resize_and_blit(
            &top,
            Size { w, h: top.size.h },
            Rect {
                position: top.position,
                size: total_dst_size,
            },
            dst,
        )?;

        self.resize_and_blit(
            &right,
            Size { w: right.size.w, h },
            Rect {
                position: PositionU {
                    x: total_dst_size.w - right.size.w,
                    y: top.size.h,
                },
                size: total_dst_size,
            },
            dst,
        )?;

        self.resize_and_blit(
            &bottom,
            Size {
                w,
                h: bottom.size.h,
            },
            Rect {
                position: PositionU {
                    x: bottom.position.x,
                    y: total_dst_size.h - bottom.size.h,
                },
                size: total_dst_size,
            },
            dst,
        )?;

        self.resize_and_blit(
            &left,
            Size { w: left.size.w, h },
            Rect {
                position: left.position,
                size: total_dst_size,
            },
            dst,
        )?;

        Ok(())
    }

    fn resize_and_blit(
        &mut self,
        src_rect: &Rect,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use png::ColorType;
    use std::fs::File;
    use std::io::{BufWriter, Cursor};
    use std::path::Path;

    #[test]
    fn test_resize() {
        let decoder =
            png::Decoder::new(Cursor::new(include_bytes!("../test_files/test_image.png")));
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut buf).unwrap();
        assert_eq!(info.color_type, ColorType::Rgba);
        let bytes = buf[..info.buffer_size()].to_vec();

        let image =
            Image::from_vec_u8(512, 512, bytes, fast_image_resize::PixelType::U8x4).unwrap();
        let slices = BorderOffsets {
            left: 32,
            top: 32,
            right: 32,
            bottom: 32,
        };
        let mut n = NineSlicedSprite::new(image, slices, BorderScaling::Stretch).unwrap();
        let width = 1024;
        let height = 768;
        let image = n.resize(width, height).unwrap();
        let path = Path::new("test_files/resized.png");
        let mut encoder =
            png::Encoder::new(BufWriter::new(File::create(path).unwrap()), width, height);
        encoder.set_color(ColorType::Rgba);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(image.buffer()).unwrap();
    }
}
