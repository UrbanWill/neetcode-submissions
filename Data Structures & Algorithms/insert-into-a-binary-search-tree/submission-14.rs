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
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root { 
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(node) => {
                let mut n = node.borrow_mut();

                if val < n.val {
                    let new_left = Self::insert_into_bst(n.left.take(), val);

                    n.left = new_left;
                } else {
                    let new_right = Self::insert_into_bst(n.right.take(), val);

                    n.right = new_right;
                }
                drop(n);
                Some(node)
            }
        }
    }
}
