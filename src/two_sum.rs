use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        let mut vec: Vec<i32> = vec![];
        for i in 0..(len - 1) {
            for j in (i + 1)..len {
                if nums[i] + nums[j] == target {
                    vec.push(i as i32);
                    vec.push(j as i32);
                }
            }
        }

        return vec;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);

        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);

        let nums = vec![3, 3];
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
