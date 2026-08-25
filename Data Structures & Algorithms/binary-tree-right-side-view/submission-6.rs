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
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut q = VecDeque::new();

        if let Some(r) = root {
            q.push_back(r);
        }

        while !q.is_empty() {
            let mut right_side = None;

            for _ in 0..q.len() {
                let cur = q.pop_front().unwrap();
                let node = cur.borrow();

                right_side = Some(node.val);

                if let Some(l) = &node.left {
                    q.push_back(l.clone());
                }

                if let Some(r) = &node.right {
                    q.push_back(r.clone());
                }
            }

            if let Some(right_val) = right_side {
                res.push(right_val);
            }
        }

        res
    }
}
