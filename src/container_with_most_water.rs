use std::cmp;

use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;

        while left < right {
            let area = (right - left) as i32 * cmp::min(height[left], height[right]);
            max_area = cmp::max(area, max_area);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(height);
        assert_eq!(result, 49);

        let height = vec![1, 1];
        let result = Solution::max_area(height);
        assert_eq!(result, 1);
    }
}
