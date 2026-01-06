use blittle::Size;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "Tried to resize a sprite to be smaller than its original size: {} to {}",
        from,
        to
    )]
    TooSmall { from: Size, to: Size },
    #[error(
        "Failed to get a clipped rect for a new bitmap of size {0}. This should probably never happen??"
    )]
    ClippedRect(Size),
}
