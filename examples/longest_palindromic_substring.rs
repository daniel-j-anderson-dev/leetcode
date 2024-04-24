/// Given a string s, return the longest
/// palindromic
/// substring
/// in s.
///
/// Example 1:
/// ```
/// Input: s = "babad"
/// Output: "bab"
/// Explanation: "aba" is also a valid answer.
/// ```
///
/// <br>Example 2:
/// ```
/// Input: s = "cbbd"
/// Output: "bb"
/// ```
///  
///
/// <br>Constraints:
/// - `1 <= s.length <= 1000`
/// - `s consist of only digits and English letters.`
pub fn main() {}

fn all_sub_sequences(s: &str) -> impl Iterator<Item = &str> {
    s.char_indices()
        .map(move |(end, _)| {
            s.char_indices()
                .filter_map(move |(start, _)| s.get(start..=end))
        })
        .flatten()
}

fn is_palindrome(s: &str) -> bool {
    s.chars().rev().zip(s.chars()).all(|(rev_c, c)| rev_c == c)
}

pub fn longest_palindrome(s: &str) -> &str {
    all_sub_sequences(s)
        .filter(|sub_str| is_palindrome(sub_str))
        .max_by_key(|sub_str| sub_str.len())
        .unwrap_or_default()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let s = "babad";
        let expected_longest1 = "bab";
        let expected_longest2 = "aba";
        let longest = longest_palindrome(s);
        assert!(longest == expected_longest1 || longest == expected_longest2);
    }
    #[test]
    fn case2() {
        let s = "cbbd";
        let expected_longest = "bb";
        let longest = longest_palindrome(s);
        assert_eq!(longest, expected_longest);
    }
}
