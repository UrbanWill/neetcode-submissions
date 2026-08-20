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
        Self::leaf_path(&root, 0, target_sum)
    }
    fn leaf_path(root: &Option<Rc<RefCell<TreeNode>>>, cur: i32, target_sum: i32 ) -> bool {
        match root {
            None => false,
            Some(node) => {
                let n = node.borrow();
                let new_sum = cur + n.val;

                match (&n.left, &n.right) {
                    (None, None) => new_sum == target_sum,
                    _ => Self::leaf_path(&n.left, new_sum, target_sum) || Self::leaf_path(&n.right, new_sum, target_sum)
                }
            }
        }
    }
}
