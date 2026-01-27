use crate::Rect;
use crate::nine_slices::NineSlices;
use crate::pixel_color::PixelColor;
use crate::pixel_type::PixelType;
use blittle::PositionU;

/// Whether to fill a bitmap with a color or to resize a bitmap that has multiple colors.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ResizeMethod {
    /// Do this if all pixels are the same.
    Fill(PixelColor),
    Resize,
}

impl ResizeMethod {
    pub fn new(slice: &Rect, w: usize, pixel_type: &PixelType, src: &[u8]) -> Self {
        // The top-left pixel.
        let color = PixelColor::at_position(slice.position, w, pixel_type, src);
        // Are all pixels the same color?
        let all = (slice.position.y..slice.position.y + slice.size.h).all(|y| {
            (slice.position.x..slice.position.x + slice.size.w).all(|x| {
                // Get the color and match it.
                color == PixelColor::at_position(PositionU { x, y }, w, pixel_type, src)
            })
        });
        if all { Self::Fill(color) } else { Self::Resize }
    }
}

/// Resize methods per slice.
pub struct ResizeMethods {
    pub left: ResizeMethod,
    pub top: ResizeMethod,
    pub right: ResizeMethod,
    pub bottom: ResizeMethod,
    pub inner: ResizeMethod,
}

impl ResizeMethods {
    pub fn new(slices: &NineSlices, w: usize, pixel_type: &PixelType, src: &[u8]) -> Self {
        Self {
            left: ResizeMethod::new(&slices.left, w, pixel_type, src),
            top: ResizeMethod::new(&slices.top, w, pixel_type, src),
            right: ResizeMethod::new(&slices.right, w, pixel_type, src),
            bottom: ResizeMethod::new(&slices.bottom, w, pixel_type, src),
            inner: ResizeMethod::new(&slices.inner, w, pixel_type, src),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::resize_method::ResizeMethod;
    use crate::{BorderOffsets, BorderScaling, NineSlicedSprite};
    use std::io::Cursor;

    #[test]
    fn test_resize_method() {
        let slices = BorderOffsets {
            left: 32,
            top: 32,
            right: 32,
            bottom: 32,
        };

        let sprite = NineSlicedSprite::from_png(
            Cursor::new(include_bytes!("../test_files/src/stretch.png")),
            slices,
            BorderScaling::Stretch,
        )
        .unwrap();

        for (i, slice) in [
            &sprite.slices.left,
            &sprite.slices.top,
            &sprite.slices.right,
            &sprite.slices.bottom,
        ]
        .into_iter()
        .enumerate()
        {
            let method = ResizeMethod::new(
                slice,
                sprite.slices.size.w,
                &sprite.pixel_type,
                sprite.image.buffer(),
            );
            debug_assert!(matches!(method, ResizeMethod::Fill(_)), "{i}");
        }

        let method = ResizeMethod::new(
            &sprite.slices.inner,
            sprite.slices.size.w,
            &sprite.pixel_type,
            sprite.image.buffer(),
        );
        assert_eq!(method, ResizeMethod::Resize);

        let sprite = NineSlicedSprite::from_png(
            Cursor::new(include_bytes!("../test_files/src/repeat.png")),
            slices,
            BorderScaling::Stretch,
        )
        .unwrap();
        for (i, slice) in [
            &sprite.slices.left,
            &sprite.slices.top,
            &sprite.slices.right,
            &sprite.slices.bottom,
            &sprite.slices.inner,
        ]
        .into_iter()
        .enumerate()
        {
            let method = ResizeMethod::new(
                slice,
                sprite.slices.size.w,
                &sprite.pixel_type,
                sprite.image.buffer(),
            );
            debug_assert_eq!(method, ResizeMethod::Resize, "{i}");
        }
    }
}
