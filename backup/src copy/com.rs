use crate::util::FileManager;

pub trait CommFunc {
    fn delete_comment(&self) -> String;
}

// We use FileManager to Read file and generate output file
pub struct CommentProcessor {
    // Read file and generate output file
    // input_output: FileManager,
    // Code + comment
    data: String,
    // Comment -> Human readable text
    // Human readable text -> Comment
}

impl CommentProcessor {
    pub fn new(
        // input_output: FileManager,
        data: String
    ) -> CommentProcessor {
        CommentProcessor { 
            // input_output, 
            data 
        }
    }
}

impl CommFunc for CommentProcessor {
    /// Delete comment
    fn delete_comment(&self) -> String {

        let data = &self.data;
        if data.is_empty() {
            return data.to_string();
        }

        let mut new_data = String::new();

        let double_slash = "//";
        let triple_slash = "///";

        for line in data.lines() {
            if line.find(double_slash).is_none() && line.find(triple_slash).is_none() {
                // No "//" or "///" found in the line, so add the entire line.
                new_data += line;
                new_data.push('\n'); // Add a newline to separate lines.
                continue;
            } else if line.find(double_slash).is_some() {
                let start_index = line.find(double_slash).unwrap();
                let text_before_double_slash: String = line.chars().take(start_index).collect();
                if text_before_double_slash.trim().is_empty() {
                    // new_data.push('\n');
                    continue;
                }
                new_data += &text_before_double_slash;
                new_data.push('\n'); // Add a newline after adding the text before "//".
                continue;
            } else if line.find(triple_slash).is_some() {
                let start_index = line.find(triple_slash).unwrap();
                let text_before_triple_slash: String = line.chars().take(start_index).collect();
                if text_before_triple_slash.trim().is_empty() {
                    // new_data.push('\n');
                    continue;
                }
                new_data += &text_before_triple_slash;
                new_data.push('\n'); // Add a newline after adding the text before "///".
                continue;
            }
        }

        new_data
    }

    // Convert comment into human readable text

    // Convert human readable text into comment
}
