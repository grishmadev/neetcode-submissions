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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let mut stack = vec![root.clone().unwrap()];
        while let Some(node) = stack.pop() {
            let mut node_ref = node.borrow_mut();
            let left = node_ref.left.take();
            let right = node_ref.right.take();
            node_ref.left = right;
            node_ref.right = left;
            if node_ref.right.is_some() {stack.push(node_ref.right.clone().unwrap())};
            if node_ref.left.is_some() {stack.push(node_ref.left.clone().unwrap())};
        }
        root
    }
}
