use regex::Regex;

// If the char after current dot sign is a symbol, then don't create a new blank space
pub fn is_symbol(c: char) -> bool {
    let symbols = "/\\:*?\"<>|!@#$%^&()-+=[]{};,.\'~`";
    symbols.contains(c)
}

// Delete entire line the contain the current the current sentence/word
pub fn delete_entire_line(data: &str, text_to_be_deleted: &str) -> String {
    let mut new_data = String::new();
    let lines: Vec<&str> = data.split('\n').collect();

    for line in lines {
        if !line.contains(text_to_be_deleted) {
            new_data.push_str(line);
            new_data.push('\n');
        }
    }

    new_data
}

// Delete specific word with regex
pub fn delete_specific_words(data: &str, words_to_be_deleted: &[&str]) -> String {
    let mut new_data = String::new();
    let lines: Vec<&str> = data.split('\n').collect();

    let re_words = Regex::new(&format!(r"({})", words_to_be_deleted.join("|"))).unwrap();

    for line in lines {
        let mut modified_line = String::new();
        let line_without_brackets = re_words.replace_all(line, "");

        for word in line_without_brackets.split(' ') {
            modified_line.push_str(word);
            modified_line.push(' ');
        }

        // Remove trailing whitespace
        modified_line.pop();

        new_data.push_str(&modified_line);
        new_data.push(' ');
    }

    // Remove trailing whitespace and extra space before newline
    new_data = new_data.trim_end().replace(" \n", "\n").to_string();

    new_data
}

// Main string processor
pub fn process_data(
    data: &str,
    text_to_be_replaced: &str,
    words_to_delete: &[&str]
) -> Result<String, Box<dyn std::error::Error>> {
    let data = delete_specific_words(&data, &words_to_delete);
    let data = delete_entire_line(&data, &text_to_be_replaced);
    
    // Remove double/triple trail white space to avoid making new paragraph with with extra space
    let data = data.replace("   ", " ");
    let data = data.replace("  ", " ");
    let data = data.replace(". ", ".\n");

    // let data = insert_blank_spaces(&data);

    let mut inside_braces = false;
    let mut new_text = String::new();

    let mut data_chars = data.chars().peekable();
    while let Some(c) = data_chars.next() {
        // For sentence/word that is inside open and close curly braces, 
        // do not format it.
        // Probably it is snippet
        if c == '{' {
            inside_braces = true;
            new_text.push(c);
        } else if c == '}' {
            inside_braces = false;
            new_text.push(c);
        } else if !inside_braces {
            match c {
                '.' => {
                    new_text.push('.');
                    if let Some(&next_char) = data_chars.peek() {
                        // After dot sign, next char is:
                        // - digit
                        // - also dot sign
                        // - contains symbol(e.g "./")

                        // Future feature should be add:
                        // - for current dot sign, if the char before and after it is Uppercase. E.g "S.R/M.R"
                        // - A list of word that should not be format by the code since it has meaning in it.
                        if !next_char.is_lowercase()
                            && !next_char.is_digit(10)
                            && next_char != '.'
                            && !is_symbol(next_char)
                        {
                            new_text.push('\n');
                        }
                    }
                }
                _ => new_text.push(c),
            }
        } else {
            new_text.push(c);
        }
    }

    Ok(new_text)
}

// There is a time that the code wont make a blank space between 2 sentences. 
// It because the next sentence is underly the current sentence, not attach to current sentence.
// In order to fix this, code need to detect:
// - If current line contains dot sign at the end of it
// - The sentence underly the current sentence, has dot sign at the end of it
// - If both requirements is meet then separate both sentence with a blank space
//
// One main problem is that, the code need to distint between 
// - sentence that end with dot sign is "sentence" 
// - sentence that end with dot sign is not a "sentence" e.g snippet contains dot sign
#[allow(dead_code)]
pub fn insert_blank_spaces(text: &str) -> String {
    let mut result = String::new();
    let lines: Vec<&str> = text.lines().collect();

    for i in 0..lines.len() {
        let current_line = lines[i].trim();
        result.push_str(current_line);

        let next_line_index = i + 1;
        if next_line_index < lines.len() {
            let next_line = lines[next_line_index].trim();

            if current_line.ends_with('.') && next_line.ends_with('.') {
                result.push('\n');
                result.push('\n');
            } else {
                result.push(' ');
            }
        } else {
            result.push('\n');
        }
    }

    result
}
