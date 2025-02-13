use lazy_static::lazy_static;
use std::fmt::Write;

/// Returns the index of the next character boundary in the given string.
///
/// If the given index is already a character boundary, it is returned as is.
/// If the given index is out of bounds, the length of the string is returned.
///
/// # Examples
/// ```
/// use television_utils::strings::next_char_boundary;
///
/// let s = "Hello, World!";
/// assert_eq!(next_char_boundary(s, 0), 0);
/// assert_eq!(next_char_boundary(s, 1), 1);
/// assert_eq!(next_char_boundary(s, 13), 13);
/// assert_eq!(next_char_boundary(s, 30), 13);
///
/// let s = "👋🌍!";
/// assert_eq!(next_char_boundary(s, 0), 0);
/// assert_eq!(next_char_boundary(s, 1), 4);
/// assert_eq!(next_char_boundary(s, 4), 4);
/// assert_eq!(next_char_boundary(s, 7), 8);
/// assert_eq!(next_char_boundary(s, 8), 8);
/// ```
pub fn next_char_boundary(s: &str, start: usize) -> usize {
    let mut i = start;
    let len = s.len();
    if i >= len {
        return len;
    }
    while !s.is_char_boundary(i) && i < len {
        i += 1;
    }
    i
}

/// Returns the index of the previous character boundary in the given string.
///
/// If the given index is already a character boundary, it is returned as is.
/// If the given index is out of bounds, 0 is returned.
///
/// # Examples
/// ```
/// use television_utils::strings::prev_char_boundary;
///
/// let s = "Hello, World!";
/// assert_eq!(prev_char_boundary(s, 0), 0);
/// assert_eq!(prev_char_boundary(s, 1), 1);
/// assert_eq!(prev_char_boundary(s, 5), 5);
///
/// let s = "👋🌍!";
/// assert_eq!(prev_char_boundary(s, 0), 0);
/// assert_eq!(prev_char_boundary(s, 4), 4);
/// assert_eq!(prev_char_boundary(s, 6), 4);
/// ```
pub fn prev_char_boundary(s: &str, start: usize) -> usize {
    let mut i = start;
    while !s.is_char_boundary(i) && i > 0 {
        i -= 1;
    }
    i
}

/// Returns a slice of the given string that starts and ends at character boundaries.
///
/// If the given start index is greater than the end index, or if either index is out of bounds,
/// an empty string is returned.
///
/// # Examples
/// ```
/// use television_utils::strings::slice_at_char_boundaries;
///
/// let s = "Hello, World!";
/// assert_eq!(slice_at_char_boundaries(s, 0, 0), "");
/// assert_eq!(slice_at_char_boundaries(s, 0, 1), "H");
///
/// let s = "👋🌍!";
/// assert_eq!(slice_at_char_boundaries(s, 0, 0), "");
/// assert_eq!(slice_at_char_boundaries(s, 0, 2), "👋");
/// assert_eq!(slice_at_char_boundaries(s, 0, 5), "👋🌍");
/// ```
pub fn slice_at_char_boundaries(
    s: &str,
    start_byte_index: usize,
    end_byte_index: usize,
) -> &str {
    if start_byte_index > end_byte_index
        || start_byte_index > s.len()
        || end_byte_index > s.len()
    {
        return EMPTY_STRING;
    }
    &s[prev_char_boundary(s, start_byte_index)
        ..next_char_boundary(s, end_byte_index)]
}

/// Returns a slice of the given string that starts at the beginning and ends at a character
/// boundary.
///
/// If the given index is out of bounds, the whole string is returned.
/// If the given index is already a character boundary, the string up to that index is returned.
///
/// # Examples
/// ```
/// use television_utils::strings::slice_up_to_char_boundary;
///
/// let s = "Hello, World!";
/// assert_eq!(slice_up_to_char_boundary(s, 0), "");
/// assert_eq!(slice_up_to_char_boundary(s, 1), "H");
/// assert_eq!(slice_up_to_char_boundary(s, 13), "Hello, World!");
///
/// let s = "👋\n🌍!";
/// assert_eq!(slice_up_to_char_boundary(s, 0), "");
/// assert_eq!(slice_up_to_char_boundary(s, 1), "👋");
/// assert_eq!(slice_up_to_char_boundary(s, 4), "👋");
/// assert_eq!(slice_up_to_char_boundary(s, 7), "👋\n🌍");
/// ```
pub fn slice_up_to_char_boundary(s: &str, byte_index: usize) -> &str {
    &s[..next_char_boundary(s, byte_index)]
}

