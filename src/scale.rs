/// How a dimension of the sprite is scaled.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Scale {
    /// Scale by this factor.
    By(f32),
    /// Scale to this many pixels.
    To(usize),
}

impl Scale {
    pub(crate) const fn scale(&self, value: usize) -> usize {
        match self {
            Self::By(factor) => (value as f32 * *factor) as usize,
            Self::To(value) => *value,
        }
    }
}
