use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

pub fn read_input_file(input_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut data = String::new();
    buf_reader.read_to_string(&mut data)?;
    Ok(data)
}

pub fn write_output_file(
    output_file: &Path,
    new_text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output_file)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(new_text.as_bytes())?;
    Ok(())
}

pub fn format_output_file_name(output_file: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output_file = output_file.replace(" ", "_").to_lowercase();
    Ok(Path::new(&output_file).to_path_buf())
}
