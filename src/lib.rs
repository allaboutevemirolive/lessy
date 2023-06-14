pub mod file_utils;
pub mod data_processing;

use file_utils::{format_output_file_name, read_input_file, write_output_file};
use data_processing::process_data;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "input.txt";
    let output_file = "11.2. Controlling How Tests Are Run.md";
    let text_to_be_replaced = "Click here to view code image";
    let output_file = format_output_file_name(output_file)?;

    let data = read_input_file(input_file)?;
    let new_text = process_data(&data, &text_to_be_replaced)?;

    write_output_file(&output_file, &new_text)?;

    Ok(())
}
