pub mod data_processing;
pub mod file_utils;
mod test_module;

use data_processing::{insert_blank_spaces, process_data};
use file_utils::{format_file_name, format_output_file_name, read_input_file, write_output_file};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "input.txt";

    let file_name = "output";
    let file_type = ".md";
    let formatted_file_name = format_file_name(&file_name);
    let file_output = formatted_file_name.to_owned() + file_type;

    let output_file = format_output_file_name(&file_output).unwrap();

    // Feature need to be add
    // - Insert multiple words/sentences
    let text_to_be_replaced = "Click here to view code image";

    // Insert multiple regex expressions here
    let words_to_delete = &[
        r#"\[[0-9]+\]"#, 
        // r"The availability of a high-level"
        // r"separator"
    ];
    // Feature that will be add:
    // - Format string containing symbol that cannot be use in file creation
    // let output_file = format_output_file_name(file_name)?;

    let data = read_input_file(input_file)?;
    let new_text = process_data(&data, &text_to_be_replaced, words_to_delete)?;

    let new_text = insert_blank_spaces(&new_text);

    write_output_file(&output_file, &new_text)?;

    Ok(())
}
