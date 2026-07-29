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
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
            match root {
                None => (true, 0),
                Some(node) => {
                    let node = node.borrow();
                    let left = dfs(&node.left);
                    let right = dfs(&node.right);
                    let balanced = left.0 && right.0 && (left.1 - right.1).abs() <= 1;
                    (balanced, 1 + left.1.max(right.1))
                }
            }
        }

        dfs(&root).0
    }
}
