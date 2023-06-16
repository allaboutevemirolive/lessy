
pub fn is_symbol(c: char) -> bool {
    let symbols = "/\\:*?\"<>|!@#$%^&()-+=[]{};,.\'~`";
    symbols.contains(c)
}

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

pub fn process_data(
    data: &str,
    text_to_be_replaced: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let data = delete_entire_line(&data, &text_to_be_replaced);
    let data = data.replace(". ", ".\n");
    // let data = insert_blank_spaces(&data);

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
                            // new_text.push('\n');
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