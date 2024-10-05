use std::{fs, path::PathBuf, str::FromStr};

use crate::{
    args::{Cli, Commands},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
    Result,
};

pub fn run(args: &Cli) -> Result<()> {
    match &args.command {
        Commands::Encode {
            png_file: file_path,
            chunk_type,
            message,
        } => encode(file_path, chunk_type, message)?,

        Commands::Decode {
            png_file: file_path,
            chunk_type,
        } => decode(file_path, chunk_type)?,

        Commands::Remove {
            png_file: file_path,
            chunk_type,
        } => remove(&file_path, &chunk_type)?,

        Commands::Print { png_file } => print(png_file)?,
        Commands::Verify { png_file } => verify(png_file)?,
    }

    Ok(())
}

fn encode(file_path: &PathBuf, chunk_type: &str, message: &str) -> Result<()> {
    if file_path.extension().unwrap() != "png" {
        return Err("This program takes only PNG files".into());
    }

    let file = fs::read(file_path)?;

    let mut png = Png::try_from(file.as_slice())?;

    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

    png.append_chunk(chunk);

    let _ = fs::write(file_path, png.as_bytes());

    println!("Message encoded successfully!");

    Ok(())
}

fn decode(file_path: &PathBuf, chunk_type: &str) -> Result<()> {
    if file_path.extension().unwrap() != "png" {
        return Err("This program takes only PNG files".into());
    }

    let file = fs::read(file_path)?;

    let png = Png::try_from(file.as_slice())?;

    match png.chunk_by_type(chunk_type) {
        Some(chunk) => {
            println!("Message: {:?}", chunk.data_as_string().unwrap());
        }
        None => println!("No message hidden in this image with this chunk type"),
    }

    Ok(())
}

fn remove(file_path: &PathBuf, chunk_type: &str) -> Result<()> {
    if file_path.extension().unwrap() != "png" {
        return Err("This program takes only PNG files".into());
    }

    let file = fs::read(file_path)?;

    let mut png = Png::try_from(file.as_slice())?;

    png.remove_first_chunk(chunk_type)?;

    let _ = fs::write(file_path, png.as_bytes())?;

    println!("Message has been removed successfully!");

    Ok(())
}

fn print(file_path: &PathBuf) -> Result<()> {
    if file_path.extension().unwrap() != "png" {
        return Err("This program takes only PNG files".into());
    }

    let file = fs::read(file_path)?;

    let png = Png::try_from(file.as_slice())?;
    println!("{}", png);

    Ok(())
}

fn verify(file_path: &PathBuf) -> Result<()> {
    if file_path.extension().unwrap() != "png" {
        return Err("This program takes only PNG files".into());
    }

    let file = fs::read(file_path)?;

    let png = Png::try_from(file.as_slice())?;

    if png.verify() {
        println!("File is a valid PNG");
    } else {
        println!("File is not a valid PNG");
    }

    Ok(())
}
