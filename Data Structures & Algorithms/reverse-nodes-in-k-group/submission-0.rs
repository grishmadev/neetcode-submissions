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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut nums = vec![];
        let mut tail = &mut head;
        while let Some(node) = tail {
            nums.push(node.val);
            tail = &mut node.next;
        }
        if k as usize >= nums.len() {
            return head;
        }
        tail = &mut head;
        for i in (0..k).rev() {
            let n = nums[i as usize];
            tail.as_mut().unwrap().val = n;
            tail = &mut tail.as_mut().unwrap().next;
        }
        let k = k as usize;
        if nums.len() - k < k {
            return head;
        }
        for i in (k..nums.len()).rev() {
            let n = nums[i];
            tail.as_mut().unwrap().val = n;
            tail = &mut tail.as_mut().unwrap().next;
        }
        head
    }
}
