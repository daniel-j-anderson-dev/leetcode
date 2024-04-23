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
fn main() {}


fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = &s[0..0];
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    const S: &str = "abcabcbb";
    #[test]
    fn case1() {
    }
    #[test]
    fn case2() {
    }
    #[test]
    fn case3() {
    }
}

