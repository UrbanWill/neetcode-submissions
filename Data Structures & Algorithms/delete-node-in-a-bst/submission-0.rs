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
use std::cmp::Ordering;

impl Solution {
    pub fn find_largest(subtree: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match subtree {
            None => None,
            Some(node) => {
                let right = node.borrow().right.clone();
                if right.is_none() {
                    Some(node)
                } else {
                    Self::find_largest(right)
                }
            }
        }
    }
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root?;
        let mut node = root.borrow_mut();

        match key.cmp(&node.val) {
            Ordering::Less => {
                node.left = Self::delete_node(node.left.take(), key);
            }
            Ordering::Greater => {
                node.right = Self::delete_node(node.right.take(), key);
            }
            Ordering::Equal => {
                if node.left.is_none() {
                    return node.right.take();
                } else {
                    if let Some(predecessor) = Self::find_largest(node.left.clone()) {
                        let val = predecessor.borrow().val;
                        node.val = val;
                        node.left = Self::delete_node(node.left.take(), val);
                    }
                }
            }
        }
        drop(node);
        Some(root)
    }
}