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
        let mut sorted = Vec::new();
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, sorted: &mut Vec<i32>) {
            match root {
                None => return,
                Some(node) => {
                    let node = node.borrow();
                    dfs(&node.left, sorted);
                    sorted.push(node.val);
                    dfs(&node.right, sorted);
                }
            }
        }
        dfs(&root, &mut sorted);
        println!("sorted: {:?}", sorted);
        sorted[(k - 1) as usize]
    }
}
