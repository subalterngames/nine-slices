use blittle::*;

/// A rectangular region.
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub position: PositionU,
    pub size: Size,
}
