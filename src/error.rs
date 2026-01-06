use blittle::Size;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create an image from a vec: {0}")]
    FromVec(fast_image_resize::ImageBufferError),
    #[error("Error cropping corner: {0}")]
    Corner(fast_image_resize::CropBoxError),
    #[error("Failed to create a mutable view of the source image.")]
    NoView
}
