pub fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();

    // look at this beautiful iterator magic
    s.chars()
        .zip(s.chars().rev())
        .all(|(c1, c2)| c1 == c2)
}
