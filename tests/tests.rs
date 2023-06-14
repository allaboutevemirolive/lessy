use std::path::PathBuf;
use std::io::{Read, Write};
use lessy::data_processing::{is_symbol, delete_entire_line, process_data};
use lessy::file_utils::{format_output_file_name, read_input_file, write_output_file};

fn setup_test_file(file_path: &PathBuf, content: &str) {
    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn remove_test_file(file_path: &PathBuf) {
    std::fs::remove_file(file_path).unwrap();
}

fn read_test_file(file_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[test]
fn test_data_processing() {
    assert!(is_symbol('/'));
    assert!(!is_symbol('A'));

    let data = "Line 1\nLine 2\nLine 3";
    let text_to_be_deleted = "Line 2";
    let expected_result = "Line 1\nLine 3";
    let result = delete_entire_line(data, text_to_be_deleted);
    assert_eq!(result, expected_result);

    let data = "Line 1\nLine 2\nLine 3";
    let text_to_be_replaced = "Line 2";
    let expected_result = "Line 1\nModified Line 2\nLine 3";
    let result = process_data(data, text_to_be_replaced).unwrap();
    assert_eq!(result, expected_result);
}

#[test]
fn test_file_utils() {
    let input_file = "input.txt";
    let output_file = "output.txt";
    let test_data = "Test data";

    // setup_test_file(&PathBuf::from(input_file), test_data);

    let result = read_input_file(input_file).unwrap();
    assert_eq!(result, test_data);

    let output_file_path = format_output_file_name(output_file).unwrap();
    assert_eq!(output_file_path.to_str(), Some("output.txt"));

    write_output_file(&output_file_path, test_data).unwrap();

    let result = read_test_file(&output_file_path).unwrap();
    assert_eq!(result, test_data);

    // remove_test_file(&PathBuf::from(input_file));
    // remove_test_file(&output_file_path);
}
