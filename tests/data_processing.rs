/// Check if the character is a symbol.
pub fn is_symbol(c: char) -> bool {
    let symbols = "/\\:*?\"<>|!@#$%^&()-+=[]{};,.\'~`";
    symbols.contains(c)
}

/// Delete entire lines containing the specified text.
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

/// Process the data by deleting lines, replacing text, and adding newlines.
pub fn process_data(data: &str, text_to_be_replaced: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = delete_entire_line(&data, &text_to_be_replaced);
    let data = data.replace(". ", ".\n");

    let mut inside_braces = false;
    let mut new_text = String::new();

    let mut data_chars = data.chars().peekable();
    while let Some(c) = data_chars.next() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol() {
        assert!(is_symbol('$'));
        assert!(!is_symbol('a'));
    }

    #[test]
    fn test_delete_entire_line() {
        let data = "Line 1\nLine 2\nLine 3\n";
        let text_to_be_deleted = "Line 2";
        let expected_result = "Line 1\nLine 3\n";
        assert_eq!(delete_entire_line(data, text_to_be_deleted), expected_result);
    }

    #[test]
    fn test_process_data() {
        let data = "Hello. This is a test.";
        let text_to_be_replaced = "This is";
        let expected_result = "Hello.\nThis is a test.";
        assert_eq!(process_data(data, text_to_be_replaced).unwrap(), expected_result);
    }
}
