//! Utilities related to text

use std::collections::HashSet;

/// Checks a string for duplicate characters. Note that this is case-sensitive
/// so the string `"aA"` will not have the two `A`s counted as duplicates.
///
/// # Example
///
/// ```rust
/// # extern crate co2;
/// use co2::util::text;
///
/// assert!(text::check_duplicate_chars("abcdebf"));
/// assert!(!text::check_duplicate_chars("abcdef"));
/// // This function is case-sensitive, so `A` is not considered as a duplicate of `a`.
/// assert!(!text::check_duplicate_chars("abcABC"));
/// ```
///
pub fn check_duplicate_chars(input: &str) -> bool {
    let mut set = HashSet::new();
    !input.chars().all(|ch| {
        if set.contains(&ch) {
            false
        } else {
            set.insert(ch);
            true
        }
    })
}
