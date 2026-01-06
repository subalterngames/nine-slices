/// How the borders are resized.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BorderScaling {
    /// Repeat the border slices.
    #[default]
    Repeat,
    /// Stretch the boarder slices.
    Stretch,
}
