/**
* 206. Reverse Linked List
*
* Given the head of a singly linked list, reverse the list, and return the reversed list.

*
* Example 1:
* Input: head = [1,2,3,4,5]
* Output: [5,4,3,2,1]
*
* Example 2:
* Input: head = [1,2]
* Output: [2,1]
*
* Example 3:
* Input: head = []
* Output: []

*/
#[cfg(test)]
struct Solution;

use crate::ListNode;

#[cfg(test)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut current = head;

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        prev
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
        // Input: head = [1,2,3,4,5]
        // Expected: [5,4,3,2,1]
        assert_eq!(
            to_node(&[5, 4, 3, 2, 1]),
            Solution::reverse_list(to_node(&[1, 2, 3, 4, 5]))
        );
    }

    #[test]
    fn example_2() {
        // Input: head = [1,2]
        // Expected: [2,1]
        assert_eq!(to_node(&[2, 1]), Solution::reverse_list(to_node(&[1, 2])));
    }

    #[test]
    fn example_3() {
        // Input: head = []
        // Expected: []
        assert_eq!(None, Solution::reverse_list(None));
    }
}
