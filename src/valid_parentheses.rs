use crate::Solution;

impl Solution {
    pub fn is_valid_parentheses(s: String) -> bool {
        let mut stack = Vec::<char>::new();
        for ch in s.chars() {
            if ch == '(' || ch == '{' || ch == '[' {
                stack.push(ch);
            } else {
                if stack.len() > 0 {
                    let last = *stack.get(stack.len() - 1).unwrap();
                    if (last == '(' && ch == ')')
                        || (last == '{' && ch == '}')
                        || (last == '[' && ch == ']')
                    {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let s = "()".to_owned();
        let result = Solution::is_valid_parentheses(s);
        assert_eq!(result, true);

        let s = "()[]{}".to_owned();
        let result = Solution::is_valid_parentheses(s);
        assert_eq!(result, true);

        let s = "(]".to_owned();
        let result = Solution::is_valid_parentheses(s);
        assert_eq!(result, false);

        let s = "[{()}]".to_owned();
        let result = Solution::is_valid_parentheses(s);
        assert_eq!(result, true);

        let s = "}[{()}]{".to_owned();
        let result = Solution::is_valid_parentheses(s);
        assert_eq!(result, false);
    }
}
