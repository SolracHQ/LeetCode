/**
* 2. Add Two Numbers
*
* You are given two non-empty linked lists representing two non-negative integers. Add the two numbers and return the sum as a linked list.

*
* Example 1:
* Input: l1 = [2,4,3], l2 = [5,6,4]
* Output: [7,0,8]
*
* Example 2:
* Input: l1 = [0], l2 = [0]
* Output: [0]
*
* Example 3:
* Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
* Output: [8,9,9,9,0,0,0,1]

*/
#[cfg(test)]
struct Solution;

use crate::ListNode;

#[cfg(test)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Recursive function that adds two linked lists and a carry value
        fn add_lists(
            l1: Option<&Box<ListNode>>,
            l2: Option<&Box<ListNode>>,
            carry: i32,
        ) -> Option<Box<ListNode>> {
            match (l1, l2) {
                // Base case: both linked lists are empty and carry is 0
                (None, None) => {
                    if carry == 0 {
                        None
                    }
                    // Base case: both linked lists are empty and carry is != 0 Create a last node with carry
                    else {
                        Some(Box::new(ListNode::new(carry)))
                    }
                }

                // Case: one linked list is empty and the other is not
                (Some(n1), None) | (None, Some(n1)) => {
                    let sum = n1.val + carry;
                    let carry = sum / 10;
                    let val = sum % 10;
                    // Create a new ListNode with the sum and recurse with the remaining elements and the carry
                    Some(Box::new(ListNode {
                        val,
                        next: add_lists(n1.next.as_ref(), None, carry),
                    }))
                }

                // Case: both linked lists have elements
                (Some(n1), Some(n2)) => {
                    let sum = n1.val + n2.val + carry;
                    let carry = sum / 10;
                    let val = sum % 10;
                    // Create a new ListNode with the sum and recurse with the remaining elements and the carry
                    Some(Box::new(ListNode {
                        val,
                        next: add_lists(n1.next.as_ref(), n2.next.as_ref(), carry),
                    }))
                }
            }
        }

        // Call the add_lists function with the input linked lists and carry value of 0
        add_lists(l1.as_ref(), l2.as_ref(), 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_node(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        for &val in values {
            let node = ListNode::new(val);
            *tail = Some(Box::new(node));
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }

    #[test]
    fn example_1() {
        // Input: l1 = [2,4,3], l2 = [5,6,4]
        // Expected: [7,0,8]
        assert_eq!(
            to_node(&[7, 0, 8]),
            Solution::add_two_numbers(to_node(&[2, 4, 3]), to_node(&[5, 6, 4]))
        )
    }

    #[test]
    fn example_2() {
        // Input: l1 = [0], l2 = [0]
        // Expected: [0]
        assert_eq!(
            to_node(&[0]),
            Solution::add_two_numbers(to_node(&[0]), to_node(&[0]))
        )
    }

    #[test]
    fn example_3() {
        // Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
        // Expected: [8,9,9,9,0,0,0,1]
        assert_eq!(
            to_node(&[8, 9, 9, 9, 0, 0, 0, 1]),
            Solution::add_two_numbers(to_node(&[9, 9, 9, 9, 9, 9, 9]), to_node(&[9, 9, 9, 9]))
        )
    }
}
