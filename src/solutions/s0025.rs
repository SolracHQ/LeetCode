/**
* 25. Reverse Nodes in k-Group
*
* Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.

*
* Example 1:
* Input: head = [1,2,3,4,5], k = 2
* Output: [2,1,4,3,5]
*
* Example 2:
* Input: head = [1,2,3,4,5], k = 3
* Output: [3,2,1,4,5]

*/
#[cfg(test)]
struct Solution;

use crate::ListNode;

#[cfg(test)]
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
        // Input: head = [1,2,3,4,5], k = 2
        // Expected: [2,1,4,3,5]
        assert_eq!(
            to_node(&[2, 1, 4, 3, 5]),
            Solution::reverse_k_group(to_node(&[1, 2, 3, 4, 5]), 2)
        );
    }

    #[test]
    fn example_2() {
        // Input: head = [1,2,3,4,5], k = 3
        // Expected: [3,2,1,4,5]
        assert_eq!(
            to_node(&[3, 2, 1, 4, 5]),
            Solution::reverse_k_group(to_node(&[1, 2, 3, 4, 5]), 3)
        );
    }
}
