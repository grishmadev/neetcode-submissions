use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        head.as_ref()?;
        let mut cur = head.clone();
        while let Some(node) = cur {
            // creating a copy of the following node
            let copy = Rc::new(RefCell::new(Node::new(node.borrow().val)));
            // attaching original node's `random` to it's copy's `next`
            copy.borrow_mut().next = node.borrow().random.clone();
            // attaching the copy of the node to the original node's `random`
            node.borrow_mut().random = Some(copy);
            // moving `cur` forward
            cur = node.borrow().next.clone();
        }

        let mut cur = head.clone();
        while let Some(node) = cur {
            let copy = node.borrow().random.clone().unwrap();
            let random_copy = copy.borrow().next.as_ref().map(|f| f.borrow().random.clone().unwrap());
            copy.borrow_mut().random = random_copy;
            cur = node.borrow().next.clone();
        }

        let new_head = head.as_ref().unwrap().borrow().random.clone();
        let mut cur = head.clone();
        while let Some(node) = cur {
            let copy = node.borrow().random.clone().unwrap();
            let next = node.borrow().next.clone();
            let orig_rand = copy.borrow().next.clone();
            node.borrow_mut().random = orig_rand;
            let copy_next = next.as_ref().map(|f| f.borrow().random.clone().unwrap());
            copy.borrow_mut().next = copy_next;
            cur = next;
        }

        new_head
    }
}
