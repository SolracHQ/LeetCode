/**
 * Problem 206. Reverse Linked List
 * Given the head of a singly linked list, reverse the list, and return the reversed list.
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let None = head {
            return None;
        }
        let mut stack = vec![];
        let mut current = head;
        while let Some(node) = current {
            stack.push(node.val);
            current = node.next;
        }
        let mut result = ListNode::new(stack.pop().unwrap());
        let mut current = &mut result;
        while let Some(val) = stack.pop() {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        Some(Box::new(result))
    }
}

mod test {
    use super::ListNode;
    use super::Solution;

    fn slice_to_node(slice: &[i32]) -> Option<Box<ListNode>> {
        if slice.is_empty() {
            return None;
        }
        let mut result = ListNode::new(slice[0]);
        let mut current = &mut result;
        for i in 1..slice.len() {
            current.next = Some(Box::new(ListNode::new(slice[i])));
            current = current.next.as_mut().unwrap();
        }
        Some(Box::new(result))
    }

    #[test]
    fn example1() {
        let head = slice_to_node(&[1, 2, 3, 4, 5]);
        let expected = slice_to_node(&[5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list(head), expected);
    }

    #[test]
    fn example2() {
        let head = slice_to_node(&[1, 2]);
        let expected = slice_to_node(&[2, 1]);
        assert_eq!(Solution::reverse_list(head), expected);
    }

    #[test]
    fn example3() {
        let head = slice_to_node(&[]);
        let expected = slice_to_node(&[]);
        assert_eq!(Solution::reverse_list(head), expected);
    }
}
