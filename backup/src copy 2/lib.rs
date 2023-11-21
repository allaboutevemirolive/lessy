mod init;
pub mod process;
pub mod util;
mod com;
use com::{CommentProcessor, CommFunc};
use init::*;
use util::{FileManager, FileWriter};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {

    let text_to_be_replaced = "Click here to view code image";
    let words_to_delete = vec![r#"\[[0-9]+\]"#];
    let file_manager = FileManager::new("input.txt", "output.md");
    let skip_code = false;
    // ignore after_dot_sign

    let main_init = Initializer::new(
        file_manager,
        text_to_be_replaced.to_string(),
        words_to_delete,
        skip_code,
    );

    main_init.run_init()?;

    Ok(())
}

pub fn run_comment() -> Result<(), Box<dyn std::error::Error>> {

    let file_manager = FileManager::new("input_commented.txt", "output_uncommented.md");

    // Read file
    let input_file = file_manager.get_input();
    let main_text = FileManager::input_reader(&input_file).unwrap();

    let str = CommentProcessor::new(main_text.to_owned());
    let result = str.delete_comment();

    // Generate output file
    let output_file = file_manager.get_output();
    FileManager::output_writer(output_file, &result).unwrap();

    Ok(())
}
