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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root).0
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        match root {
            None => (true, 0),
            Some(node) => {
                let n = node.borrow();
                let l = Self::dfs(&n.left);
                let r = Self::dfs(&n.right);
                let balanced = l.0 && r.0 && (l.1 - r.1).abs() <= 1;
                (balanced, 1 + l.1.max(r.1))
            }
        }
    }
}
