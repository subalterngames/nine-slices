use crate::borders::Borders;
use crate::Rect;

pub struct Corner {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32
}

pub struct Corners {
    pub top_left: Corner,
    pub top_right: Corner,
    pub bottom_right: Corner,
    pub bottom_left: Corner,
}

impl Corners {
    pub const fn new(borders: &Borders, rect: &Rect) -> Self {
        match &rect {
            Rect::InAtlas { atlas_size: _, position, sprite_size} => {
                let top_left = Corner {
                    left: position.x,
                    top: position.y,
                    width: borders.left,
                    height: borders.top
                };
                let top_right = Corner {
                    left: position.x + sprite_size.x - borders.right,
                    top: top_left.top,
                    width: borders.right,
                    height: top_left.height,
                };
                let bottom_right = Corner {
                    left: top_right.left,
                    top: position.y + sprite_size.y - borders.bottom,
                    width: top_right.width,
                    height: borders.bottom,
                };
                let bottom_left = Corner {
                    left: top_left.left,
                    top: bottom_right.top,
                    width: top_left.width,
                    height: bottom_right.height
                };
                Self {
                    top_left,
                    top_right,
                    bottom_right,
                    bottom_left
                }
            },
            Rect::Size(size) => {
                let top_left = Corner {
                    left: 0,
                    top: 0,
                    width: borders.left,
                    height: borders.top
                };
                let top_right = Corner {
                    left: size.x - borders.right,
                    top: top_left.top,
                    width: borders.right,
                    height: top_left.height,
                };
                let bottom_right = Corner {
                    left: top_right.left,
                    top: size.y - borders.bottom,
                    width: top_right.width,
                    height: borders.bottom,
                };
                let bottom_left = Corner {
                    left: top_left.left,
                    top: bottom_right.top,
                    width: top_left.width,
                    height: bottom_right.height
                };
                Self {
                    top_left,
                    top_right,
                    bottom_right,
                    bottom_left
                }
            }
        }
    }
}