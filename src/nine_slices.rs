use crate::BorderOffsets;
use blittle::{PositionU, RectU, Size};

/// Nine slices of a sprite.
pub struct NineSlices {
    pub inner: RectU,
    pub left: RectU,
    pub top_left: RectU,
    pub top: RectU,
    pub top_right: RectU,
    pub right: RectU,
    pub bottom_right: RectU,
    pub bottom: RectU,
    pub bottom_left: RectU,
}

impl NineSlices {
    pub const fn new(offsets: BorderOffsets, size: Size) -> Self {
        let inner = RectU {
            position: PositionU {
                x: offsets.left,
                y: offsets.top,
            },
            size: Size {
                width: size.width - (offsets.left + offsets.right),
                height: size.height - (offsets.top + offsets.bottom),
            },
        };
        let left = RectU {
            position: PositionU {
                x: 0,
                y: offsets.top,
            },
            size: Size {
                width: offsets.left,
                height: size.height - (offsets.top + offsets.bottom),
            },
        };
        let top_left = RectU {
            position: PositionU { x: 0, y: 0 },
            size: Size {
                width: offsets.left,
                height: offsets.top,
            },
        };
        let top = RectU {
            position: PositionU {
                x: offsets.left,
                y: 0,
            },
            size: Size {
                width: size.width - (offsets.left + offsets.right),
                height: offsets.top,
            },
        };
        let top_right = RectU {
            position: PositionU {
                x: size.width - offsets.left,
                y: 0,
            },
            size: Size {
                width: offsets.right,
                height: offsets.top,
            },
        };
        let right = RectU {
            position: PositionU {
                x: size.width - offsets.right,
                y: offsets.top,
            },
            size: Size {
                width: offsets.right,
                height: size.height - (offsets.top + offsets.bottom),
            },
        };
        let bottom_right = RectU {
            position: PositionU {
                x: size.width - offsets.right,
                y: size.height - offsets.bottom,
            },
            size: Size {
                width: offsets.right,
                height: offsets.bottom,
            },
        };
        let bottom = RectU {
            position: PositionU {
                x: offsets.left,
                y: size.height - offsets.bottom,
            },
            size: Size {
                width: size.width - (offsets.left + offsets.right),
                height: offsets.bottom,
            },
        };
        let bottom_left = RectU {
            position: PositionU {
                x: 0,
                y: size.height - offsets.bottom,
            },
            size: Size {
                width: offsets.left,
                height: offsets.bottom,
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
        .into_slices(Size {
            width: D,
            height: D,
        })
        .unwrap();
        assert_eq!(slices.top_left.position, PositionU::default());
        assert_eq!(
            slices.top_left.size,
            Size {
                width: LEFT,
                height: TOP
            }
        );
        assert_eq!(slices.top.position, PositionU { x: LEFT, y: 0 });
        assert_eq!(
            slices.top.size,
            Size {
                width: D - LEFT - RIGHT,
                height: TOP
            }
        );
        assert_eq!(slices.top_right.position, PositionU { x: D - LEFT, y: 0 });
        assert_eq!(
            slices.top_right.size,
            Size {
                width: RIGHT,
                height: TOP
            }
        );
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
                width: RIGHT,
                height: D - TOP - BOTTOM
            }
        );
        assert_eq!(
            slices.bottom_right.position,
            PositionU {
                x: D - RIGHT,
                y: D - BOTTOM,
            }
        );
        assert_eq!(
            slices.right.size,
            Size {
                width: RIGHT,
                height: D - TOP - BOTTOM
            }
        );
        assert_eq!(
            slices.bottom.position,
            PositionU {
                x: LEFT,
                y: D - BOTTOM,
            }
        );
        assert_eq!(
            slices.bottom.size,
            Size {
                width: D - LEFT - RIGHT,
                height: BOTTOM
            }
        );
        assert_eq!(
            slices.bottom_left.position,
            PositionU {
                x: 0,
                y: D - BOTTOM,
            }
        );
        assert_eq!(
            slices.bottom_left.size,
            Size {
                width: LEFT,
                height: BOTTOM
            }
        );
        assert_eq!(slices.left.position, PositionU { x: 0, y: TOP });
        assert_eq!(
            slices.left.size,
            Size {
                width: LEFT,
                height: D - TOP - BOTTOM
            }
        )
    }
}
