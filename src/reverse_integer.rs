use crate::Solution;

impl Solution {
    pub fn reverse_integer(x: i32) -> i32 {
        let mut reversed: i32 = 0;
        let mut quotient = x;

        while quotient != 0 {
            if let Some(product) = reversed.checked_mul(10) {
                if let Some(sum) = product.checked_add(quotient % 10) {
                    reversed = sum;
                } else {
                    reversed = 0;
                    break;
                }
            } else {
                reversed = 0;
                break;
            };
            quotient = quotient / 10;
        }

        reversed
    }
}
