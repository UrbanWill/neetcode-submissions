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
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut count = 0;
        let mut result = 0;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut i32, count: &mut i32, k: i32) {
            match root {
                None => return,
                Some(node) => {
                    let node = node.borrow();
                    dfs(&node.left, result, count, k);
                    *count += 1;
                    if *count == k {
                        *result = node.val;
                        return;
                    }
                    dfs(&node.right, result, count, k);
                }
            }
        }
        dfs(&root, &mut result, &mut count, k);
        result
    }
}