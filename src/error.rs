use fast_image_resize::PixelType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create an image from a vec: {0}")]
    FromVec(fast_image_resize::ImageBufferError),
    #[error("Error cropping inner area: {0}")]
    Inner(fast_image_resize::CropBoxError),
    #[error("Failed to resize inner area: {0}")]
    ResizeInner(fast_image_resize::ResizeError),
    #[error("Failed to create a mutable view of the source image.")]
    NoView,
    #[error("Invalid clipped rect")]
    InvalidClippedRect,
    #[error("Unsupported pixel type: {:?}", 0)]
    UnsupportedPixelType(PixelType),
}
