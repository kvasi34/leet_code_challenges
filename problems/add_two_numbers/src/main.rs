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
    let mut sum = 0;
    let mut carry = 0;

    let mut l1 = l1.as_ref();
    let mut l2 = l2.as_ref();

    let mut head = Some(Box::new(ListNode { val: 0, next: None }));
    let mut current = head.as_mut();

    while l1.is_some() || l2.is_some() {
        sum = 0;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next.as_ref();
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next.as_ref();
        }

        sum += carry;
        carry = sum / 10;
        sum %= 10;

        current.as_mut().unwrap().next = Some(Box::new(ListNode {
            val: sum,
            next: None,
        }));
        current = current.unwrap().next.as_mut();
    }

    if carry != 0 {
        current.as_mut().unwrap().next = Some(Box::new(ListNode { val: carry, next: None }));
    }

    head.unwrap().next
}
