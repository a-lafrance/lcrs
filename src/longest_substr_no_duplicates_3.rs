use std::cmp::max;
use std::collections::{HashMap, HashSet};

// A solution that's very fast, but uses a bit more memory than the below. It does so by storing
// the last occurrence of each character in the substring, which removes some manual iteration at the cost
// of storing that extra info.
//
// Also, this solution allows you to touch each character exactly once, meaning
// you can iterate straight through the string without having to access any character through indexing.
// This allows you to avoid creating a vector of chars from the original string (see below). Although this
// is an implementation detail rather than an algorithmic improvement, it still vastly improves the performance
// of the algorithm; my Leetcode submission went down from 4 ms before the change (~75th percentile)
// to 0 ms after (100th percentile).
pub fn length_of_longest_substring_fast(s: String) -> i32 {
    let mut longest_substr_len = 0i32;
    let mut lower = 0;
    let mut upper = 0;
    let mut current_substr_chars = HashMap::new(); // char -> last occurrence

    for c in s.chars() {
        if let Some(last_i) = current_substr_chars.insert(c, upper) {
            if last_i >= lower {
                let current_substr_len = (upper - lower) as i32;
                longest_substr_len = max(
                    current_substr_len, longest_substr_len
                );

                lower = last_i + 1;
            }
        }

        upper += 1;
    }

    if upper >= s.len() {
        let current_substr_len = (upper - lower) as i32;
        max(current_substr_len, longest_substr_len)
    } else {
        longest_substr_len
    }
}

// A solution that uses slightly less memory. Namely, it doesn't store the map with last occurrence,
// and instead manually "catches up" when a substring is terminated.
pub fn length_of_longest_substring_small(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut longest_substr_len = 0i32;
    let mut lower = 0;
    let mut upper = 0;
    let mut current_substr_chars = HashSet::new();

    while lower < chars.len() && upper < chars.len() {
        if current_substr_chars.contains(&chars[upper]) {
            let current_substr_len = (upper - lower) as i32;
            longest_substr_len = max(current_substr_len, longest_substr_len);

            while chars[lower] != chars[upper] {
                current_substr_chars.remove(&chars[lower]);
                lower += 1;
            }

            lower += 1;
        }

        current_substr_chars.insert(chars[upper]);
        upper += 1;
    }

    if upper >= chars.len() {
        let current_substr_len = (upper - lower) as i32;
        max(current_substr_len, longest_substr_len)
    } else {
        longest_substr_len
    }
}
