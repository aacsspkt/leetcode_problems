use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0;

        for i in 1..nums.len() {
            if nums[idx] != nums[i] {
                idx+=1;
                nums[idx] = nums[i];
            }
        }
        (idx + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let mut nums = vec![1, 1, 2];
        let expected = vec![1, 2];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, expected.len() as i32);
        for i in 0..k as usize {
            assert_eq!(nums[i], expected[i]);
        }
    }
}
