pub mod data_processing;
pub mod file_utils;

use data_processing::{process_data};
use file_utils::{format_output_file_name, read_input_file, write_output_file};

pub fn run(
    input_file: &str,
    output_file: &str,
    text_to_be_replaced: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Rest of your code
    let output_file = format_output_file_name(output_file)?;

    let data = read_input_file(input_file)?;
    let new_text = process_data(&data, text_to_be_replaced)?;

    write_output_file(&output_file, &new_text)?;

    Ok(())
}

