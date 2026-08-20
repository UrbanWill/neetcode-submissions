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
        Self::dfs(&root,0 ,target_sum)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cur: i32, target: i32) -> bool {
        if let Some(n) = root {
            let n = n.borrow();
            let new_sum = cur + n.val;

            if n.left.is_none() && n.right.is_none() {
                return new_sum == target;
            }

            return Self::dfs(&n.left, new_sum, target) || Self::dfs(&n.right, new_sum, target)

        } else {
            return false;
        }
    }
}
