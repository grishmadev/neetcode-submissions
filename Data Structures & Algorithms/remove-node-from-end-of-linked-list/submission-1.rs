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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {val: 0, next: head});
        let mut len = 0;
        {
            let mut cur = &dummy.next;
            while let Some(node) = cur {
                len += 1;
                cur = &node.next;
            }
        }
        let target = len - n as usize;
        let mut cur = &mut dummy;
        for _ in 0..target {
            cur = cur.next.as_mut().unwrap();
        }
        let next = cur.next.as_mut().unwrap().next.take();
        cur.next = next;
        dummy.next
    }
}
