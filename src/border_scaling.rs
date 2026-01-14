use blittle::Size;

/// How the borders are resized.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum BorderScaling {
    /// Repeat chunks of the border slices.
    Repeat,
    /// Stretch the border slices.
    #[default]
    Stretch,
}
