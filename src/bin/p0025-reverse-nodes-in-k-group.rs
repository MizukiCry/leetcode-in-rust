type Pointer = Option<Box<ListNode>>;

fn len(mut head: &Pointer) -> i32 {
    let mut res = 0;
    while let Some(cur) = head {
        head = &cur.next;
        res += 1;
    }
    res
}

fn reverse(mut head: Pointer) -> Pointer {
    let mut res: Pointer = None;
    while let Some(mut cur) = head {
        head = cur.next.take();
        cur.next = res;
        res = Some(cur);
    }
    res
}

fn connect(mut tail: &mut Box<ListNode>, other: Pointer) -> &mut Box<ListNode> {
    tail.next = other;
    while tail.next.is_some() {
        tail = tail.next.as_mut().unwrap();
    }
    tail
}

impl Solution {
    pub fn reverse_k_group(mut head: Pointer, k: i32) -> Pointer {
        let len = len(&head);
        let mut res_head = Some(Box::new(ListNode::new(0)));
        let mut res_tail = res_head.as_mut().unwrap();
        for _ in 0..len / k {
            let mut p = head.as_mut().unwrap();
            for _ in 0..k - 1 {
                p = p.next.as_mut().unwrap();
            }
            let tail = p.next.take();
            res_tail = connect(res_tail, reverse(head));
            head = tail;
        }
        res_tail.next = head;
        res_head.unwrap().next.take()
    }
}