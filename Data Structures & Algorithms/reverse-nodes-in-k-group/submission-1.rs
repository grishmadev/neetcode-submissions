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
        let k = k as usize;
        while let Some(node) = tail {
            nums.push(node.val);
            tail = &mut node.next;
        }
        // println!("{nums:?}");
        tail = &mut head;
        let n = nums.len();
        for i in (0..nums.len()).step_by(k) {
            // println!("i: {i}");
            if i + k >= nums.len() + 1 {
                return head;
            }
            for j in (i..k+i).rev() {
                // println!("j: {j}");
                let n = nums[j];
                tail.as_mut().unwrap().val = n;
                tail = &mut tail.as_mut().unwrap().next;
            }
        }
        head
    }
}
