use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

/*
// Definition for a Node.
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}
*/

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut old_to_new = HashMap::new();

        Self::dfs(node, &mut old_to_new)
    }

    fn dfs(node: Option<Rc<RefCell<Node>>>, old_to_new: &mut HashMap<i32, Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let node = node?;
        let node_val = node.borrow().val;

        match old_to_new.get_mut(&node_val) {
            Some(existing) => {
                return Some(existing.clone());
            },
            None => {
                let copy = Rc::new(RefCell::new(Node::new(node_val)));
                old_to_new.insert(node_val, copy.clone());

                for n in node.borrow().neighbors.iter() {
                    if let Some(existing) = Self::dfs(Some(n.clone()), old_to_new) {
                        copy.borrow_mut().neighbors.push(existing);
                    }
                }

                Some(copy)
            }
        }
    }
}
