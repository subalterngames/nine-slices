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
        let c0 = get_index(slice.position.x, slice.position.y, w, stride);
        let c1 = get_index(slice.position.x + 1, slice.position.y, w, stride);
        let color = &src[c0..c1];
        let all = (slice.position.y..slice.position.y + slice.size.h)
            .zip(slice.position.x..slice.position.x + slice.size.w)
            .all(|(x, y)| {
                let c0 = get_index(x, y, w, stride);
                let c1 = get_index(x + 1, y, w, stride);
                color == &src[c0..c1]
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
        let sprite = NineSlicedSprite::from_png(
            Cursor::new(include_bytes!("../test_files/src/stretch.png")),
            slices,
            BorderScaling::Stretch,
        )
        .unwrap();
        let stride = sprite.pixel_type.blittle.stride();

        for slice in [
            &sprite.slices.left,
            &sprite.slices.top,
            &sprite.slices.right,
            &sprite.slices.bottom,
        ] {
            let method =
                ResizeMethod::new(slice, sprite.slices.size.w, sprite.image.buffer(), stride);
            assert_eq!(method, ResizeMethod::Fill);
        }

        let method = ResizeMethod::new(
            &sprite.slices.inner,
            sprite.slices.size.w,
            sprite.image.buffer(),
            stride,
        );
        assert_eq!(method, ResizeMethod::Resize);

        let sprite = NineSlicedSprite::from_png(
            Cursor::new(include_bytes!("../test_files/src/repeat.png")),
            slices,
            BorderScaling::Stretch,
        )
        .unwrap();
        let stride = sprite.pixel_type.blittle.stride();
        for slice in [
            &sprite.slices.left,
            &sprite.slices.top,
            &sprite.slices.right,
            &sprite.slices.bottom,
            &sprite.slices.inner,
        ] {
            let method =
                ResizeMethod::new(slice, sprite.slices.size.w, sprite.image.buffer(), stride);
            assert_eq!(method, ResizeMethod::Resize);
        }
    }
}
