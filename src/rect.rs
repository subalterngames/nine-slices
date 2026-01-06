use glam::UVec2;

#[derive(Clone, Debug)]
pub enum Rect {
    /// A rectangle within a sprite atlas.
    InAtlas {
        /// The size of the atlas.
        atlas_size: UVec2,
        /// The top-left position of the sprite in the atlas.
        position: UVec2,
        /// The size of the sprite in the atlas.
        sprite_size: UVec2,
    },
    /// A rectangle, defined by a size.
    Size(UVec2)
}

impl Rect {
    pub const fn image_size(&self) -> UVec2 {
        match self {
            Self::InAtlas { atlas_size, position: _, sprite_size: _} => *atlas_size,
            Self::Size(size) => *size
        }
    }

    pub const fn sprite_size(&self) -> UVec2 {
        match self {
            Self::InAtlas { atlas_size: _, position: _, sprite_size } => *sprite_size,
            Self::Size(size) => *size
        }
    }
}