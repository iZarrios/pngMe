use std::fmt::Display;

use crate::{chunk::Chunk, Error, Result};

#[derive(Debug)]
pub struct IhdrChunk {
    width: u32,
    height: u32,

    /// Represents the color types and their corresponding allowed bit depths.
    ///
    /// Each color type corresponds to a specific way of interpreting pixel data in an image.
    ///
    /// # Color Types
    /// | Color Type | Allowed Bit Depths | Interpretation |
    /// |------------|---------------------|----------------|
    /// | 0          | 1, 2, 4, 8, 16      | Each pixel is a grayscale sample. |
    /// | 2          | 8, 16               | Each pixel is an RGB triple. |
    /// | 3          | 1, 2, 4, 8          | Each pixel is a palette index; a PLTE chunk must appear. |
    /// | 4          | 8, 16               | Each pixel is a grayscale sample, followed by an alpha sample. |
    /// | 6          | 8, 16               | Each pixel is an RGB triple, followed by an alpha sample. |
    ///
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

impl IhdrChunk {
    const CHUNK_LENGTH: u32 = 13;
}

impl Display for IhdrChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "IHDR : {}x{} bith_depth={}, color_type={}, compression_method={}, filter_method={}, interlace_method={}",
            self.width,
            self.height,
            self.bit_depth,
            self.color_type,
            self.compression_method,
            self.filter_method,
            self.interlace_method
        )
    }
}

#[derive(Debug)]
pub enum IhdrChunkError {
    InvalidLength,
}
impl std::error::Error for IhdrChunkError {}

impl std::fmt::Display for IhdrChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IhdrChunkError::InvalidLength => write!(
                f,
                "IHDR chunk must be exactly {} bytes long",
                IhdrChunk::CHUNK_LENGTH
            ),
        }
    }
}

impl TryFrom<Chunk> for IhdrChunk {
    type Error = Error;

    fn try_from(chunk: Chunk) -> Result<Self> {
        let bytes = chunk.data.clone();

        if chunk.len() != IhdrChunk::CHUNK_LENGTH {
            return Err(IhdrChunkError::InvalidLength.into());
        }

        Ok(IhdrChunk {
            width: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            height: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            bit_depth: bytes[8],
            color_type: bytes[9],
            compression_method: bytes[10],
            filter_method: bytes[11],
            interlace_method: bytes[12],
        })
    }
}
