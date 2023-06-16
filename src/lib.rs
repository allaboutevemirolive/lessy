pub mod data_processing;
pub mod file_utils;

use data_processing::{process_data, insert_blank_spaces};
use file_utils::{format_output_file_name, read_input_file, write_output_file};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "input.txt";
    let output_file = "output.md";
    let text_to_be_replaced = "Click here to view code image";

    let words_to_delete = &[
        r#"\[[0-9]+\]"#,
        r"The availability of a high-level"
        // r"separator"
    ]; // Insert multiple regex expressions here

    let output_file = format_output_file_name(output_file)?;

    
    let data = read_input_file(input_file)?;
    let new_text = process_data(&data, &text_to_be_replaced, words_to_delete)?;
    
    // let new_text = insert_blank_spaces(&new_text);

    write_output_file(&output_file, &new_text)?;

    Ok(())
}