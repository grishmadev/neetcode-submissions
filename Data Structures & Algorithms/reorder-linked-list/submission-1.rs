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
    pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        let mut idx = 0;
        let mut list = [0; 1024];
        let mut new_head = &mut *head;
        while let Some(node) = new_head {
            list[idx] = node.val;
            idx += 1;
            new_head = &mut node.next;
        }
        let end_idx = idx;
        let (mut l, mut r) = (0, end_idx - 1);
        let mut turn_left = true;
        while let Some(node) = head {
            if turn_left {
                node.val = list[l];
                l += 1;
            } else {
                node.val = list[r];
                r -= 1;            
            }
            head = &mut node.next;
            turn_left = !turn_left;
        }
    }
}
