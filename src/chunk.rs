use crate::{chunk_type, ihdr_chunk::IhdrChunk, Error, Result};
use std::{fmt::Display, str::FromStr};

use crate::chunk_type::ChunkType;
use crc;

#[derive(Debug)]
pub enum ChunkError {
    InvalidUtf8,
    InvalidChunkType,
    InvalidCrc,
    TooShort,
}
impl std::error::Error for ChunkError {}

impl std::fmt::Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChunkError::InvalidUtf8 => write!(f, "Invalid UTF-8 in chunk type"),
            ChunkError::InvalidChunkType => write!(f, "Invalid chunk type"),
            ChunkError::InvalidCrc => write!(f, "Invalid CRC"),
            ChunkError::TooShort => write!(f, "Input data must be at least 12 bytes long"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    len: u32,
    chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub crc: u32,
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.chunk_type.typ {
            chunk_type::Types::IHDR => {
                write!(f, "{}", IhdrChunk::try_from(self.clone()).unwrap())
            }
            chunk_type::Types::IDAT => {
                writeln!(
                    f,
                    "Chunk: Data_len={}, Data={:?} , type={}, crc={}",
                    self.len, self.data, self.chunk_type, self.crc
                )
            }
            chunk_type::Types::IEND => writeln!(f, "IEND : END OF IMAGE"),
            _ => writeln!(
                f,
                "Chunk: Data_len={}, type={}, crc={}",
                self.len, self.chunk_type, self.crc
            ),
        }
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Chunk> {
        let vc = value.to_vec();

        // check if the input slice is at least 12 bytes long
        if vc.len() < 12 {
            return Err(ChunkError::TooShort.into());
        }

        // first 4 bytes is the length of the data
        let len = u32::from_be_bytes([vc[0], vc[1], vc[2], vc[3]]);

        // next 4 bytes is the chunk type
        let chunk_type_bytes = &vc[4..8];
        let chunk_type_str = String::from_utf8(chunk_type_bytes.to_vec())
            .map_err(|_| "Invalid UTF-8 in chunk type")?;

        let chunk_type = ChunkType::from_str(&chunk_type_str)?;

        // next n bytes is the data
        let data = vc[8..vc.len() - 4].to_vec();

        // last 4 bytes is the crc
        let crc = u32::from_be_bytes([
            vc[vc.len() - 4],
            vc[vc.len() - 3],
            vc[vc.len() - 2],
            vc[vc.len() - 1],
        ]);

        // create a CRC instance and validate the checksum
        let crc32 = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

        let bytes: Vec<_> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        if crc32.checksum(&bytes) == crc {
            Ok(Self {
                len,
                chunk_type,
                data,
                crc,
            })
        } else {
            Err(ChunkError::InvalidCrc.into())
        }
    }
}

#[allow(unused)]
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

        let bytes: Vec<_> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        Self {
            len: data.len() as u32,
            chunk_type,
            data: data.clone(),
            crc: crc32.checksum(&bytes),
        }
    }
    pub fn length(&self) -> u32 {
        self.len
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.clone())?)
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.len.to_be_bytes());
        bytes.extend_from_slice(&self.chunk_type.bytes());
        bytes.extend_from_slice(&self.data);
        bytes.extend_from_slice(&self.crc.to_be_bytes());

        bytes
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn type_str(&self) -> String {
        self.chunk_type.code.iter().map(|&b| b as char).collect()
    }

    pub fn get_type(&self) -> chunk_type::Types {
        self.chunk_type.typ.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chnk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        chnk
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
