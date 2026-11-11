use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut map = HashMap::new();
        let mut cur = head.clone();
        while let Some(node) = cur {
            map.insert(
                Rc::as_ptr(&node) as usize,
                Rc::new(RefCell::new(Node::new(node.borrow().val)))
            );
            cur = node.borrow().next.clone();
        }
        cur = head.clone();
        while let Some(node) = cur {
            let copy = map[&(Rc::as_ptr(&node) as usize)].clone();
            copy.borrow_mut().next = node.borrow().next.as_ref().map(|n| map[&(Rc::as_ptr(&n) as usize)].clone());
            copy.borrow_mut().random = node.borrow().random.as_ref().map(|n| map[&(Rc::as_ptr(&n) as usize)].clone());
            cur = node.borrow().next.clone();
        }
        head.map(|n| map[&(Rc::as_ptr(&n) as usize)].clone())
    }
}
