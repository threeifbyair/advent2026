//! Helpers for parsing recurring Advent of Code input shapes.

use std::fmt::Debug;
use std::str::FromStr;

/// Splits a command like `"L42"` into its leading letter and trailing number,
/// e.g. `('L', 42)`.
pub fn letter_and_number<T: FromStr>(s: &str) -> (char, T)
where
    T::Err: Debug,
{
    let mut chars = s.chars();
    let letter = chars.next().expect("command should not be empty");
    let number = chars.as_str().parse().expect("trailing number should parse");
    (letter, number)
}
