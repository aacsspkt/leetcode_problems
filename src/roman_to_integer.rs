use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let numbers = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut sum = 0;
        let mut highest_value = 0;
        // MCMXCIV

        for ch in s.chars().rev() {
            let &value = numbers.get(&ch).unwrap();

            match value.cmp(&highest_value) {
                std::cmp::Ordering::Greater => {
                    sum += value;
                    highest_value = value;
                }
                std::cmp::Ordering::Equal => sum += value,
                std::cmp::Ordering::Less => sum -= value,
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let s = "III".to_string();
        let i = Solution::roman_to_int(s);
        assert_eq!(i, 3);

        let s = "LVIII".to_string();
        let i = Solution::roman_to_int(s);
        assert_eq!(i, 58);

        let s = "MCMXCIV".to_string();
        let i = Solution::roman_to_int(s);
        assert_eq!(i, 1994);

        let s = "MMMCMXCIX".to_string();
        let i = Solution::roman_to_int(s);
        assert_eq!(i, 3999);
    }
}
