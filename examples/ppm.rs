use std::fs;
use fs::File;
use std::io::{BufWriter, Write, Result};
use std::path::PathBuf;

use rolivec::Framebuffer;

const DEFAULT_PATH: &str = "output";

pub fn write(fb: &Framebuffer, filename: &str) -> Result<()> {
    let mut path = PathBuf::from(DEFAULT_PATH);
    fs::create_dir_all(&path)?;
    path.push(filename);

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    write!(writer, "P6\n{} {}\n255\n", fb.width(), fb.height())?;
    for color in fb.pixels() {
        writer.write_all(&[color.r, color.g, color.b])?;
    }

    writer.flush()?;
    Ok(())
}

pub fn reset() -> Result<()> {
    let path = PathBuf::from(DEFAULT_PATH);

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}
