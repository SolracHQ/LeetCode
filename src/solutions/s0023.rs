/// 23. Merge k Sorted Lists - https://leetcode.com/problems/merge-k-sorted-lists/description/
/// You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
/// Merge all the linked-lists into one sorted linked-list and return it.
struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn to_node(vec: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

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
    use super::to_node;
    use super::Solution;    

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::merge_k_lists(vec![to_node(&[1,4,5]),to_node(&[1,3,4]),to_node(&[2,6])]),
            to_node(&[1,1,2,3,4,4,5,6])
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::merge_k_lists(vec![]),
            to_node(&[])
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::merge_k_lists(vec![to_node(&[])]),
            to_node(&[])
        )
    }
}