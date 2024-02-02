use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn new_from_i32(num: i32) -> Self {
        if num == 0 {
            return ListNode {
                next: None,
                val: num,
            };
        }

        let mut digits: Vec<i32> = Vec::new();

        let mut quotient = num;

        while quotient != 0 {
            let modulos = quotient % 10;
            quotient = quotient / 10;
            digits.insert(0, modulos);
        }

        let mut tail = None;
        for digit in digits {
            let node = ListNode {
                next: tail,
                val: digit,
            };

            tail = Some(Box::new(node));
        }

        *tail.unwrap()
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;

        let mut l3 = Box::new(ListNode::new(0));

        let (mut l1, mut l2, mut pointer) = (l1.as_ref(), l2.as_ref(), l3.as_mut());

        while l1.is_some() || l2.is_some() {
            let mut sum = 0;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next.as_ref();
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next.as_ref();
            }

            sum += carry;
            carry = sum / 10;

            pointer.next = Some(Box::new(ListNode::new(sum % 10)));
            pointer = pointer.next.as_mut().unwrap();
        }

        if carry != 0 {
            pointer.next = Some(Box::new(ListNode::new(carry)));
        }

        l3.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node = ListNode::new_from_i32(9999999);
        let node1 = ListNode::new_from_i32(9999);
        let result = Solution::add_two_numbers(Some(Box::new(node)), Some(Box::new(node1)));
        assert_eq!(result, Some(Box::new(ListNode::new_from_i32(10009998))));

        let node = ListNode::new_from_i32(342);
        let node1 = ListNode::new_from_i32(465);
        let result = Solution::add_two_numbers(Some(Box::new(node)), Some(Box::new(node1)));
        assert_eq!(result, Some(Box::new(ListNode::new_from_i32(807))));

        let node = ListNode::new_from_i32(0);
        let node1 = ListNode::new_from_i32(0);
        let result = Solution::add_two_numbers(Some(Box::new(node)), Some(Box::new(node1)));
        assert_eq!(result, Some(Box::new(ListNode::new_from_i32(0))));
    }
}