/// Attempts to parse a UTF-8 character from the given byte slice.
fn try_parse_utf8_char(input: &[u8]) -> Option<(char, usize)> {
    let str_from_utf8 = |seq| std::str::from_utf8(seq).ok();

    let decoded = input
        .get(0..1)
        .and_then(str_from_utf8)
        .map(|c| (c, 1))
        .or_else(|| input.get(0..2).and_then(str_from_utf8).map(|c| (c, 2)))
        .or_else(|| input.get(0..3).and_then(str_from_utf8).map(|c| (c, 3)))
        .or_else(|| input.get(0..4).and_then(str_from_utf8).map(|c| (c, 4)));

    decoded.map(|(seq, n)| (seq.chars().next().unwrap(), n))
}

lazy_static! {
    /// The Unicode symbol to use for non-printable characters.
    static ref NULL_SYMBOL: char = char::from_u32(0x2400).unwrap();
}

pub const EMPTY_STRING: &str = "";
pub const TAB_WIDTH: usize = 4;

const SPACE_CHARACTER: char = ' ';
const TAB_CHARACTER: char = '\t';
const LINE_FEED_CHARACTER: char = '\x0A';
const DELETE_CHARACTER: char = '\x7F';
const BOM_CHARACTER: char = '\u{FEFF}';
const NULL_CHARACTER: char = '\x00';
const UNIT_SEPARATOR_CHARACTER: char = '\u{001F}';
const APPLICATION_PROGRAM_COMMAND_CHARACTER: char = '\u{009F}';

/// Replaces non-printable characters in the given byte slice with default printable characters.
///
/// The tab width is used to determine how many spaces to replace a tab character with.
/// The default printable character for non-printable characters is the Unicode symbol for NULL.
///
/// # Examples
/// ```
/// use television_utils::strings::replace_non_printable;
///
/// let input = b"Hello, World!";
/// let output = replace_non_printable(input, 2);
/// assert_eq!(output, "Hello, World!");
///
/// let input = b"Hello\tWorld!";
/// let output = replace_non_printable(input, 2);
/// assert_eq!(output, "Hello  World!");
///
/// let input = b"Hello\nWorld!";
/// let output = replace_non_printable(input, 2);
/// assert_eq!(output, "HelloWorld!");
///
/// let input = b"Hello\x00World!";
/// let output = replace_non_printable(input, 2);
/// assert_eq!(output, "Hello␀World!");
///
/// let input = b"Hello\x7FWorld!";
/// let output = replace_non_printable(input, 2);
/// assert_eq!(output, "Hello␀World!");
/// ```
pub fn replace_non_printable(input: &[u8], tab_width: usize) -> String {
    let mut output = String::new();

    let mut idx = 0;
    let len = input.len();
    while idx < len {
        if let Some((chr, skip_ahead)) = try_parse_utf8_char(&input[idx..]) {
            idx += skip_ahead;

            match chr {
                // space
                SPACE_CHARACTER => output.push(' '),
                // tab
                TAB_CHARACTER => {
                    output.push_str(&" ".repeat(tab_width));
                }
                // line feed
                LINE_FEED_CHARACTER => {}

                // ASCII control characters from 0x00 to 0x1F
                // + control characters from \u{007F} to \u{009F}
                NULL_CHARACTER..=UNIT_SEPARATOR_CHARACTER
                | DELETE_CHARACTER..=APPLICATION_PROGRAM_COMMAND_CHARACTER => {
                    output.push(*NULL_SYMBOL);
                }
                // don't print BOMs
                BOM_CHARACTER => {}
                // Unicode characters above 0x0700 seem unstable with ratatui
                c if c > '\u{0700}' => {
                    output.push(*NULL_SYMBOL);
                }
                // everything else
                c => output.push(c),
            }
        } else {
            write!(output, "\\x{:02X}", input[idx]).ok();
            idx += 1;
        }
    }

    output
}

