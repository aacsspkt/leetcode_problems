use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strs = strs;
        strs.sort();

        let first = strs.first().unwrap();
        let last = strs.last().unwrap();
        
        let mut index = 0;
        let min = std::cmp::min(first.len(), last.len());

        for i in 0..min {
            if first.chars().nth(i).unwrap() == last.chars().nth(i).unwrap() {
                index += 1;
            } else {
                break;
            }
        }
        first[..index].to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let strs = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
        let result = Solution::longest_common_prefix(strs);
        assert_eq!(result, "fl".to_string());

        let strs = vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
        let result = Solution::longest_common_prefix(strs);
        assert_eq!(result, "".to_string());
    }
}