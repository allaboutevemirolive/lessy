#[cfg(test)]
mod tests {

    use crate::data_processing::{delete_entire_line, delete_specific_words};

    #[test]
    fn test_delete_entire_line_single_line() {
        let data = "Hello, world!";
        let text_to_be_deleted = "world";
        let result = delete_entire_line(data, text_to_be_deleted);
        assert_eq!(result, "");
    }

    #[test]
    fn test_delete_entire_line_multiple_lines() {
        // TODO
        let data = "Line 1\nLine 2\nLine 3";
        let text_to_be_deleted = "Line 2";
        let result = delete_entire_line(data, text_to_be_deleted);
        assert_eq!(result, "Line 1\nLine 3\n");
    }

    #[test]
    fn test_delete_entire_line_multiple_lines_2() {
        let data = "Hello, **Rust**! How are you doing **today**?";
        let text_to_be_deleted = "**";
        let result = delete_entire_line(data, text_to_be_deleted);
        assert_eq!(result, "");
    }

    #[test]
    fn test_delete_entire_line_text_not_found() {
        let data = "This is a test.";
        let text_to_be_deleted = "banana";
        let result = delete_entire_line(data, text_to_be_deleted);
        assert_eq!(result, "This is a test.\n");
    }

    #[test]
    fn test_delete_entire_line_empty_data() {
        let data = "";
        let text_to_be_deleted = "test";
        let result = delete_entire_line(data, text_to_be_deleted);
        assert_eq!(result, "");
    }

    #[test]
    fn test_delete_entire_line_empty_text_to_be_deleted() {
        let data = "Line 1\nLine 2";
        let text_to_be_deleted = "";
        let result = delete_entire_line(data, text_to_be_deleted);
        assert_eq!(result, "Line 1\nLine 2");
    }

    #[test]
    fn test_delete_entire_line_multiline_text_to_be_deleted() {
        // This test will fail because in the code we process each line
        let data = "Line 1\nLine 2\nLine 3";
        let text_to_be_deleted = "Line 1\nLine 2";
        let result = delete_entire_line(data, text_to_be_deleted);
        assert_eq!(result, "Line 1\nLine 2\nLine 3\n");
    }

    #[test]
    fn test_delete_specific_words_single_line() {
        let data = "The quick brown fox jumps over the lazy dog.";
        let words_to_be_deleted = ["quick", "over", "lazy"];
        let result = delete_specific_words(data, &words_to_be_deleted);
        assert_eq!(result, "The  brown fox jumps  the  dog.");
    }

    // TODO
    #[test]
    fn test_delete_specific_words_multi_line() {
        let data = "Line 1: Hello, world!\nLine 2: Goodbye, world!";
        let words_to_be_deleted = ["Hello", "Goodbye"];
        let result = delete_specific_words(data, &words_to_be_deleted);
        assert_eq!(result, "Line 1: , world!Line 2: , world!");
    }

    #[test]
    fn test_delete_specific_words_part_of_larger_words() {
        let data = "The category contains a cat and a caterpillar.";
        let words_to_be_deleted = ["cat"];
        let result = delete_specific_words(data, &words_to_be_deleted);
        assert_eq!(result, "The category contains a  and a caterpillar.");
    }

    #[test]
    fn test_delete_specific_words_empty_data() {
        let data = "";
        let words_to_be_deleted = ["word", "another"];
        let result = delete_specific_words(data, &words_to_be_deleted);
        assert_eq!(result, "");
    }

    #[test]
    fn test_delete_specific_words_empty_words_to_be_deleted() {
        let data = "This is a test.";
        let words_to_be_deleted = [];
        let result = delete_specific_words(data, &words_to_be_deleted);
        assert_eq!(result, "This is a test.");
    }

    // TODO
    #[test]
    fn test_delete_specific_words_with_special_characters() {
        let data = "Escape special characters: ^$*.[\\";
        let words_to_be_deleted = ["^", "$", "*", "[", "\\"];
        let result = delete_specific_words(data, &words_to_be_deleted);
        assert_eq!(result, "Escape special characters: .\n");
    }
}
