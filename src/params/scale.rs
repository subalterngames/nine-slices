#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Scale {
    By(f32),
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
