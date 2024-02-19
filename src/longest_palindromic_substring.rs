use crate::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let mut dp = vec![vec![false; len]; len];

        let mut ans: [usize; 2] = [0, 0];

        for i in 0..len {
            dp[i][i] = true;
        }

        let mut chars = Vec::new();

        for ch in s.chars() {
            chars.push(ch);
        }

        for i in 0..len - 1 {
            if chars[i].eq(&chars[i + 1]) {
                dp[i][i + 1] = true;
                ans[0] = i;
                ans[1] = i + 1;
            }
        }

        for diff in 2..len {
            for i in 0..len - diff {
                let j = i + diff;
                if chars[i].eq(&chars[j]) && dp[i + 1][j - 1] {
                    dp[i][j] = true;
                    ans[0] = i;
                    ans[1] = j;
                }
            }
        }

        s[ans[0]..(ans[1] + 1)].to_string()
    }
}
