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
        let mut hare = head;
        let mut tortoise = head;
        while !hare.is_null() && unsafe { !(*hare).next.is_null() } {
            tortoise = unsafe { (*tortoise).next };
            hare = unsafe { (*(*hare).next).next };
            if hare == tortoise {
                return true;
            }
        }
        false
    }
}
