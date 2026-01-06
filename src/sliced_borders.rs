use blittle::Size;

pub struct SlicedBorders {
    pub top: usize,
    pub right: usize,
    pub bottom: usize, 
    pub left: usize
}

impl SlicedBorders {
    pub const fn new_checked(top: usize, right: usize, bottom: usize, left: usize, size: Size) -> Option<Self> {
        // There must be at least some padding.
        if top == 0 || right == 0 || bottom == 0 || left == 0 || top >= bottom || top >= size.h ||
            right >= size.w || bottom >= size.h || left >= right {
            None
        }
        else {
            Some(Self {
                top,
                right,
                bottom,
                left
            })
        }
    }
}

