use crate::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (l1, l2) = (nums1.len(), nums2.len());
        let (mut p1, mut p2) = (0, 0);

        let mut get_min = |v1: &Vec<i32>, v2: &Vec<i32>| -> i32 {
            if p1 < l1 && p2 < l2 {
                if v1[p1] < v2[p2] {
                    let item = v1[p1];
                    p1 += 1;
                    item
                } else {
                    let item = v2[p2];
                    p2 += 1;
                    item
                }
            } else if p1 < l1 {
                let item = v1[p1];
                p1 += 1;
                item
            } else if p2 < l2 {
                let item = v2[p2];
                p2 += 1;
                item
            } else {
                unreachable!()
            }
        };

        if (l1 + l2) % 2 == 0 {
            let high = ((l1 + l2) / 2) - 1;
            for _ in 0..high {
                _ = get_min(&nums1, &nums2);
            }
            let min1 = get_min(&nums1, &nums2);
            let min2 = get_min(&nums1, &nums2);

            (min1 + min2) as f64 / 2_f64
        } else {
            let high = (l1 + l2) / 2;
            for _ in 0..high {
                _ = get_min(&nums1, &nums2);
            }
            get_min(&nums1, &nums2) as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let result = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 2.0);

        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let result = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 2.5);
    }
}
