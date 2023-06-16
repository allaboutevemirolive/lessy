use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "input.txt";
    let output_file = "11.2. Controlling How Tests Are Run.md";
    let text_to_be_replaced = "Click here to view code image";
    let output_file = format_output_file_name(output_file)?;

    let data = read_input_file(input_file)?;
    let new_text = process_data(&data, &text_to_be_replaced)?;

    write_output_file(&output_file, &new_text)?;

    Ok(())
}

fn format_output_file_name(output_file: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output_file = output_file.replace(" ", "_").to_lowercase();
    Ok(Path::new(&output_file).to_path_buf())
}

fn read_input_file(input_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut data = String::new();
    buf_reader.read_to_string(&mut data)?;
    Ok(data)
}

fn is_symbol(c: char) -> bool {
    let symbols = "/\\:*?\"<>|!@#$%^&()-+=[]{};,.\'~`";
    symbols.contains(c)
}

fn delete_entire_line(data: &str, text_to_be_deleted: &str) -> String {
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

fn process_data(
    data: &str,
    text_to_be_replaced: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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

fn write_output_file(output_file: &Path, new_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output_file)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(new_text.as_bytes())?;
    Ok(())
}
