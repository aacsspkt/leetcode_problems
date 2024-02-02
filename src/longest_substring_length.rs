use std::{cmp::max, collections::HashMap};
use crate::Solution;

// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.

// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.

// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {

        let mut m = HashMap::new();
        let mut ans = 0;
        let mut before = -1;
        let mut current = 0;
        for c in s.chars() {
            if let Some(last) = m.insert(c, current) {
                before = max(before, last);
            }
            ans = max(ans, current - before);
            current += 1;
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let s = "dvdf".to_owned();
        let result = Solution::length_of_longest_substring(s);
        assert_eq!(result, 3);

        let s = "abcabcbb".to_owned();
        let result = Solution::length_of_longest_substring(s);
        assert_eq!(result, 3);

        let s = "bbbbb".to_owned();
        let result = Solution::length_of_longest_substring(s);
        assert_eq!(result, 1);

        let s = "pwwkew".to_owned();
        let result = Solution::length_of_longest_substring(s);
        assert_eq!(result, 3);
    }
}