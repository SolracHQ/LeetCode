/**
 * 2. Add Two Numbers - https://leetcode.com/problems/add-two-numbers/
 *
 * You are given two non-empty linked lists representing two non-negative integers.
 * The digits are stored in reverse order, and each of their nodes contains a single digit.
 * Add the two numbers and return the sum as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 */
#[cfg(test)]
struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[cfg(test)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Recursive function that adds two linked lists and a carry value
        fn add_lists(l1: Option<&Box<ListNode>>, l2: Option<&Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
            match (l1, l2) {
                // Base case: both linked lists are empty and carry is 0
                (None, None) => if carry == 0 { None }
                // Base case: both linked lists are empty and carry is != 0 Create a last node with carry
                else { Some(Box::new(ListNode::new(carry))) },

                // Case: one linked list is empty and the other is not
                (Some(n1), None) | (None, Some(n1)) => {
                    let sum = n1.val + carry;
                    let carry = sum / 10;
                    let val = sum % 10;
                    // Create a new ListNode with the sum and recurse with the remaining elements and the carry
                    Some(Box::new(ListNode { val, next: add_lists(n1.next.as_ref(), None, carry) }))
                }

                // Case: both linked lists have elements
                (Some(n1), Some(n2)) => {
                    let sum = n1.val + n2.val + carry;
                    let carry = sum / 10;
                    let val = sum % 10;
                    // Create a new ListNode with the sum and recurse with the remaining elements and the carry
                    Some(Box::new(ListNode { val, next: add_lists(n1.next.as_ref(), n2.next.as_ref(), carry) }))
                }
            }
        }

        // Call the add_lists function with the input linked lists and carry value of 0
        add_lists(l1.as_ref(), l2.as_ref(), 0)
    }
}

#[cfg(test)]
mod test {
    use crate::s0002::ListNode;
    use super::Solution;

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
        assert_eq!(
            to_node(&[7, 0, 8]),
            Solution::add_two_numbers(
                to_node(&[2, 4, 3]),
                to_node(&[5, 6, 4])
            )
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            to_node(&[0]),
            Solution::add_two_numbers(
                to_node(&[0]),
                to_node(&[0])
            )
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            to_node(&[8,9,9,9,0,0,0,1]),
            Solution::add_two_numbers(
                to_node(&[9,9,9,9,9,9,9]),
                to_node(&[9,9,9,9])
            )
        )
    }
}