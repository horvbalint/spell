mod args;
mod chunk;
mod chunk_type;
mod png;

use anyhow::Result;
use clap::Parser;
use snailshell::snailprint;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    str::FromStr,
};

use args::{Args, Command};
use chunk::Chunk;
use chunk_type::ChunkType;
use png::Png;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Hide {
            input_path,
            message,
            chunk_type,
            output_path,
        } => hide_msg(input_path, message, chunk_type, output_path),
        Command::Find { path, chunk_type } => find_msg(path, chunk_type),
        Command::Delete { path, chunk_type } => remove_msg(path, chunk_type),
    }?;

    Ok(())
}

fn hide_msg(
    input_path: String,
    message: String,
    chunk_type: String,
    output_path: Option<String>,
) -> Result<()> {
    let mut png = get_png_from_path(&input_path)?;

    let chunk_type = ChunkType::from_str(&chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.into_bytes());

    png.append_chunk(chunk);

    let path = PathBuf::from(output_path.unwrap_or(input_path));
    fs::write(path, png.as_bytes())?;

    Ok(())
}

fn find_msg(path: String, chunk_type: String) -> Result<()> {
    let png = get_png_from_path(&path)?;
    let chunk_type = ChunkType::from_str(&chunk_type)?;

    let mut messages = vec![];
    for chunk in png.chunks() {
        if *chunk.chunk_type() == chunk_type {
            let msg = chunk.data_as_string()?;
            messages.push(msg);
        }
    }

    snailprint(messages.join("\n"));

    Ok(())
}

fn remove_msg(path: String, chunk_type: String) -> Result<()> {
    let mut png = get_png_from_path(&path)?;
    png.remove_chunks(&chunk_type);

    fs::write(path, png.as_bytes())?;

    Ok(())
}

fn get_png_from_path(input_path: &str) -> Result<Png> {
    let path = PathBuf::from(input_path);
    let mut bytes = vec![];
    File::open(path)?.read_to_end(&mut bytes)?;
    let png = Png::try_from(bytes.as_ref())?;

    Ok(png)
}
