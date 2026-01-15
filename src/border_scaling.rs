/// How to scale the borders of a nine-sliced sprite.
///
/// Borders in this case refers to the parts of the borders that aren't corners.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum BorderScaling {
    /// Repeatedly copy+paste the existing border onto the resized image.
    Repeat,
    /// Stretch the border across the resized image.
    #[default]
    Stretch,
}