/// The threshold for considering a buffer to be printable ASCII.
///
/// This is used to determine whether a file is likely to be a text file
/// based on a sample of its contents.
pub const PRINTABLE_ASCII_THRESHOLD: f32 = 0.7;

/// Returns the proportion of printable ASCII characters in the given buffer.
///
/// This really is a cheap way to determine if a buffer is likely to be a text file.
///
/// # Examples
/// ```
/// use television_utils::strings::proportion_of_printable_ascii_characters;
///
/// let buffer = b"Hello, World!";
/// let proportion = proportion_of_printable_ascii_characters(buffer);
/// assert_eq!(proportion, 1.0);
///
/// let buffer = b"Hello, World!\x00";
/// let proportion = proportion_of_printable_ascii_characters(buffer);
/// assert_eq!(proportion, 0.9285714);
///
/// let buffer = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
/// let proportion = proportion_of_printable_ascii_characters(buffer);
/// assert_eq!(proportion, 0.0);
/// ```
pub fn proportion_of_printable_ascii_characters(buffer: &[u8]) -> f32 {
    let mut printable: usize = 0;
    for &byte in buffer {
        if (32..127).contains(&byte) {
            printable += 1;
        }
    }
    printable as f32 / buffer.len() as f32
}

const MAX_LINE_LENGTH: usize = 300;

/// Preprocesses a line of text for display.
///
/// This function trims the line, replaces non-printable characters, and truncates the line if it
/// is too long.
///
/// # Examples
/// ```
/// use television_utils::strings::preprocess_line;
///
/// let line = "Hello, World!";
/// let processed = preprocess_line(line);
/// assert_eq!(processed, "Hello, World!");
///
/// let line = "\x00World\x7F!";
/// let processed = preprocess_line(line);
/// assert_eq!(processed, "␀World␀!");
///
/// let line = "a".repeat(400);
/// let processed = preprocess_line(&line);
/// assert_eq!(processed.len(), 300);
/// ```
pub fn preprocess_line(line: &str) -> String {
    replace_non_printable(
        {
            if line.len() > MAX_LINE_LENGTH {
                slice_up_to_char_boundary(line, MAX_LINE_LENGTH)
            } else {
                line
            }
        }
        .trim_end_matches(['\r', '\n', '\0'])
        .as_bytes(),
        TAB_WIDTH,
    )
}

