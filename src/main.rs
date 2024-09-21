use std::str::FromStr;

use chunk_type::ChunkType;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let str = "This is where your secret message will be!"
        .as_bytes()
        .to_vec();

    let crc32 = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    let chunk_type = ChunkType::from_str("RuSt").unwrap();
    let bytes: Vec<_> = chunk_type
        .bytes()
        .iter()
        .chain(str.iter())
        .copied()
        .collect();
    let crcc = crc32.checksum(&bytes);
    println!("{crcc:?}");

    println!("{:?}", list_files("."));

    Ok(())
}

fn list_files(path: &str) -> Result<Vec<String>> {
    let files = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    Ok(files)
}
