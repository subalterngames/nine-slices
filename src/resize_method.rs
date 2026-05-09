use crate::nine_slices::NineSlices;
use blittle::{PositionU, RectU, Surface};

/// Whether to fill a bitmap with a color or to resize a bitmap that has multiple colors.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ResizeMethod<P: Copy + Clone + Sized + Default + PartialEq> {
    /// Do this if all pixels are the same.
    Fill(P),
    Resize,
}

impl<P: Copy + Clone + Sized + Default + PartialEq> ResizeMethod<P> {
    pub fn new<S: AsRef<[P]> + AsMut<[P]>>(slice: &RectU, surface: &Surface<'_, S, P>) -> Self {
        // The top-left pixel.
        let color = surface.get_pixel_unchecked(slice.position);
        // Are all pixels the same color?
        let all = (slice.position.y..slice.position.y + slice.size.height).all(|y| {
            (slice.position.x..slice.position.x + slice.size.width).all(|x| {
                // Get the color and match it.
                color == surface.get_pixel_unchecked(PositionU { x, y })
            })
        });
        if all { Self::Fill(color) } else { Self::Resize }
    }
}

/// Resize methods per slice.
pub struct ResizeMethods<P: Copy + Clone + Sized + Default + PartialEq> {
    pub left: ResizeMethod<P>,
    pub top: ResizeMethod<P>,
    pub right: ResizeMethod<P>,
    pub bottom: ResizeMethod<P>,
    pub inner: ResizeMethod<P>,
}

impl<P: Copy + Clone + Sized + Default + PartialEq> ResizeMethods<P> {
    pub fn new<S: AsRef<[P]> + AsMut<[P]>>(
        slices: &NineSlices,
        surface: &Surface<'_, S, P>,
    ) -> Self {
        Self {
            left: ResizeMethod::new(&slices.left, surface),
            top: ResizeMethod::new(&slices.top, surface),
            right: ResizeMethod::new(&slices.right, surface),
            bottom: ResizeMethod::new(&slices.bottom, surface),
            inner: ResizeMethod::new(&slices.inner, surface),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::resize_method::ResizeMethod;
    use crate::{BorderOffsets, BorderScaling, NineSlicedSprite};
    use blittle::Rgb8Surface;
    use blittle::png::Png;
    use std::io::Cursor;

    #[test]
    fn test_resize_method() {
        let slices = BorderOffsets {
            left: 32,
            top: 32,
            right: 32,
            bottom: 32,
        };

        let surface =
            Rgb8Surface::read_png(Cursor::new(include_bytes!("../test_files/src/stretch.png")))
                .unwrap();

        let surface = NineSlicedSprite::new(surface, slices, BorderScaling::Stretch).unwrap();

        for (i, slice) in [
            &surface.slices.left,
            &surface.slices.top,
            &surface.slices.right,
            &surface.slices.bottom,
        ]
        .into_iter()
        .enumerate()
        {
            let method = ResizeMethod::new(slice, &surface.surface);
            debug_assert!(matches!(method, ResizeMethod::Fill(_)), "{i}");
        }

        let method = ResizeMethod::new(&surface.slices.inner, &surface.surface);
        assert_eq!(method, ResizeMethod::Resize);

        let surface = NineSlicedSprite::new(
            Rgb8Surface::read_png(Cursor::new(include_bytes!("../test_files/src/repeat.png")))
                .unwrap(),
            slices,
            BorderScaling::Stretch,
        )
        .unwrap();

        for (i, slice) in [
            &surface.slices.left,
            &surface.slices.top,
            &surface.slices.right,
            &surface.slices.bottom,
            &surface.slices.inner,
        ]
        .into_iter()
        .enumerate()
        {
            let method = ResizeMethod::new(slice, &surface.surface);
            debug_assert_eq!(method, ResizeMethod::Resize, "{i}");
        }
    }
}
