/**
 * # 21. Merge Two Sorted Lists - https://leetcode.com/problems/merge-two-sorted-lists/
 * You are given the heads of two sorted linked lists list1 and list2.
 * Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
 * Return the head of the merged linked list.
 */
#[cfg(test)]
struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg(test)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

#[cfg(test)]
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[cfg(test)]
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
    use super::{Solution, ListNode};

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
        assert_eq!(
            Solution::merge_two_lists(to_node(&[1,2,4]), to_node(&[1,3,4])),
            to_node(&[1,1,2,3,4,4])
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::merge_two_lists(to_node(&[]), to_node(&[])),
            to_node(&[])
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::merge_two_lists(to_node(&[]), to_node(&[0])),
            to_node(&[0])
        )
    }
}