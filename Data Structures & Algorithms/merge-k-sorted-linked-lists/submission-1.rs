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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        while lists.len() > 1 {
            let mut merged = Vec::new();
            for i in (0..lists.len()).step_by(2) {
                let l1 = lists[i].take();
                let l2 = if i + 1 < lists.len() { lists[i + 1].take() } else { None };
                merged.push(Self::merge_list(l1, l2));
            }
            lists = merged;
        }
        lists.into_iter().next().unwrap()
    }

    fn merge_list(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut *dummy;
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let next = l1.as_mut().unwrap().next.take();
                tail.next = l1;
                l1 = next;
            } else {
                let next = l2.as_mut().unwrap().next.take();
                tail.next = l2;
                l2 = next;
            }
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if l1.is_some() {l1} else {l2};
        dummy.next
    }
}