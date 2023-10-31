use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

pub trait FileWriter {
    fn input_reader(input_file: &str) -> Result<String, Box<dyn std::error::Error>>;
    fn output_writer(output_file: &str, new_text: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct FileManager {
    input: String,
    output: String,
}

impl FileManager {
    pub fn new(input: &str, output: &str) -> Self {
        FileManager {
            input: input.to_string(),
            output: output.to_string(),
        }
    }

    pub fn get_input(&self) -> &String {
        &self.input
    }

    pub fn get_output(&self) -> &String {
        &self.output
    }
}

impl FileWriter for FileManager {
    fn input_reader(input_file: &str) -> Result<String, Box<dyn std::error::Error>> {
        let file = File::open(input_file)?;
        let mut buf_reader = BufReader::new(file);
        let mut data = String::new();
        buf_reader.read_to_string(&mut data)?;
        Ok(data)
    }

    fn output_writer(output_file: &str, new_text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(output_file)?;
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(new_text.as_bytes())?;
        Ok(())
    }
}
