use crate::BorderOffsets;
use fast_image_resize::PixelType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid borders: {:?}", 0)]
    InvalidSlices(BorderOffsets),
    #[error("Failed to create an image from a vec: {0}")]
    FromVec(fast_image_resize::ImageBufferError),
    #[error("Failed to resize inner area: {0}")]
    Resize(fast_image_resize::ResizeError),
    #[error("Invalid clipped rect")]
    InvalidClippedRect,
    #[error("Unsupported pixel type: {:?}", 0)]
    UnsupportedPixelType(PixelType),
    #[cfg(feature = "png")]
    #[error("{0}")]
    Png(PngError),
    #[cfg(feature = "jpg")]
    #[error("{0}")]
    Jpg(JpgError)
}

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
    #[error("Failed to write png header: {0}")]
    WriteHeader(png::EncodingError),
    #[error("Failed to write png: {0}")]
    WritePng(png::EncodingError),
}

#[cfg(feature = "jpg")]
#[derive(Debug, Error)]
pub enum JpgError {
    #[error("Failed to read jpg: {0}")]
    Read(jpeg_decoder::Error),
    #[error("Failed to get jpg info.")]
    Info,
    #[error("Unsupported pixel format: {:?}", 0)]
    PixelFormat(PixelType),
    #[error("Unsupported jpg pixel format: {:?}", 0)]
    JpgPixelFormat(jpeg_decoder::PixelFormat),
    #[error("Failed to decode jpg: {0}")]
    Decode(jpeg_decoder::Error),
    #[error("Failed to encode jpg: {0}")]
    Encode(jpeg_encoder::EncodingError)
}