// Definition for a Node.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: i32,
//     pub next: Option<Rc<RefCell<Node>>>,
//     pub random: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         Node {
//             val,
//             next: None,
//             random: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut map = HashMap::new();
        let mut cur = head.clone();
        while let Some(node) = cur {
            let copy = Rc::new(RefCell::new(Node::new(node.borrow().val)));
            map.insert(Rc::as_ptr(&node), copy);
            cur = node.borrow().next.clone();
        }
        let mut cur = head.clone();
        while let Some(node) = cur {
            let copy = map[&Rc::as_ptr(&node)].clone();
            let next = node.borrow().next.as_ref().map(|f| map[&Rc::as_ptr(f)].clone());
            copy.borrow_mut().next = next;
            let random = node.borrow().random.as_ref().map(|f| map[&Rc::as_ptr(f)].clone());
            copy.borrow_mut().random = random;
            cur = node.borrow().next.clone();
        }

        head.as_ref().map(|f| map[&Rc::as_ptr(f)].clone())
    }
}
