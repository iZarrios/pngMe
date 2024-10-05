use std::{fmt::Display, str::FromStr};

use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Types {
    IHDR,
    IDAT,
    PLTE,
    IEND,
    ANCILLARY,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    pub typ: Types,
    pub code: [u8; 4],
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.code).unwrap_or_default())
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err(Box::from(ChunkTypeError::InvalidLength));
        }

        let code = s.as_bytes();

        if !code
            .iter()
            .all(|&byt| byt.is_ascii() & byt.is_ascii_alphabetic())
        {
            return Err(Box::from(ChunkTypeError::NotASCII));
        }

        let mut chunk_code = [0u8; 4];
        chunk_code.copy_from_slice(code);

        let typ = ChunkType::_get_type_from_code(chunk_code);

        Ok(ChunkType {
            code: chunk_code,
            typ,
        })
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        let typ = ChunkType::_get_type_from_code(value);
        Ok(ChunkType { code: value, typ })
    }
}

#[allow(unused)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.code
    }

    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        self.code.iter().all(|&byt| byt.is_ascii()) & self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.code[0].is_ascii_uppercase()
    }
    pub fn is_public(&self) -> bool {
        self.code[1].is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.code[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        !self.code[3].is_ascii_uppercase()
    }

    fn _get_type_from_code(code: [u8; 4]) -> Types {
        match code {
            [73, 72, 68, 82] => Types::IHDR,
            [73, 68, 65, 84] => Types::IDAT,
            [80, 76, 84, 69] => Types::PLTE,
            [73, 69, 78, 68] => Types::IEND,
            _ => Types::ANCILLARY,
        }
    }
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidLength,
    NotASCII,
}

impl std::error::Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ChunkTypeError::InvalidLength => {
                write!(f, "Input must be exactly 4 bytes/characters long")
            }
            ChunkTypeError::NotASCII => write!(f, "Not all data is ASCII"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
