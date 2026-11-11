use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut cur = head.clone();
        while let Some(node) = cur {
            let copy = Rc::new(RefCell::new(Node::new(node.borrow().val)));
            copy.borrow_mut().next = node.borrow().random.clone();
            node.borrow_mut().random = Some(copy);
            cur = node.borrow().next.clone();
        }

        let mut cur = head.clone();
        while let Some(node) = cur {
            let copy = node.borrow().random.clone().unwrap();
            let random_copy = copy
                .borrow()
                .next
                .as_ref()
                .map(|f| f.borrow().random.clone().unwrap());
            copy.borrow_mut().random = random_copy;
            cur = node.borrow().next.clone();
        }

        let new_head = head.as_ref().unwrap().borrow().random.clone();
        let mut cur = head.clone();
        while let Some(node) = cur {
            let copy = node.borrow().random.clone().unwrap();
            let next = node.borrow().next.clone();
            let def_rand = copy.borrow().next.clone();
            node.borrow_mut().random = def_rand;
            let copy_next = next.as_ref().map(|f| f.borrow().random.clone().unwrap());
            copy.borrow_mut().next = copy_next;
            cur = next;
        }

        new_head
    }
}
