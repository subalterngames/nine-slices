use blittle::{PositionU, Size};

/// A rectangle that defines a slice within a nine-sliced sprite.
pub struct Rect {
    pub position: PositionU,
    pub size: Size,
}
