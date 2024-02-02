use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x >= 0 && x < 10  { 
            return true;
        };

        if x.is_negative() {
            return false;
        }

        let mut reversed = 0_i64;
        let mut quotient = x;

        while quotient > 0 {
            let remainder = quotient % 10;
            quotient = quotient / 10;
            reversed = (reversed + remainder as i64) * 10;
        }
        (reversed / 10) as i32 == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let x = 121;
        let result = Solution::is_palindrome(x);
        assert_eq!(result, true);

        let x = -121;
        let result = Solution::is_palindrome(x);
        assert_eq!(result, false);

        let x = 1;
        let result = Solution::is_palindrome(x);
        assert_eq!(result, true);

        let x = 10;
        let result = Solution::is_palindrome(x);
        assert_eq!(result, false);

        let x = 1000000001;
        let result = Solution::is_palindrome(x);
        assert_eq!(result, true);
    }
}