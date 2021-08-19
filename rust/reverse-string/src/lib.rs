#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    let mut res = String::with_capacity(input.len());
    for c in input.to_string().chars().rev() {
        res.push(c);
    }
    res
}

#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    let mut res = String::with_capacity(input.len());
    for c in input.to_string().graphemes(true).rev() {
        res.push_str(c);
    }
    res
}
