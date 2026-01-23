use crate::Rect;
use blittle::get_index;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ResizeMethod {
    /// Do this if all pixels are the same.
    Fill,
    Resize,
}

impl ResizeMethod {
    pub fn new(slice: &Rect, w: usize, src: &[u8], stride: usize) -> Self {
        // The index of the start of the top-left pixel.
        let c0 = get_index(slice.position.x, slice.position.y, w, stride);
        // The top-left pixel.
        let color = &src[c0..c0 + stride];
        // Are all pixels the same color?
        let all = (slice.position.y..slice.position.y + slice.size.h).all(|y| {
            (slice.position.x..slice.position.x + slice.size.w).all(|x| {
                let c0 = get_index(x, y, w, stride);
                color == &src[c0..c0 + stride]
            })
        });
        if all { Self::Fill } else { Self::Resize }
    }
}

pub struct ResizeMethods {
    pub left: ResizeMethod,
    pub top: ResizeMethod,
    pub right: ResizeMethod,
    pub bottom: ResizeMethod,
    pub inner: ResizeMethod,
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
        /*
        let sprite = NineSlicedSprite::from_png(
            Cursor::new(include_bytes!("../test_files/src/stretch.png")),
            slices,
            BorderScaling::Stretch,
        )
        .unwrap();
        let stride = sprite.pixel_type.blittle.stride();

        for (i, slice) in [
            &sprite.slices.left,
            &sprite.slices.top,
            &sprite.slices.right,
            &sprite.slices.bottom,
        ]
        .into_iter()
        .enumerate()
        {
            let method =
                ResizeMethod::new(slice, sprite.slices.size.w, sprite.image.buffer(), stride);
            debug_assert_eq!(method, ResizeMethod::Fill, "{i}");
        }

        let method = ResizeMethod::new(
            &sprite.slices.inner,
            sprite.slices.size.w,
            sprite.image.buffer(),
            stride,
        );
        assert_eq!(method, ResizeMethod::Resize);

         */

        let sprite = NineSlicedSprite::from_png(
            Cursor::new(include_bytes!("../test_files/src/repeat.png")),
            slices,
            BorderScaling::Stretch,
        )
        .unwrap();
        let stride = sprite.pixel_type.blittle.stride();
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
            let method =
                ResizeMethod::new(slice, sprite.slices.size.w, sprite.image.buffer(), stride);
            debug_assert_eq!(method, ResizeMethod::Resize, "{i}");
        }
    }
}
