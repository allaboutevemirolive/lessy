
use lessy::data_processing::{delete_entire_line, delete_specific_words, is_symbol, process_data};
use lessy::file_utils::{format_output_file_name, read_input_file, write_output_file};
use regex::Regex;

// Test case

// 1. The sky is blue.
// 2. Dogs bark loudly.
// 3. Time flies quickly.
// 4. Rain falls softly.
// 5. Birds sing sweetly.

#[test]
fn test_my_test_case() {
    let input_1 = "\
1. The sky is blue.
2. Dogs bark loudly.
3. Time flies quickly.
4. Rain falls softly.
5. Birds sing sweetly.";

    let input_2 = "\
1. The sky is blue.
2. Dogs bark loudly.
3. Time flies quickly.
4. Rain falls softly.
5. Birds sing sweetly.";

    assert_eq!(input_1, input_2);
}

#[test]
fn test_delete_entire_line() {
    let input_1 = "\
1. The sky is blue.
2. Dogs bark loudly.
3. Time flies quickly.
4. Rain falls softly.
5. Birds sing sweetly.";

    let expected_result = "\
1. The sky is blue.
2. Dogs bark loudly.
4. Rain falls softly.
5. Birds sing sweetly.
";

    let target_text = "flies";

    let result = delete_entire_line(input_1, target_text);

    assert_eq!(result, expected_result);
}

#[test]
fn test_delete_specific_words() {
    let input_1 = "\
1. The sky[1] is blue.
2. Dogs bark [45]loudly.
3. [67]Time flies quickly.
4. Rain falls softly.
5. Birds[23] sing sweetly[132].";

    let words_to_delete = &[
        r#"\[[0-9]+\]"#, // r"The availability of a high-level"
                         // r"separator"
    ];

    let expected_result = "\
1. The sky is blue.
2. Dogs bark loudly.
3. Time flies quickly.
4. Rain falls softly.
5. Birds sing sweetly.";

    let result = delete_specific_words(input_1, words_to_delete);

    println!("Result: {}", result);

    assert_eq!(result, expected_result)
}

#[test]
fn test_delete_specific_snippet() {

    let input_1 = "\
{
    println[21]{
        test.this
    }
}";

    let words_to_delete = &[
        r#"\[[0-9]+\]"#
        // r"The availability of a high-level"
        // r"separator"
    ]; 

    let expected_output = "\
{
    println{
        test.this
    }
}";

    let result = delete_specific_words(input_1, words_to_delete);

    println!("Result:{}", result);

    assert_eq!(result, expected_output);

}
