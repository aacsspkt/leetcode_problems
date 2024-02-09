use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2; 
        
        let mut dummy_node = ListNode::new(-1);
        let mut tail =  &mut dummy_node;
        
        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            let val1 = node1.val;
            let val2 = node2.val;

            match val1.cmp(&val2) {
                std::cmp::Ordering::Equal => {
                    tail.next = Some(Box::new(ListNode {
                        val: val1,
                        next: None,
                    }));
                    tail = tail.next.as_mut().unwrap();
                    tail.next = Some(Box::new(ListNode {
                        val: val2,
                        next: None,
                    }));
                    list1 = list1.unwrap().next;
                    list2 = list2.unwrap().next;
                }
                std::cmp::Ordering::Greater => {
                    tail.next = Some(Box::new(ListNode {
                        val: val2,
                        next: None,
                    }));
                    list2 = list2.unwrap().next;
                }
                std::cmp::Ordering::Less => {
                    tail.next = Some(Box::new(ListNode {
                        val: val1,
                        next: None,
                    }));
                    list1 = list1.unwrap().next;
                }
            }
            tail = tail.next.as_mut().unwrap();

        }

        if list1.is_some() {
            tail.next = list1;
        } else {
            tail.next = list2;
        }

        dummy_node.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };
        let list2 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };

        let expected = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next:  None,
                            })),
                        })),
                    })),
                })),
            })),
        };

        let result = Solution::merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)));

        assert_eq!(result, Some(Box::new(expected)));

        let result = Solution::merge_two_lists(None, None);
        assert_eq!(result, None);

        let result = Solution::merge_two_lists(None, Some(Box::new(ListNode::new(0))));
        assert_eq!(result, Some(Box::new(ListNode::new(0))));
    }
}
