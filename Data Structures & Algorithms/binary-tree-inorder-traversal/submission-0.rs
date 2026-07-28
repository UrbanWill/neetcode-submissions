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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::inorder(&root, &mut res);
        res
    }

    pub fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            Self::inorder(&n.left, res);
            res.push(n.val);
            Self::inorder(&n.right, res);
        }
    }
}
