use crate::{BorderOffsets, Rect};
use blittle::{PositionU, Size};

/// Nine slices of a sprite.
pub struct NineSlices {
    pub inner: Rect,
    pub left: Rect,
    pub top_left: Rect,
    pub top: Rect,
    pub top_right: Rect,
    pub right: Rect,
    pub bottom_right: Rect,
    pub bottom: Rect,
    pub bottom_left: Rect,
    /// The size of the sprite.
    pub size: Size,
}

impl NineSlices {
    pub const fn new(offsets: BorderOffsets, size: Size) -> Self {
        let inner = Rect {
            position: PositionU {
                x: offsets.left,
                y: offsets.top,
            },
            size: Size {
                w: size.w - (offsets.left + offsets.right),
                h: size.h - (offsets.top + offsets.bottom),
            },
        };
        let left = Rect {
            position: PositionU {
                x: 0,
                y: offsets.top,
            },
            size: Size {
                w: offsets.left,
                h: size.h - (offsets.top + offsets.bottom),
            },
        };
        let top_left = Rect {
            position: PositionU { x: 0, y: 0 },
            size: Size {
                w: offsets.left,
                h: offsets.top,
            },
        };
        let top = Rect {
            position: PositionU {
                x: offsets.left,
                y: 0,
            },
            size: Size {
                w: size.w - (offsets.left + offsets.right),
                h: offsets.top,
            },
        };
        let top_right = Rect {
            position: PositionU {
                x: size.w - offsets.left,
                y: 0,
            },
            size: Size {
                w: offsets.right,
                h: offsets.top,
            },
        };
        let right = Rect {
            position: PositionU {
                x: size.w - offsets.right,
                y: offsets.top,
            },
            size: Size {
                w: offsets.right,
                h: size.h - (offsets.top + offsets.bottom),
            },
        };
        let bottom_right = Rect {
            position: PositionU {
                x: size.w - offsets.right,
                y: size.h - offsets.bottom,
            },
            size: Size {
                w: offsets.right,
                h: offsets.bottom,
            },
        };
        let bottom = Rect {
            position: PositionU {
                x: offsets.left,
                y: size.h - offsets.bottom,
            },
            size: Size {
                w: size.w - (offsets.left + offsets.right),
                h: offsets.bottom,
            },
        };
        let bottom_left = Rect {
            position: PositionU {
                x: 0,
                y: size.h - offsets.bottom,
            },
            size: Size {
                w: offsets.left,
                h: offsets.bottom,
            },
        };
        Self {
            inner,
            left,
            top_left,
            top,
            top_right,
            right,
            bottom_right,
            bottom,
            bottom_left,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slices() {
        const LEFT: usize = 6;
        const TOP: usize = 5;
        const RIGHT: usize = 4;
        const BOTTOM: usize = 3;
        const D: usize = 64;

        let slices = BorderOffsets {
            left: LEFT,
            top: TOP,
            right: RIGHT,
            bottom: BOTTOM,
        }
        .into_slices(Size { w: D, h: D })
        .unwrap();
        assert_eq!(slices.top_left.position, PositionU::default());
        assert_eq!(slices.top_left.size, Size { w: LEFT, h: TOP });
        assert_eq!(slices.top.position, PositionU { x: LEFT, y: 0 });
        assert_eq!(
            slices.top.size,
            Size {
                w: D - LEFT - RIGHT,
                h: TOP
            }
        );
        assert_eq!(slices.top_right.position, PositionU { x: D - LEFT, y: 0 });
        assert_eq!(slices.top_right.size, Size { w: RIGHT, h: TOP });
        assert_eq!(
            slices.right.position,
            PositionU {
                x: D - RIGHT,
                y: TOP
            }
        );
        assert_eq!(
            slices.right.size,
            Size {
                w: RIGHT,
                h: D - TOP - BOTTOM
            }
        );
        assert_eq!(
            slices.bottom_right.position,
            PositionU {
                x: D - RIGHT,
                y: D - BOTTOM
            }
        );
        assert_eq!(
            slices.right.size,
            Size {
                w: RIGHT,
                h: D - TOP - BOTTOM
            }
        );
        assert_eq!(
            slices.bottom.position,
            PositionU {
                x: LEFT,
                y: D - BOTTOM
            }
        );
        assert_eq!(
            slices.bottom.size,
            Size {
                w: D - LEFT - RIGHT,
                h: BOTTOM
            }
        );
        assert_eq!(
            slices.bottom_left.position,
            PositionU {
                x: 0,
                y: D - BOTTOM
            }
        );
        assert_eq!(slices.bottom_left.size, Size { w: LEFT, h: BOTTOM });
        assert_eq!(slices.left.position, PositionU { x: 0, y: TOP });
        assert_eq!(
            slices.left.size,
            Size {
                w: LEFT,
                h: D - TOP - BOTTOM
            }
        )
    }
}
