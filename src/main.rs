use std::fs;
use std::io;

pub struct Parser {
    data: Vec<String>,
    // ty_com: CommParser
    // supported_language

    // Comment -> Human readable text
    // Human readable text -> Comment
}

// TODO
pub struct CommParser {
    single_line: String,
    multiline_comment: String,
    documentation_comments: String,
}

impl CommParser {
    pub fn new() -> CommParser {
        CommParser {
            single_line: String::new(),
            multiline_comment: String::new(),
            documentation_comments: String::new(),
        }
    }
}

impl Parser {
    pub fn new(data: Vec<String>) -> Parser {
        Parser { data }
    }
}

impl Parser {
    /// Delete comment
    fn delete_comment(&self) -> String {
        let mut new_data = String::new();

        let double_slash = "//";
        let triple_slash = "///";

        for line in &self.data {
            if line.find(double_slash).is_none() && line.find(triple_slash).is_none() {
                // No "//" or "///" found in the line, so add the entire line.
                new_data += line;
                new_data.push('\n'); // Add a newline to separate lines.
            } else if line.find(double_slash).is_some() {
                let start_index = line.find(double_slash).unwrap();
                let text_before_double_slash: String = line.chars().take(start_index).collect();
                
                if text_before_double_slash.trim().is_empty() {
                    // new_data.push('\n');
                    continue;
                }
                new_data += &text_before_double_slash;
                new_data.push('\n'); // Add a newline after adding the text before "//".
            } else if line.find(triple_slash).is_some() {
                let start_index = line.find(triple_slash).unwrap();
                let text_before_triple_slash: String = line.chars().take(start_index).collect();
                if text_before_triple_slash.trim().is_empty() {
                    // new_data.push('\n');
                    continue;
                }
                new_data += &text_before_triple_slash;
                new_data.push('\n'); // Add a newline after adding the text before "///".
            }
        }

        new_data
    }

    // Convert comment into human readable text

    // Convert human readable text into comment
}

fn main() -> io::Result<()> {
    let file_path = "/home/nemesis/Documents/Github/Focus/util/lessy/input_commented.txt";

    let file_content = fs::read_to_string(&file_path)?;

    let lines: Vec<String> = file_content.lines().map(String::from).collect();

    let parser_com = Parser::new(lines);

    let result = parser_com.delete_comment();

    fs::write(&file_path, result)?;

    Ok(())
}
