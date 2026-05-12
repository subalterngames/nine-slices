use crate::BorderOffsets;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid borders: {:?}", 0)]
    InvalidSlices(BorderOffsets),
    #[error("Failed to view destination image from a surface: {0}")]
    FromSurface(fast_image_resize::ImageBufferError),
    #[error("Failed to crop destination image: {0}")]
    Crop(fast_image_resize::CropBoxError),
    #[error("Failed to resize inner area: {0}")]
    Resize(fast_image_resize::ResizeError),
    #[error("{0}")]
    Blittle(blittle::Error),
}
