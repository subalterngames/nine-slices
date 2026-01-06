/// How the borders are resized.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum BorderScaling {
    /// Repeat the border slices.
    #[default]
    Repeat,
    /// Stretch the boarder slices.
    Stretch,
}
