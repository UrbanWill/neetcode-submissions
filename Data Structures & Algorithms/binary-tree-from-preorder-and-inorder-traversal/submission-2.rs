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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }

        let root_val = preorder[0];
        let mid = inorder.iter().position(|&x| x == root_val).unwrap();

        let mut root = TreeNode::new(root_val);

        root.left = Self::build_tree(
            preorder[1..=mid].to_vec(),
            inorder[..mid].to_vec()
        );

        root.right = Self::build_tree(
            preorder[mid + 1..].to_vec(),
            inorder[mid + 1..].to_vec()
        );

        Some(Rc::new(RefCell::new(root)))

    }
}
