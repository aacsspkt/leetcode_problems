use crate::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut high = nums.len();
        let mut low = 0_usize;
        let mut middle;
        
        if target > nums[high - 1] {
            return high as i32;
        } else if target < nums[0] {
            return 0;
        }

        while low < high {
            middle = (high + low) / 2;

            match target.cmp(&nums[middle]) {
                std::cmp::Ordering::Equal => return middle as i32,
                std::cmp::Ordering::Greater => {
                    low = middle + 1;
                }
                std::cmp::Ordering::Less => {
                    high = middle;
                }
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works(){
        let nums = vec![1,3,5,6];
        let target = 5;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 2);

        let nums = vec![1,3,5,6];
        let target = 2;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 1);

        let nums = vec![1,3,5,6];
        let target = 7;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 4);

        let nums = vec![1,3,5,6];
        let target = 0;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 0);
    }
}