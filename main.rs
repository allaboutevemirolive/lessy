mod file_utils;
mod data_processing;

use file_utils::*;
use data_processing::*;


use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    lessy::run()
}


fn format_output_file_name(output_file: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output_file = output_file.replace(" ", "_").to_lowercase();
    Ok(Path::new(&output_file).to_path_buf())
}