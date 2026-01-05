/**
* 21. Merge Two Sorted Lists
*
* You are given the heads of two sorted linked lists list1 and list2. Merge the two lists into one sorted list.

*
* Example 1:
* Input: list1 = [1,2,4], list2 = [1,3,4]
* Output: [1,1,2,3,4,4]
*
* Example 2:
* Input: list1 = [], list2 = []
* Output: []
*
* Example 3:
* Input: list1 = [], list2 = [0]
* Output: [0]

*/
#[cfg(test)]
struct Solution;

use crate::ListNode;

#[cfg(test)]
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(node1), Some(node2)) => {
                if node1.val < node2.val {
                    Some(Box::new(ListNode {
                        val: node1.val,
                        next: Self::merge_two_lists(node1.next, Some(node2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: node2.val,
                        next: Self::merge_two_lists(Some(node1), node2.next),
                    }))
                }
            }
            (Some(node), None) => Some(node),
            (None, Some(node)) => Some(node),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::ListNode;

    fn to_node(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn example_1() {
        // Input: list1 = [1,2,4], list2 = [1,3,4]
        // Expected: [1,1,2,3,4,4]
        assert_eq!(
            to_node(&[1, 1, 2, 3, 4, 4]),
            Solution::merge_two_lists(to_node(&[1, 2, 4]), to_node(&[1, 3, 4]))
        );
    }

    #[test]
    fn example_2() {
        // Input: list1 = [], list2 = []
        // Expected: []
        assert_eq!(None, Solution::merge_two_lists(None, None));
    }

    #[test]
    fn example_3() {
        // Input: list1 = [], list2 = [0]
        // Expected: [0]
        assert_eq!(
            to_node(&[0]),
            Solution::merge_two_lists(None, to_node(&[0]))
        );
    }
}
