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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1 == None {
            return list2;
        } else if list2 == None {
            return list1;
        }
        let mut res = ListNode::new(0);
        let mut tail = &mut res.next;
        while let Some(ref node1) = list1 && let Some(ref node2) = list2 {
            if node1.val <= node2.val {
                let Some(node1) = list1 else {
                    break;
                };
                
                *tail = Some(Box::new(ListNode::new(node1.val)));
                tail = &mut tail.as_mut().unwrap().next;
                list1 = node1.next;
            } else {
                let Some(node2) = list2 else {
                    break;
                };

                *tail = Some(Box::new(ListNode::new(node2.val)));
                tail = &mut tail.as_mut().unwrap().next;
                list2 = node2.next;
            }
        }
        *tail = list1.or(list2);

        res.next
    }
}
