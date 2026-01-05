/**
* 19. Remove Nth Node From End of List
*
* Given the head of a linked list, remove the nth node from the end of the list and return its head.

*
* Example 1:
* Input: head = [1,2,3,4,5], n = 2
* Output: [1,2,3,5]
*
* Example 2:
* Input: head = [1], n = 1
* Output: []
*
* Example 3:
* Input: head = [1,2], n = 1
* Output: [1]

*/
#[cfg(test)]
struct Solution;

use crate::ListNode;

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
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: head = [1,2,3,4,5], n = 2
        // Expected: [1,2,3,5]
        assert_eq!(
            super::to_list(&[&[1, 2, 3, 5]]),
            Solution::remove_nth_from_end(super::to_list(&[&[1, 2, 3, 4, 5]]), 2)
        );
    }

    #[test]
    fn example_2() {
        // Input: head = [1], n = 1
        // Expected: []
        assert_eq!(
            None,
            Solution::remove_nth_from_end(super::to_list(&[&[1]]), 1)
        );
    }

    #[test]
    fn example_3() {
        // Input: head = [1,2], n = 1
        // Expected: [1]
        assert_eq!(
            super::to_list(&[&[1]]),
            Solution::remove_nth_from_end(super::to_list(&[&[1, 2]]), 1)
        );
    }
}
