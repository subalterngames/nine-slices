use crate::Error;
use crate::nine_slices::NineSlices;
use blittle::Size;

/// Border offsets that define a 9-sliced sprite.
///
/// Each of these fields in an absolute pixel count from its respective side.
/// For example, if `left == 2`, the left boundary will be 2 pixels offset from the left side of the bitmap.
/// And, if `right == 2`, the right boundary will be 2 pixels offset from the right side of the bitmap.
///
/// Rules for offset values:
///
/// - Must be greater than 0
/// - Must be within the bounds of the bitmap
/// - Must not cross each other on the bitmap
///
///
/// This is a valid `BorderOffsets`:
///
/// ```
/// use nine_slices::BorderOffsets;
///
/// let width = 12;
/// let height = 12;
///
/// let offsets = BorderOffsets {
///     left: 2,
///     top: 1,
///     right: 3,
///     bottom: 8
/// };
///
/// assert!(offsets.is_valid(width, height));
/// ```
///
/// And this is an *invalid* `BorderOffsets`:
///
/// ```
/// use nine_slices::BorderOffsets;
///
/// let width = 12;
/// let height = 12;
///
/// let offsets = BorderOffsets {
///     left: 0, // Must be greater than 0
///     top: 12,
///     right: 20, // Must be within the bounds of the bitmap,
///     bottom: 5, // Must not cross each other on the bitmap
/// };
///
/// assert!(!offsets.is_valid(width, height));
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BorderOffsets {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

impl BorderOffsets {
    /// Check if the offsets are valid and then convert into `NineSlices`.
    pub(crate) const fn into_slices(self, size: Size) -> Result<NineSlices, Error> {
        if self.is_valid(size.width, size.height) {
            Ok(NineSlices::new(self, size))
        } else {
            Err(Error::InvalidSlices(self))
        }
    }

    /// Check whether we can use these offsets:
    ///
    /// - Every offset must be greater than 0.
    /// - The offsets must be within the bounds defined by `width` and `height`.
    /// - The offsets can't cross each other.
    pub const fn is_valid(&self, width: usize, height: usize) -> bool {
        self.top > 0
            && self.left > 0
            && self.bottom > 0
            && self.right > 0
            && self.bottom < height
            && self.right < width
            && self.top < height - self.bottom
            && self.left < width - self.right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliced_borders() {
        let width = 400;
        let height = 300;
        assert!(
            BorderOffsets {
                left: 2,
                top: 2,
                right: 2,
                bottom: 2
            }
            .is_valid(width, height)
        );
        assert!(
            BorderOffsets {
                left: 2,
                top: 270,
                right: 2,
                bottom: 2
            }
            .is_valid(width, height)
        );

        // Can't have values that equal zero.
        assert!(
            !BorderOffsets {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0
            }
            .is_valid(width, height)
        );
        assert!(
            !BorderOffsets {
                left: 0,
                top: 1,
                right: 2,
                bottom: 3
            }
            .is_valid(width, height)
        );

        // Can't have borders cross each other.
        assert!(
            !BorderOffsets {
                left: 500,
                top: 2,
                right: 2,
                bottom: 2
            }
            .is_valid(width, height)
        );

        assert!(
            !BorderOffsets {
                left: 250,
                top: 2,
                right: 270,
                bottom: 2
            }
            .is_valid(width, height)
        );
        assert!(
            !BorderOffsets {
                left: 250,
                top: 2,
                right: 250,
                bottom: 2
            }
            .is_valid(width, height)
        );
        assert!(
            !BorderOffsets {
                left: 2,
                top: 500,
                right: 2,
                bottom: 2
            }
            .is_valid(width, height)
        );

        // Can't have borders exceed size.
        assert!(
            !BorderOffsets {
                left: 900,
                top: 1000,
                right: 1200,
                bottom: 2000
            }
            .is_valid(width, height)
        );
        assert!(
            !BorderOffsets {
                left: 2,
                top: 2,
                right: 2,
                bottom: 2000
            }
            .is_valid(width, height)
        );
    }
}
