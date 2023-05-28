/**
 * # 19. Remove Nth Node From End of List - https://leetcode.com/problems/remove-nth-node-from-end-of-list/
 * Given the head of a linked list, remove the nth node from the end of the list and return its head.
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[cfg(test)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

#[cfg(test)]
fn to_list(nums: &[&[i32]]) -> Option<Box<ListNode>> {
    let count = nums.iter().flat_map(|e| *e).sum::<i32>();
    if count == 0 {
        return None;
    }
    let mut nums = nums.into_iter().flat_map(|e| *e);
    let mut head = Box::new(ListNode::new(*nums.next().unwrap()));
    let mut current = &mut head;
    for n in nums {
        current.next = Some(Box::new(ListNode::new(*n)));
        current = current.next.as_mut().unwrap();
    }
    Some(head)
}

#[cfg(test)]
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut buffer = vec![];
        while let Some(n) = head {
            buffer.push(n.val);
            head = n.next
        }
        if buffer.len() < 1 {
            return None;
        }
        let objective = buffer.len() as i32 - n;
        return if objective <= 0 {
            to_list(&[&buffer[1..]])
        } else {
            to_list(&[
                &buffer[..(objective as usize)],
                &buffer[(objective as usize + 1)..],
            ])
        };
    }
}

#[cfg(test)]
mod test {
    use crate::s0019::{ListNode, to_list};
    use super::Solution;

    fn from_vec(list: Vec<i32>) -> Option<Box<ListNode>> {
        to_list(&[&list])
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::remove_nth_from_end(from_vec(vec![1, 2, 3, 4, 5]), 2),
            from_vec(vec![1, 2, 3, 5])
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::remove_nth_from_end(from_vec(vec![1]), 1),
            from_vec(vec![])
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::remove_nth_from_end(from_vec(vec![1,2]), 1),
            from_vec(vec![1])
        )
    }
}
