use crate::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut idx = 0_i32;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums.swap(i, idx as usize);
                idx+=1;
            } 
        }
        idx
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let mut nums = vec![0,1,2,2,3,0,4,2];
        let mut expected = vec![0,1,4,0,3];
        let k = Solution::remove_element(&mut nums, 2);
        println!("Output {}: {:#?}", k, nums);
        assert_eq!(k, expected.len() as i32);
        let mut nums = nums[..k as usize].to_vec();
        nums.sort();
        expected.sort();
        for i in 0..k as usize {
            assert_eq!(nums[i], expected[i]);
        }
    }
}
