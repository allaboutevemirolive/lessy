use crate::{process::*, util::*};

pub trait MainInitializer {
    fn run_init(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Initializer<'a> {
    input_output: FileManager,
    text_to_be_replaced: String,
    words_to_delete: Vec<&'a str>,
    skip_code: bool,
}

impl<'a> Initializer<'a> {
    pub fn new(
        input_output: FileManager,
        text_to_be_replaced: String,
        words_to_delete: Vec<&str>,
        skip_code: bool,
    ) -> Initializer {
        Initializer {
            input_output,
            text_to_be_replaced: text_to_be_replaced.to_string(),
            words_to_delete,
            skip_code,
        }
    }
}

impl<'a> MainInitializer for Initializer<'a> {
    fn run_init(&self) -> Result<(), Box<dyn std::error::Error>> {

        let input_file = &self.input_output.get_input();
        let main_text = FileManager::input_reader(&input_file)?;

        let text_to_be_replaced = &self.text_to_be_replaced;

        // Convert &str to String explicitly |  Vec<&str>  =>  Vec<String>
        let words_to_delete = self.words_to_delete.iter().map(|s| s.to_string()).collect();

        let processor = TextProcessor::new(
            &main_text,
            text_to_be_replaced,
            words_to_delete,
            self.skip_code,
        );

        let new_text = &TextProcessor::process_data(&processor)?;
        let output_file = &self.input_output.get_output();

        FileManager::output_writer(output_file, new_text)?;

        Ok(())
    }
}