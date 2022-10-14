// https://leetcode.com/problems/add-two-numbers/

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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res = 0;
    let (l1, l2) = (l1.unwrap(), l2.unwrap());
    let mut answer = ListNode::new(add_with_res(l1.val, l2.val, &mut res));
    let mut current = &mut answer;
    let mut l1 = &l1.next;
    let mut l2 = &l2.next;
    loop {
        if l1.is_none() && l2.is_none() {
            break;
        } else if l1.is_none() {
            current.next = Some(Box::new(ListNode::new(add_with_res(
                0,
                l2.as_ref().unwrap().val,
                &mut res,
            ))));
            l2 = &l2.as_ref().unwrap().next;
        } else if l2.is_none() {
            current.next = Some(Box::new(ListNode::new(add_with_res(
                0,
                l1.as_ref().unwrap().val.clone(),
                &mut res,
            ))));
            l1 = &l1.as_ref().unwrap().next;
        } else {
            current.next = Some(Box::new(ListNode::new(add_with_res(
                l1.as_ref().unwrap().val.clone(),
                l2.as_ref().unwrap().val.clone(),
                &mut res,
            ))));
            l1 = &l1.as_ref().unwrap().next;
            l2 = &l2.as_ref().unwrap().next;
        }
        current = current.next.as_mut().unwrap();
    }
    if res != 0 {
        current.next = Some(Box::new(ListNode::new(res)));
    }
    Some(Box::new(answer))
}

fn add_with_res(n1: i32, n2: i32, res: &mut i32) -> i32 {
    let sum = n1 + n2 + *res;
    if sum > 9 {
        *res = sum / 10;
        return sum % 10;
    }
    *res = 0;
    sum
}
