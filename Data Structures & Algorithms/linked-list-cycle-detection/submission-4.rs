// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: *mut ListNode,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: std::ptr::null_mut(), val }
//     }
// }

impl Solution {
    pub fn has_cycle(mut head: *mut ListNode) -> bool {
        let mut set = HashSet::new();
        let mut res = ListNode::new(0);

        loop {
            if head.is_null() {
                break;
            } else {
                let addr = head as usize;
                if set.contains(&addr)  {
                    return true;
                }
                set.insert(addr);
                head = unsafe { (*head).next };
            }
        }
        false
    }
}
