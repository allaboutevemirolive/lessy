use regex::Regex;

pub trait Manipulator {
    fn delete_specific_words(data: &str, words_to_be_deleted: &[String]) -> String;
    fn delete_entire_line(data: &str, text_to_be_deleted: &str) -> String;
    fn is_symbol(c: char) -> bool;
    fn insert_blank_spaces(text: &str) -> String;
    fn after_dot_sign(next_char: char, new_text: &mut String, prev_char: char);
}

pub trait ProcessData {
    fn process_data(&self) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct TextProcessor {
    pub main_text: String,
    pub text_to_be_replaced: String,
    pub words_to_delete: Vec<String>,
    pub skip_code: bool,
}

impl TextProcessor {
    pub fn new(
        main_text: &str,
        text_to_be_replaced: &str,
        words_to_delete: Vec<String>,
        skip_code: bool,
    ) -> Self {
        TextProcessor {
            main_text: main_text.to_string(),
            text_to_be_replaced: text_to_be_replaced.to_string(),
            words_to_delete,
            skip_code,
        }
    }
}

impl Manipulator for TextProcessor {
    /// Delete specific word with regex
    fn delete_specific_words(data: &str, words_to_be_deleted: &[String]) -> String {
        if data.is_empty() || words_to_be_deleted.is_empty() {
            return data.to_string();
        }

        let re_words = Regex::new(&format!(r"({})", words_to_be_deleted.join("|"))).unwrap();
        let mut new_data = String::new();

        for line in data.lines() {
            let line_without_brackets = re_words.replace_all(line, "");
            new_data += &line_without_brackets;
            new_data.push('\n');
        }

        new_data
    }

    /// Delete entire line the contain the target sentence/word
    fn delete_entire_line(data: &str, text_to_be_deleted: &str) -> String {
        if text_to_be_deleted.is_empty() || data.is_empty() {
            return data.to_string();
        }

        let mut new_data = String::new();

        for line in data.lines() {
            if line.find(text_to_be_deleted).is_none() {
                new_data += line;
                new_data.push('\n');
            }
        }

        new_data
    }

    /// If the char after current dot sign is a symbol, then don't
    /// create a new blank space
    fn is_symbol(c: char) -> bool {
        let symbols = "/\\:*?\"<>|!@#$%^&()-+=[]{};,.'~`";
        symbols.contains(c)
    }

    /// There is a time that the code wont make a blank space between 2 sentences.
    /// It because the next sentence is underly the current sentence, not attach to current sentence.
    /// In order to fix this, code need to detect:
    /// - If current line contains dot sign at the end of it
    /// - The sentence underly the current sentence, has dot sign at the end of it
    /// - If both requirements is meet then separate both sentence with a blank space
    ///
    /// One main problem is that, the code need to distint between
    /// - sentence that end with dot sign is "sentence"
    /// - sentence that end with dot sign is not a "sentence" e.g snippet contains dot sign
    fn insert_blank_spaces(text: &str) -> String {
        let lines_original: Vec<&str> = text.lines().collect();
        let mut result = String::new();

        for (i, line) in lines_original.iter().enumerate() {
            result.push_str(line);
            if i < lines_original.len() - 1 {
                result.push('\n');
            }
        }

        result
    }

    /// After dot sign, next char is:
    /// - digit
    /// - also dot sign
    /// - contains symbol (e.g "./")
    ///
    /// Future feature should be add:
    /// - for current dot sign, if the char before and after it is Uppercase. E.g "S.R/M.R"
    /// - A list of word that should not be format by the code since it has meaning in it.
    fn after_dot_sign(next_char: char, new_text: &mut String, prev_char: char) {
        if !next_char.is_lowercase()
            && !next_char.is_digit(10)
            && next_char != '.'
            && !Self::is_symbol(next_char)
            && !(prev_char.is_uppercase() && next_char.is_uppercase())
        {
            new_text.push('\n');
            new_text.push('\n');
        }
    }
}

impl ProcessData for TextProcessor {
    /// Main processor
    fn process_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        // TODO: Potential error process text
        // Bad design?
        // We cannot invoke this function in "else if char" below,
        // because we want all item should be scan first
        // so there is no missed item
        let del_spec_word = <TextProcessor as Manipulator>::delete_specific_words(
            &self.main_text,
            &self.words_to_delete,
        );
        let del_enti_line = <TextProcessor as Manipulator>::delete_entire_line(
            &del_spec_word,
            &self.text_to_be_replaced,
        );
        // Remove double/triple trail white space to
        // avoid making new paragraph with with extra space
        let ref_dot = del_enti_line.replace(". ", ".");

        let mut inside_braces = false;
        let mut new_text = String::new();
        let mut prev_char = 'a';

        let mut ref_dot = ref_dot.chars().peekable();

        // This snippet seems a bit bloated but it does what it mean to be
        while let Some(char) = ref_dot.next() {
            // For sentence/word that is inside open
            // and close curly braces, do not format it.
            // Probably it is snippet
            match char {
                '{' if (self.skip_code == true) => {
                    inside_braces = true;
                    new_text.push(char);
                }
                '}' if (self.skip_code == true) => {
                    inside_braces = false;
                    new_text.push(char);
                }
                _ if inside_braces == false => {
                    if char == '.' {
                        new_text.push('.');

                        if let Some(&next_char) = ref_dot.peek() {
                            <TextProcessor as Manipulator>::after_dot_sign(
                                next_char,
                                &mut new_text,
                                prev_char,
                            );
                        }

                        prev_char = char;
                    } else {
                        new_text.push(char);
                        prev_char = char;
                    }
                }
                _ => new_text.push(char),
            }
        }

        let new_text = <TextProcessor as Manipulator>::insert_blank_spaces(&new_text);

        Ok(new_text)
    }
}
