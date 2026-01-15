use crate::BorderOffsets;
use fast_image_resize::PixelType;
use thiserror::Error;

#[cfg(feature = "png")]
#[derive(Debug, Error)]
pub enum PngError {
    #[error("Failed to read png info: {0}")]
    Info(png::DecodingError),
    #[error("Failed to get the output buffer size of the source .png")]
    OutputBufferSize,
    #[error("Failed to read png frame: {0}")]
    Frame(png::DecodingError),
    #[error("Invalid png color type and bit depth: {:?}, {:?}", 0, 1)]
    InvalidColorBitDepth(png::ColorType, png::BitDepth),
    #[error("nine-slice doesn't supported indexed colors.")]
    IndexedColorType,
    #[error(
        "Invalid blittle pixel type: {:?}. To use this pixel type, you must must disable the png feature",
        0
    )]
    BlittlePixelType(blittle::PixelType),
    #[error("Failed to write png header: {0}")]
    WriteHeader(png::EncodingError),
    #[error("Failed to write png: {0}")]
    WritePng(png::EncodingError),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid borders: {:?}", 0)]
    InvalidSlices(BorderOffsets),
    #[error("Failed to create an image from a vec: {0}")]
    FromVec(fast_image_resize::ImageBufferError),
    #[error("Error cropping inner area: {0}")]
    Inner(fast_image_resize::CropBoxError),
    #[error("Failed to resize inner area: {0}")]
    Resize(fast_image_resize::ResizeError),
    #[error("Failed to create a mutable view of the source image.")]
    NoView,
    #[error("Invalid clipped rect")]
    InvalidClippedRect,
    #[error("Unsupported pixel type: {:?}", 0)]
    UnsupportedPixelType(PixelType),
    #[cfg(feature = "png")]
    #[error("{0}")]
    Png(PngError),
}
