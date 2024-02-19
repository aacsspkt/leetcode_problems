use crate::Solution;

impl Solution {
    pub fn convert_to_zigzag_string(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows as usize >= s.len() {
            return s;
        }

        let mut v = vec![vec![]; num_rows as usize];

        let mut index = 0_i32;
        let mut step = 1;
        for ch in s.chars() {
            v[index as usize].push(ch);
            if index == 0 {
                step = 1;
            } else if index == num_rows - 1 {
                step = -1;
            }
            index += step;
        }

        let ans: String = v
            .iter()
            .map(|chars| {
                let str: String = chars.iter().collect();
                str
            })
            .collect();
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let s = "PAYPALISHIRING".to_owned();
        let result = Solution::convert_to_zigzag_string(s, 3);
        assert_eq!(result, "PAHNAPLSIIGYIR".to_owned());

        let s = "PAYPALISHIRING".to_owned();
        let result = Solution::convert_to_zigzag_string(s, 4);
        assert_eq!(result, "PINALSIGYAHRPI".to_owned());

        let s = "A".to_owned();
        let result = Solution::convert_to_zigzag_string(s, 1);
        assert_eq!(result, "A".to_owned());
    }
}
