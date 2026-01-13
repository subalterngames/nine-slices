use crate::{Error, Rect};
use blittle::{PositionU, Size};

pub struct NineSlices {
    pub(crate) left: usize,
    pub(crate) top: usize,
    pub(crate) right: usize,
    pub(crate) bottom: usize,
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
                x: self.size.w - self.right,
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
}
