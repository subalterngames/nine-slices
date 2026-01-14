use crate::{Error, Rect};
use blittle::{PositionU, Size};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NineSlices {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

impl NineSlices {
    pub(crate) fn into_internal(self, size: Size) -> Result<NineSlicesInternal, Error> {
        if self.is_valid(&size) {
            Ok(NineSlicesInternal {
                left: self.left,
                top: self.top,
                right: self.right,
                bottom: self.bottom,
                size,
            })
        } else {
            Err(Error::InvalidSlices(self))
        }
    }

    /// Check whether we can use these borders:
    ///
    /// - There must be at least some padding.
    /// - The borders must be within the area.
    /// - The borders can't cross each other.
    const fn is_valid(&self, size: &Size) -> bool {
        self.top > 0
            && self.left > 0
            && self.bottom > 0
            && self.right > 0
            && self.top < size.h - self.bottom
            && self.left < size.w - self.right
    }
}

pub struct NineSlicesInternal {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
    pub(crate) size: Size,
}

impl NineSlicesInternal {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliced_borders() {
        let size = Size { w: 400, h: 300 };
        assert!(
            NineSlices {
                left: 2,
                top: 2,
                right: 2,
                bottom: 2
            }
            .is_valid(&size)
        );
        assert!(
            NineSlices {
                left: 2,
                top: 270,
                right: 2,
                bottom: 2
            }
            .is_valid(&size)
        );

        // Can't have values that equal zero.
        assert!(
            !NineSlices {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0
            }
            .is_valid(&size)
        );
        assert!(
            !NineSlices {
                left: 0,
                top: 1,
                right: 2,
                bottom: 3
            }
            .is_valid(&size)
        );

        // Can't have borders cross each other.
        assert!(
            !NineSlices {
                left: 500,
                top: 2,
                right: 2,
                bottom: 2
            }
            .is_valid(&size)
        );

        assert!(
            !NineSlices {
                left: 250,
                top: 2,
                right: 270,
                bottom: 2
            }
            .is_valid(&size)
        );
        assert!(
            !NineSlices {
                left: 250,
                top: 2,
                right: 250,
                bottom: 2
            }
            .is_valid(&size)
        );
        assert!(
            !NineSlices {
                left: 2,
                top: 500,
                right: 2,
                bottom: 2
            }
            .is_valid(&size)
        );

        // Can't have borders exceed size.
        assert!(
            !NineSlices {
                left: 900,
                top: 1000,
                right: 1200,
                bottom: 2000
            }
            .is_valid(&size)
        );
        assert!(
            !NineSlices {
                left: 2,
                top: 2,
                right: 2,
                bottom: 2000
            }
            .is_valid(&size)
        );
    }

    #[test]
    fn test_slices() {
        const LEFT: usize = 6;
        const TOP: usize = 5;
        const RIGHT: usize = 4;
        const BOTTOM: usize = 3;
        const D: usize = 64;

        let slices = NineSlices {
            left: LEFT,
            top: TOP,
            right: RIGHT,
            bottom: BOTTOM,
        }
        .into_internal(Size { w: D, h: D })
        .unwrap();
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
