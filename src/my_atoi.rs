use crate::Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();

        if s.len() == 0 {
            return 0;
        }

        let chars: Vec<char> = s.chars().collect();

        let mut i = 0;
        let mut ans = 0_i32;

        let is_neg = if chars[i] == '-' {
            i += 1;
            true
        } else {
            if chars[i] == '+' {
                i += 1;
            }
            false
        };

        while i < chars.len() {
            let ch = chars[i];

            if ch >= '0' && ch <= '9' {
                if let Some(prod) = ans.checked_mul(10) {
                    if let Some(sum) = prod.checked_add(chars[i].to_digit(10).unwrap() as i32) {
                        ans = sum;
                    } else {
                        if is_neg {
                            return i32::MIN;
                        }
                        return i32::MAX;
                    }
                } else {
                    if is_neg {
                        return i32::MIN;
                    }
                    return i32::MAX;
                }
            } else {
                break;
            }
            i += 1;
        }

        if is_neg {
            if let Some(neg) = ans.checked_neg() {
                return neg;
            }
            return i32::MIN;
        }

        ans
    }
}
