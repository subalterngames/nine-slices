use crate::Scale;

/// Resize the image using these axes.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Axis {
    Horizontal(Scale),
    Vertical(Scale),
    Both {
        horizontal: Scale,
        vertical: Scale,
    }
}