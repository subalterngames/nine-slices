use crate::{Error, Rect};
use blittle::{PositionU, Size};

pub struct NineSlices {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
    size: Size,
}

impl NineSlices {
    pub const fn new(
        left: usize,
        top: usize,
        right: usize,
        bottom: usize,
        size: Size,
    ) -> Result<Self, Error> {
        if !Self::valid_borders(left, top, bottom, right, &size) {
            Err(Error::InvalidBorders {
                left,
                top,
                right,
                bottom,
            })
        } else {
            Ok(Self {
                left,
                top,
                right,
                bottom,
                size,
            })
        }
    }

    pub const fn inner(&self) -> Rect {
        Rect {
            position: PositionU {
                x: self.left,
                y: self.top,
            },
            size: Size {
                w: self.size.w - (self.left + self.right),
                h: self.size.h - (self.top + self.bottom),
            },
        }
    }

    pub const fn top_left(&self) -> Rect {
        Rect {
            position: PositionU { x: 0, y: 0 },
            size: Size {
                w: self.left,
                h: self.top,
            },
        }
    }

    pub const fn top(&self) -> Rect {
        Rect {
            position: PositionU { x: self.left, y: 0 },
            size: Size {
                w: self.size.w - (self.left + self.right),
                h: self.top,
            },
        }
    }

    pub const fn top_right(&self) -> Rect {
        Rect {
            position: PositionU {
                x: self.size.w - self.left,
                y: 0,
            },
            size: Size {
                w: self.right,
                h: self.top,
            },
        }
    }

    pub const fn right(&self) -> Rect {
        Rect {
            position: PositionU {
                x: self.size.w - self.right,
                y: self.top,
            },
            size: Size {
                w: self.right,
                h: self.size.h - (self.top + self.bottom),
            },
        }
    }

    pub const fn bottom_right(&self) -> Rect {
        Rect {
            position: PositionU {
                x: self.size.w - self.right,
                y: self.size.h - self.bottom,
            },
            size: Size {
                w: self.right,
                h: self.bottom,
            },
        }
    }

    pub const fn bottom(&self) -> Rect {
        Rect {
            position: PositionU {
                x: self.left,
                y: self.size.h - self.bottom,
            },
            size: Size {
                w: self.size.w - (self.left + self.right),
                h: self.bottom,
            },
        }
    }

    pub const fn bottom_left(&self) -> Rect {
        Rect {
            position: PositionU {
                x: 0,
                y: self.size.h - self.bottom,
            },
            size: Size {
                w: self.left,
                h: self.bottom,
            },
        }
    }

    pub const fn left(&self) -> Rect {
        Rect {
            position: PositionU { x: 0, y: self.top },
            size: Size {
                w: self.left,
                h: self.size.h - (self.top + self.bottom),
            },
        }
    }

    /// Check whether we can use these borders:
    ///
    /// - There must be at least some padding.
    /// - The borders must be within the area.
    /// - The borders can't cross each other.
    const fn valid_borders(
        left: usize,
        top: usize,
        right: usize,
        bottom: usize,
        size: &Size,
    ) -> bool {
        top > 0
            && left > 0
            && bottom > 0
            && right > 0
            && top < size.h - bottom
            && left < size.w - right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliced_borders() {
        let size = Size { w: 400, h: 300 };
        assert!(!NineSlices::valid_borders(0, 1, 2, 3, &size));
        assert!(NineSlices::valid_borders(2, 2, 2, 2, &size));
        assert!(!NineSlices::valid_borders(500, 2, 2, 2, &size));
        assert!(!NineSlices::valid_borders(250, 2, 270, 2, &size));
        assert!(!NineSlices::valid_borders(250, 2, 250, 2, &size));
        assert!(!NineSlices::valid_borders(2, 500, 2, 2, &size));
        assert!(!NineSlices::valid_borders(2, 270, 2, 0, &size));
        assert!(NineSlices::valid_borders(2, 270, 2, 1, &size));
        assert!(!NineSlices::valid_borders(2, 270, 2, 250, &size));
        assert!(!NineSlices::valid_borders(2, 250, 2, 250, &size));
    }

    #[test]
    fn test_slices() {
        const LEFT: usize = 6;
        const TOP: usize = 5;
        const RIGHT: usize = 4;
        const BOTTOM: usize = 3;
        const D: usize = 64;

        let slices = NineSlices::new(LEFT, TOP, RIGHT, BOTTOM, Size { w: D, h: D }).unwrap();
        let top_left = slices.top_left();
        assert_eq!(top_left.position, PositionU::default());
        assert_eq!(top_left.size, Size { w: LEFT, h: TOP });
        let top = slices.top();
        assert_eq!(top.position, PositionU { x: LEFT, y: 0 });
        assert_eq!(
            top.size,
            Size {
                w: D - LEFT - RIGHT,
                h: TOP
            }
        );
        let top_right = slices.top_right();
        assert_eq!(top_right.position, PositionU { x: D - LEFT, y: 0 });
        assert_eq!(top_right.size, Size { w: RIGHT, h: TOP });
        let right = slices.right();
        assert_eq!(
            right.position,
            PositionU {
                x: D - RIGHT,
                y: TOP
            }
        );
        assert_eq!(
            right.size,
            Size {
                w: RIGHT,
                h: D - TOP - BOTTOM
            }
        );
        let bottom_right = slices.bottom_right();
        assert_eq!(
            bottom_right.position,
            PositionU {
                x: D - RIGHT,
                y: D - BOTTOM
            }
        );
        assert_eq!(
            right.size,
            Size {
                w: RIGHT,
                h: D - TOP - BOTTOM
            }
        );
        let bottom = slices.bottom();
        assert_eq!(
            bottom.position,
            PositionU {
                x: LEFT,
                y: D - BOTTOM
            }
        );
        assert_eq!(
            bottom.size,
            Size {
                w: D - LEFT - RIGHT,
                h: BOTTOM
            }
        );
        let bottom_left = slices.bottom_left();
        assert_eq!(
            bottom_left.position,
            PositionU {
                x: 0,
                y: D - BOTTOM
            }
        );
        assert_eq!(bottom_left.size, Size { w: LEFT, h: BOTTOM });
        let left = slices.left();
        assert_eq!(left.position, PositionU { x: 0, y: TOP });
        assert_eq!(
            left.size,
            Size {
                w: LEFT,
                h: D - TOP - BOTTOM
            }
        )
    }
}
