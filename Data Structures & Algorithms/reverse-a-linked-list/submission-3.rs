// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub vals: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(vals: i32) -> Self {
//         ListNode { next: None, vals }
//     }
// }

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vals = VecDeque::<i32>::new();
        let Some(mut input) = head else {
            return None;
        };
        vals.push_back(input.val);
        while let Some(node) = input.next {
            vals.push_back(node.val);
            input = node;
        }
        let mut res = None;
        while let Some(num) = vals.pop_front() {
            res = Some(Box::new(ListNode {val: num, next: res}));
        }
        res
    }
}
