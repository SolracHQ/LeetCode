/**
 * Problem 25. Reverse Nodes in k-Group
 * Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.
 * k is a positive integer and is less than or equal to the length of the linked list.
 * if the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
 * You may not alter the values in the list's nodes, only nodes themselves may be changed.
 */
struct Solution;

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
}

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut node: &mut Option<Box<ListNode>> = &mut head;
        // check if there are k nodes left
        for _ in 0..k {
            if let Some(n) = node {
                node = &mut n.next;
            } else {
                // if not, return the head
                return head;
            }
        }
        // call reverse_k_group recursively keeping the firts k nodes on head
        let mut reversed: Option<Box<ListNode>> = Self::reverse_k_group(node.take(), k);
        // reverse the first k nodes
        while let Some(node) = head.take() {
            reversed = Some(Box::new(ListNode {
                val: node.val,
                next: reversed,
            }));
            head = node.next;
        }
        reversed
    }
}

mod test {
    use super::*;

    fn slice_to_node(slice: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in slice.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn example1() {
        let head = slice_to_node(&[1, 2, 3, 4, 5]);
        let expected = slice_to_node(&[2, 1, 4, 3, 5]);
        assert_eq!(Solution::reverse_k_group(head, 2), expected);
    }

    #[test]
    fn example2() {
        let head = slice_to_node(&[1, 2, 3, 4, 5]);
        let expected = slice_to_node(&[3, 2, 1, 4, 5]);
        assert_eq!(Solution::reverse_k_group(head, 3), expected);
    }
}