/// Shrink a string to a maximum length, adding an ellipsis in the middle.
///
/// If the string is shorter than the maximum length, it is returned as is.
/// If the string is longer than the maximum length, it is shortened and an ellipsis is added in
/// the middle.
///
/// # Examples
/// ```
/// use television_utils::strings::shrink_with_ellipsis;
///
/// let s = "Hello, World!";
/// assert_eq!(shrink_with_ellipsis(s, 13), "Hello, World!");
/// assert_eq!(shrink_with_ellipsis(s, 6), "H…!");
/// ```
pub fn shrink_with_ellipsis(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        return s.to_string();
    }

    let half_max_length = (max_length / 2).saturating_sub(2);
    let first_half = slice_up_to_char_boundary(s, half_max_length);
    let second_half =
        slice_at_char_boundaries(s, s.len() - half_max_length, s.len());
    format!("{first_half}…{second_half}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_next_char_boundary(input: &str, start: usize, expected: usize) {
        let actual = next_char_boundary(input, start);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next_char_boundary_ascii() {
        test_next_char_boundary("Hello, World!", 0, 0);
        test_next_char_boundary("Hello, World!", 1, 1);
        test_next_char_boundary("Hello, World!", 13, 13);
        test_next_char_boundary("Hello, World!", 30, 13);
    }

    #[test]
    fn test_next_char_boundary_emoji() {
        test_next_char_boundary("👋🌍!", 0, 0);
        test_next_char_boundary("👋🌍!", 1, 4);
        test_next_char_boundary("👋🌍!", 4, 4);
        test_next_char_boundary("👋🌍!", 8, 8);
        test_next_char_boundary("👋🌍!", 7, 8);
    }

    fn test_previous_char_boundary(
        input: &str,
        start: usize,
        expected: usize,
    ) {
        let actual = prev_char_boundary(input, start);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_previous_char_boundary_ascii() {
        test_previous_char_boundary("Hello, World!", 0, 0);
        test_previous_char_boundary("Hello, World!", 1, 1);
        test_previous_char_boundary("Hello, World!", 5, 5);
    }

    #[test]
    fn test_previous_char_boundary_emoji() {
        test_previous_char_boundary("👋🌍!", 0, 0);
        test_previous_char_boundary("👋🌍!", 4, 4);
        test_previous_char_boundary("👋🌍!", 6, 4);
        test_previous_char_boundary("👋🌍!", 8, 8);
    }

    fn test_slice_at_char_boundaries(
        input: &str,
        start: usize,
        end: usize,
        expected: &str,
    ) {
        let actual = slice_at_char_boundaries(input, start, end);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_slice_at_char_boundaries_ascii() {
        test_slice_at_char_boundaries("Hello, World!", 0, 0, "");
        test_slice_at_char_boundaries("Hello, World!", 0, 1, "H");
        test_slice_at_char_boundaries("Hello, World!", 0, 13, "Hello, World!");
        test_slice_at_char_boundaries("Hello, World!", 0, 30, "");
    }

    #[test]
    fn test_slice_at_char_boundaries_emoji() {
        test_slice_at_char_boundaries("👋🌍!", 0, 0, "");
        test_slice_at_char_boundaries("👋🌍!", 0, 4, "👋");
        test_slice_at_char_boundaries("👋🌍!", 0, 8, "👋🌍");
        test_slice_at_char_boundaries("👋🌍!", 0, 7, "👋🌍");
        test_slice_at_char_boundaries("👋🌍!", 0, 9, "👋🌍!");
    }

    fn test_replace_non_printable(input: &str, expected: &str) {
        let actual = replace_non_printable(input.as_bytes(), 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_replace_non_printable_ascii() {
        test_replace_non_printable("Hello, World!", "Hello, World!");
    }

    #[test]
    fn test_replace_non_printable_tab() {
        test_replace_non_printable("Hello\tWorld!", "Hello  World!");
        test_replace_non_printable(
            "	-- AND
", "  -- AND",
        )
    }

    #[test]
    fn test_replace_non_printable_line_feed() {
        test_replace_non_printable("Hello\nWorld!", "HelloWorld!");
    }

    #[test]
    fn test_replace_non_printable_null() {
        test_replace_non_printable("Hello\x00World!", "Hello␀World!");
        test_replace_non_printable("Hello World!\0", "Hello World!␀");
    }

    #[test]
    fn test_replace_non_printable_delete() {
        test_replace_non_printable("Hello\x7FWorld!", "Hello␀World!");
    }

    #[test]
    fn test_replace_non_printable_bom() {
        test_replace_non_printable("Hello\u{FEFF}World!", "HelloWorld!");
    }

    #[test]
    fn test_replace_non_printable_start_txt() {
        test_replace_non_printable("Àì", "Àì␀");
    }

    fn test_proportion_of_printable_ascii_characters(
        input: &str,
        expected: f32,
    ) {
        let actual =
            proportion_of_printable_ascii_characters(input.as_bytes());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_proportion_of_printable_ascii_characters_ascii() {
        test_proportion_of_printable_ascii_characters("Hello, World!", 1.0);
        test_proportion_of_printable_ascii_characters(
            "Hello, World!\x00",
            0.9285714,
        );
        test_proportion_of_printable_ascii_characters(
            "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F",
            0.0,
        );
    }

    fn test_preprocess_line(input: &str, expected: &str) {
        let actual = preprocess_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_preprocess_line_cases() {
        test_preprocess_line("Hello, World!", "Hello, World!");
        test_preprocess_line("Hello, World!\n", "Hello, World!");
        test_preprocess_line("Hello, World!\x00", "Hello, World!");
        test_preprocess_line("Hello, World!\x7F", "Hello, World!␀");
        test_preprocess_line("Hello, World!\u{FEFF}", "Hello, World!");
        test_preprocess_line(&"a".repeat(400), &"a".repeat(300));
    }
}
