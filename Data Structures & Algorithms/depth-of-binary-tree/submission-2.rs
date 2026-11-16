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

type Tree = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn max_depth(root: Option<Tree>) -> i32 {
        // iterative depth first search
        if root.is_none() {return 0};
        let mut stk = vec![(root.unwrap(), 1)];
        let mut max = 0;
        while !stk.is_empty() {
            let (node, lvl) = stk.pop().unwrap();
            let node = node.borrow();
            if let Some(ref l) = node.left {
                stk.push((l.clone(), lvl + 1));
            }
            if let Some(ref r) = node.right {
                stk.push((r.clone(), lvl + 1));
            }
            max = lvl + 1;
        }
        max
    }
}
