use regex::Regex;

// If the char after current dot sign is a symbol, then don't create a new blank space
pub fn is_symbol(c: char) -> bool {
    let symbols = "/\\:*?\"<>|!@#$%^&()-+=[]{};,.\'~`";
    symbols.contains(c)
}

// Delete entire line the contain the current the current sentence/word
pub fn delete_entire_line(data: &str, text_to_be_deleted: &str) -> String {
    if text_to_be_deleted.is_empty() || data.is_empty() {
        return data.to_string();
    }

    let mut new_data = String::new();
    let lines: Vec<&str> = data.split('\n').collect();

    for line in lines {
        if line.find(text_to_be_deleted).is_none() {
            new_data.push_str(line);
            new_data.push('\n');
        }
    }

    new_data
}

// TODO: Can be improve

// Delete specific word with regex
pub fn delete_specific_words(data: &str, words_to_be_deleted: &[&str]) -> String {
    if data.is_empty() || words_to_be_deleted.is_empty() {
        return data.to_string();
    }
    let mut new_data = String::new();
    let lines: Vec<&str> = data.split('\n').collect();

    // let re_words = Regex::new(&format!(r"({})", words_to_be_deleted.join("|"))).unwrap();

    let pattern = words_to_be_deleted
        .iter()
        .map(|word| format!(r"\b{}\b", regex::escape(word)))
        .collect::<Vec<String>>()
        .join("|");

    let re_words = Regex::new(&pattern).unwrap();

    for line in lines {
        // Cleaning line from pattern
        let line_without_brackets = re_words.replace_all(line, "");

        // Cow => &str
        new_data.push_str(&line_without_brackets.to_string());
        // new_data.push_str("\n");
    }

    // Remove trailing whitespace and extra space before newline
    new_data = new_data.to_string();

    new_data
}

// Main string processor
pub fn process_data(
    data: &str,
    text_to_be_replaced: &str,
    words_to_delete: &[&str],
) -> Result<String, Box<dyn std::error::Error>> {
    let data = delete_specific_words(&data, &words_to_delete);
    let data = delete_entire_line(&data, &text_to_be_replaced);

    // Remove double/triple trail white space to
    // avoid making new paragraph with with extra space
    let data = data.replace(". ", ".");

    let mut inside_braces = false;
    let mut new_text = String::new();
    let mut prev_char = 'a';

    let mut data_chars = data.chars().peekable();

    while let Some(c) = data_chars.next() {
        // For sentence/word that is inside open
        // and close curly braces, do not format it.
        // Probably it is snippet
        match c {
            '{' => {
                inside_braces = true;
                new_text.push(c);
            }
            '}' => {
                inside_braces = false;
                new_text.push(c);
            }
            _ if !inside_braces => match c {
                '.' => {
                    new_text.push('.');

                    if let Some(&next_char) = data_chars.peek() {
                        after_dot_sign(next_char, &mut new_text, prev_char);
                    }

                    prev_char = c;
                }
                _ => {
                    new_text.push(c);
                    prev_char = c;
                }
            },
            _ => new_text.push(c),
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
pub fn insert_blank_spaces(text: &str) -> String {
    let mut result = String::new();

    // Original vector
    let lines_original: Vec<&str> = text.lines().collect();

    for i in 0..(lines_original.len() - 1) {
        result.push_str(&format!("{}\n", lines_original[i]));
    }

    result
}

// After dot sign, next char is:
// - digit
// - also dot sign
// - contains symbol(e.g "./")

// Future feature should be add:
// - for current dot sign, if the char before and after it is Uppercase. E.g "S.R/M.R"
// - A list of word that should not be format by the code since it has meaning in it.

pub fn after_dot_sign(next_char: char, new_text: &mut String, prev_char: char) {
    if !next_char.is_lowercase()
        && !next_char.is_digit(10)
        && next_char != '.'
        && !is_symbol(next_char)
        && !(prev_char.is_uppercase() && next_char.is_uppercase())
    {
        new_text.push('\n');
        new_text.push('\n');
    }
}

// Before format the sentence or text, check if there is blank space in the text
// as it is indicator for new paragraph.
// If it is, replace the parapraph with 3 paragraph between it is "---"
// Example: "\n" replace with "\n---\n"
// The purpose is that to separate first the original paragraph
// and to know when is the end of current paragraph for the current separated text
