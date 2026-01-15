use blittle::{PositionU, Size};

/// A rectangle that defines a slice within a nine-sliced sprite.
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub position: PositionU,
    pub size: Size,
}
