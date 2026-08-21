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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::dfs(&root, 0, target_sum)
    }
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cur: i32, target: i32) -> bool {
        match root {
            None => false,
            Some(node) => {
                let n = node.borrow();
                let new_sum = n.val + cur;

                match (&n.left, &n.right) {
                    (None, None) => new_sum == target,
                    _ => Self::dfs(&n.left, new_sum, target) || Self::dfs(&n.right, new_sum, target)
                }
            }
        }
    }
}
