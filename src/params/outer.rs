use crate::params::Rect;
use blittle::Size;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Copy, Clone, Debug)]
pub enum OuterArea {
    Rect(Rect),
    Size(Size),
}

impl OuterArea {
    pub const fn size(&self) -> &Size {
        match self {
            Self::Rect(rect) => &rect.size,
            Self::Size(size) => size,
        }
    }
}
