use blittle::Size;

pub struct Borders {
    pub top: u32,
    pub right: u32,
    pub bottom: u32, 
    pub left: u32
}

impl Borders {
    pub const fn new_checked(top: usize, right: usize, bottom: usize, left: usize, size: &Size) -> Option<Self> {
        // There must be at least some padding.
        if top == 0 || right == 0 || bottom == 0 || left == 0 || top >= size.h - bottom || top >= size.h ||
            right >= size.w || bottom >= size.h || left >= size.w - right {
            None
        }
        else {
            Some(Self {
                top: top as u32,
                right: right as u32,
                bottom: bottom as u32,
                left: left as u32
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_sliced_borders() {
        let size = Size {
            w: 400,
            h: 300
        };
        assert!(Borders::new_checked(0, 1, 2, 3, &size).is_none());
        assert!(Borders::new_checked(2, 2, 2, 2, &size).is_some());
        assert!(Borders::new_checked(500, 2, 2, 2, &size).is_none());
        assert!(Borders::new_checked(250, 2, 270, 2, &size).is_none());
        assert!(Borders::new_checked(250, 2, 250, 2, &size).is_none());
        assert!(Borders::new_checked(2, 500, 2, 2, &size).is_none());
        assert!(Borders::new_checked(2, 270, 2, 250, &size).is_none());
        assert!(Borders::new_checked(2, 250, 2, 250, &size).is_none());
    }
}

