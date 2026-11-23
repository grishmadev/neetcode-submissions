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

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Rc<RefCell<TreeNode>>;
struct Codec;
impl Codec {
    fn new() -> Self {
        Self
    }

    fn serialize(&self, root: Option<Tree>) -> String {
        let mut nodes = vec![];
        Self::ser_dfs(&root, &mut nodes);
        nodes.join("-")
    }

    fn ser_dfs(root: &Option<Tree>, nodes: &mut Vec<String>) {
        let Some(node) = root else {
            nodes.push("#".into());
            return;
        };
        let n = node.borrow();
        nodes.push(n.val.to_string());
        Self::ser_dfs(&n.left, nodes);
        Self::ser_dfs(&n.right, nodes);
    }

    fn deserialize(&self, data: String) -> Option<Tree> {
        let vals: Vec<&str> = data.split('-').collect();
        let mut i = 0;
        Self::des_dfs(&vals, &mut i)
    }

    fn des_dfs(vals: &[&str], i: &mut usize) -> Option<Tree> {
        if vals[*i] == "#" {
            *i += 1;
            return None;
        }
        let val = vals[*i].parse::<i32>().unwrap();
        *i += 1;
        let left = Self::des_dfs(vals, i);
        let right = Self::des_dfs(vals, i);
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}
