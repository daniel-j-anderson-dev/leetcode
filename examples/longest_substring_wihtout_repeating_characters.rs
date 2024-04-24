/// Given a string s, find the length of the **longest substring**
/// without repeating characters.
///
/// Example 1:
/// ```
/// Input: s = "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3.
/// ```
///
/// Example 2:
/// ```
/// Input: s = "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.
/// ```
/// Example 3:
///
/// ```
/// Input: s = "pwwkew"
/// Output: 3
/// Explanation: The answer is "wke", with the length of 3.
/// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
/// ```
///  
///
/// Constraints:
/// - 0 <= s.length <= 5 * 104
/// - s consists of English letters, digits, symbols and spaces.
/// ```
fn main() {
    let s = "⬛⬜⬜🟥🟧🟨🟩🟩🟦🟪🟫";
    dbg!(length_of_longest_substring_without_repeats(s));
}

fn no_repeated_chars(s: &str) -> bool {
    use std::collections::HashSet;

    let mut characters_seen = HashSet::<char>::new();

    for character in s.chars() {
        if !characters_seen.insert(character) {
            return false;
        }
    }

    true
}

fn length_of_longest_substring_without_repeats(s: &str) -> usize {
    let mut max_length = 0;

    for end in s.char_indices().map(|(i, _)| i) {
        for no_repeats in s.char_indices().filter_map(move |(start, _)| {
            s.get(start..end).and_then(|sub_str| {
                if sub_str.len() > max_length && no_repeated_chars(sub_str) {
                    Some(sub_str)
                } else {
                    None
                }
            })
        }) {
            max_length = no_repeats.len();
        }
    }

    return max_length;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        const S: &str = "abcabcbb";
        let length = length_of_longest_substring_without_repeats(S);
        assert_eq!(length, 3)
    }
    #[test]
    fn case2() {
        const S: &str = "bbbbb";
        let length = length_of_longest_substring_without_repeats(S);
        assert_eq!(length, 1)
    }
    #[test]
    fn case3() {
        const S: &str = "pwwkew";
        let length = length_of_longest_substring_without_repeats(S);
        assert_eq!(length, 3)
    }
}
