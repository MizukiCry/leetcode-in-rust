impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut v = Vec::<i32>::new();
        for i in lists {
            let mut p = i;
            while p.is_some() {
                v.push(p.clone().unwrap().val);
                p = p.unwrap().next;
            }
        }
        v.sort();
        let mut p: Option<Box<ListNode>> = None;
        for i in v.iter().rev() {
            p = Some(Box::new(ListNode { next: p, val: i.clone() }));
        }
        p
    }
}