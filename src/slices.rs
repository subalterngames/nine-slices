use blittle::{PositionU, Size};

pub struct Rect {
    pub position: PositionU,
    pub size: Size,
}

pub struct Slices {
    pub top_left: Rect,
    pub top: Rect,
    pub top_right: Rect,
    pub right: Rect,
    pub bottom_right: Rect,
    pub bottom: Rect,
    pub bottom_left: Rect,
    pub left: Rect,
    pub inner: Rect,
}

impl Slices {
    pub const fn new(
        left: usize,
        top: usize,
        bottom: usize,
        right: usize,
        size: &Size,
    ) -> Option<Self> {
        if !Self::valid_borders(left, top, bottom, right, size) {
            None
        } else {
            let top_left = Rect {
                position: PositionU { x: 0, y: 0 },
                size: Size { w: left, h: top },
            };
            let top_right = Rect {
                position: PositionU {
                    x: size.w - right,
                    y: 0,
                },
                size: Size { w: right, h: top },
            };
            let bottom_right = Rect {
                position: PositionU {
                    x: size.w - right,
                    y: size.h - bottom,
                },
                size: Size {
                    w: right,
                    h: bottom,
                },
            };
            let bottom_left = Rect {
                position: PositionU {
                    x: 0,
                    y: size.h - bottom,
                },
                size: Size { w: left, h: bottom },
            };

            let top_middle = Rect {
                position: PositionU { x: left, y: 0 },
                size: Size {
                    w: size.w - (left + right),
                    h: top,
                },
            };
            let right_middle = Rect {
                position: PositionU {
                    x: size.w - right,
                    y: top,
                },
                size: Size {
                    w: right,
                    h: size.h - (top + bottom),
                },
            };
            let bottom_middle = Rect {
                position: PositionU {
                    x: left,
                    y: size.h - bottom,
                },
                size: Size {
                    w: size.w - (left + right),
                    h: bottom,
                },
            };
            let left_middle = Rect {
                position: PositionU { x: 0, y: top },
                size: Size {
                    w: left,
                    h: size.h - (top + bottom),
                },
            };
            let inner = Rect {
                position: PositionU { x: left, y: top },
                size: Size {
                    w: size.w - (left + right),
                    h: size.h - (top + bottom),
                },
            };
            Some(Self {
                top_left,
                top: top_middle,
                top_right,
                right: right_middle,
                bottom_right,
                bottom: bottom_middle,
                bottom_left,
                left: left_middle,
                inner,
            })
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
        bottom: usize,
        right: usize,
        size: &Size,
    ) -> bool {
        top == 0
            || right == 0
            || bottom == 0
            || left == 0
            || top >= size.h - bottom
            || top >= size.h
            || right >= size.w
            || bottom >= size.h
            || left >= size.w - right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliced_borders() {
        let size = Size { w: 400, h: 300 };
        assert!(!Slices::valid_borders(0, 1, 2, 3, &size));
        assert!(Slices::valid_borders(2, 2, 2, 2, &size));
        assert!(!Slices::valid_borders(500, 2, 2, 2, &size));
        assert!(!Slices::valid_borders(250, 2, 270, 2, &size));
        assert!(!Slices::valid_borders(250, 2, 250, 2, &size));
        assert!(!Slices::valid_borders(2, 500, 2, 2, &size));
        assert!(Slices::valid_borders(2, 270, 2, 0, &size));
        assert!(!Slices::valid_borders(2, 270, 2, 250, &size));
        assert!(!Slices::valid_borders(2, 250, 2, 250, &size));
    }
}
