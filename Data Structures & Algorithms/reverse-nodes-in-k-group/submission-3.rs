// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;
        loop {
            let mut count = 0;
            let mut check = cur.next.as_ref();
            while let Some(node) = check {
                count += 1;
                if count == k {
                    break;
                }
                check = node.next.as_ref();
            }
            // list smaller than k
            if count < k {
                break;
            }

            let rest = Self::split_off(&mut cur.next, k);
            let group = cur.next.take();
            cur.next = Self::reverse(group);

            for _ in 0..k {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = rest;
        }

        dummy.next
    }

    fn split_off(head: &mut Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut cur = head;
        for _ in 0..k - 1 {
            cur = &mut cur.as_mut().unwrap().next;
        }
        cur.as_mut().unwrap().next.take()
    }

    fn reverse(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = list.take() {
            list = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}
