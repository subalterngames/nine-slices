mod border_scaling;
mod error;
mod pixel_type;
mod rect;
mod slices;

use blittle::{ClippedRect, PositionI, Size, blit};
pub use border_scaling::BorderScaling;
pub use error::Error;
pub use fast_image_resize::images::Image;
use fast_image_resize::{ResizeOptions, Resizer};
use pixel_type::PixelType;
pub use rect::Rect;
pub use slices::*;

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
        slices: NineSlices,
        border_scaling: BorderScaling,
    ) -> Result<Self, Error> {
        let pixel_type = PixelType::new(&image)?;
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
        // Blit corners.
        self.blit(src, dst_buffer, &self.slices.top_left(), &clipped_rect);
        self.blit(src, dst_buffer, &self.slices.top_right(), &clipped_rect);
        self.blit(src, dst_buffer, &self.slices.bottom_right(), &clipped_rect);
        self.blit(src, dst_buffer, &self.slices.bottom_left(), &clipped_rect);

        // Resize and blit the inner area.
        self.resize_and_blit(
            &self.slices.inner(),
            width - (self.slices.left().size.w + self.slices.right().size.w) as u32,
            height - (self.slices.top().size.h + self.slices.bottom().size.h) as u32,
            Size {
                w: width as usize,
                h: height as usize,
            },
            dst_buffer,
        )?;

        match &self.border_scaling {
            BorderScaling::Repeat => todo!(),
            BorderScaling::Stretch => self.stretch_edges(width, height, dst_buffer)?,
        }

        Ok(dst)
    }

    fn blit(&self, src: &[u8], dst: &mut [u8], rect: &Rect, clipped_rect: &ClippedRect) {
        let mut clipped_rect = *clipped_rect;
        clipped_rect.set_src_rect(rect.position, rect.size);
        blit(src, dst, &clipped_rect, &self.pixel_type.blittle);
    }

    fn stretch_edges(&mut self, width: u32, height: u32, dst: &mut [u8]) -> Result<(), Error> {
        let dst_size = Size {
            w: width as usize,
            h: height as usize,
        };
        let top = self.slices.top();
        self.resize_and_blit(&top, width, top.size.h as u32, dst_size, dst)?;
        let right = self.slices.right();
        self.resize_and_blit(&right, right.size.w as u32, height, dst_size, dst)?;
        let bottom = self.slices.bottom();
        self.resize_and_blit(&bottom, width, bottom.size.h as u32, dst_size, dst)?;
        let left = self.slices.left();
        self.resize_and_blit(&left, left.size.w as u32, height, dst_size, dst)
    }

    fn resize_and_blit(
        &mut self,
        rect: &Rect,
        width: u32,
        height: u32,
        dst_size: Size,
        dst: &mut [u8],
    ) -> Result<(), Error> {
        // Resize.
        let options = ResizeOptions::new().crop(
            rect.position.x as f64,
            rect.position.y as f64,
            rect.size.w as f64,
            rect.size.h as f64,
        );
        let mut resized = Image::new(width, height, self.pixel_type.fast_image_resize);
        self.resizer
            .resize(&self.image, &mut resized, Some(&options))
            .map_err(Error::ResizeInner)?;
        // Blit.
        let clipped_rect = ClippedRect::new(
            rect.position.into(),
            dst_size,
            Size {
                w: resized.width() as usize,
                h: resized.height() as usize,
            },
        )
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
        let bytes = buf[..info.buffer_size()].to_vec();

        let image =
            Image::from_vec_u8(512, 512, bytes, fast_image_resize::PixelType::U8x3).unwrap();
        let slices = NineSlices::new(16, 16, 16, 16, Size { w: 512, h: 512 }).unwrap();
        let mut n = NineSlicedSprite::new(image, slices, BorderScaling::Stretch).unwrap();
        let width = 1024;
        let height = 768;
        let image = n.resize(width, height).unwrap();
        let path = Path::new("test_files/resized.png");
        let mut encoder =
            png::Encoder::new(BufWriter::new(File::create(path).unwrap()), width, height);
        encoder.set_color(ColorType::Rgb);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(image.buffer()).unwrap();
    }
}
