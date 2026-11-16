// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // breadth first search
        let mut q = VecDeque::new();
        if let Some(root) = root {
            q.push_back(root);
        }

        let mut count = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let node = node.borrow();
                if let Some(ref l) = node.left {
                    q.push_back(l.clone());
                }
                if let Some(ref r) = node.right {
                    q.push_back(r.clone());
                }
            }
            count += 1;
        }
        count
    }
}
