#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    input.to_string().chars().rev().collect()
}

#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    input.to_string().graphemes(true).rev().collect()
}
