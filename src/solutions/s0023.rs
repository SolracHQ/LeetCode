/**
* 23. Merge k Sorted Lists
*
* You are given an array of k linked-lists lists, each linked-list is sorted in ascending order. Merge all the linked-lists into one sorted linked-list and return it.

*
* Example 1:
* Input: lists = [[1,4,5],[1,3,4],[2,6]]
* Output: [1,1,2,3,4,4,5,6]
*
* Example 2:
* Input: lists = []
* Output: []
*
* Example 3:
* Input: lists = [[]]
* Output: []

*/
#[cfg(test)]
struct Solution;

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

#[cfg(test)]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut collector = vec![];
        for node in lists {
            let mut current = &node;
            while let Some(node) = current {
                collector.push(node.val);
                current = &node.next;
            }
        }
        collector.sort_unstable();
        to_node(&collector)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: lists = [[1,4,5],[1,3,4],[2,6]]
        // Expected: [1,1,2,3,4,4,5,6]
        assert_eq!(
            super::to_node(&[1, 1, 2, 3, 4, 4, 5, 6]),
            Solution::merge_k_lists(vec![
                super::to_node(&[1, 4, 5]),
                super::to_node(&[1, 3, 4]),
                super::to_node(&[2, 6])
            ])
        );
    }

    #[test]
    fn example_2() {
        // Input: lists = []
        // Expected: []
        assert_eq!(None, Solution::merge_k_lists(vec![]));
    }

    #[test]
    fn example_3() {
        // Input: lists = [[]]
        // Expected: []
        assert_eq!(None, Solution::merge_k_lists(vec![None]));
    }
}
