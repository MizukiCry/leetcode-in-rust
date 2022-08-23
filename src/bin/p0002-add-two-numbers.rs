fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut v: i32) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && v == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: add(
                l1.and_then(|x| {v += x.val; x.next}),
                l2.and_then(|x| {v += x.val; x.next}),
                v / 10
                ),
            val: v % 10
        }))
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        add(l1, l2, 0)
    }
}