use std::cmp::max;
use std::collections::HashMap;

// Previously there was a solution that used slightly less memory than this one at the cost
// of being slightly slower, but once this solution was modified to iterate straight through
// the string rather than copying it into a character array, its speed and memory usage
// both improved, to the point where it surpassed the other solution by both metrics.
//
// Long story short, the massive improvement came because the other solution needed to access
// characters by arbitrary indices, while this solution just iterates straight through. Because
// Rust strings don't allow you to index them by character directly, that solution required creating
// a character array from the string, which involved making a new heap allocation and copying every
// character into it. As you can imagine, that was both slow and used a lot more memory.
//
// Although this is a Rust-specific implementation detail rather than an algorithmic improvement,
// it did still result in large gains in the algorithm's performance: the runtime dropped from
// 4 ms (~75th percentile according to Leetcode) to 0 ms (100th percentile according to Leetcode),
// and managed to match the memory usage of the other solution at 2.2 MB. To me, this underscores
// another important consideration when writing code: oftentimes it's just as lucrative to know
// and exploit implementation-specific details as it is to understand good algorithm design & analysis
// when trying to eke out every last bit of performance from a program.
pub fn length_of_longest_substring(s: String) -> i32 {
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
