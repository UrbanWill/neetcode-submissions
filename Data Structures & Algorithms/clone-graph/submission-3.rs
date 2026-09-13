use std::collections::HashMap;

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut cloned_map = HashMap::new();

        Self::dfs(node, &mut cloned_map)
    }

    fn dfs(node: Option<Rc<RefCell<Node>>>, cloned_map: &mut HashMap<i32, Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let node = node?;
        let val = node.borrow().val;

        match cloned_map.get(&val) {
            Some(cloned) => Some(cloned.clone()),
            None => {
                let copy = Rc::new(RefCell::new(Node::new(val)));
                cloned_map.insert(val, copy.clone());

                for n in node.borrow().neighbors.iter() {
                    if let Some(cloned) = Self::dfs(Some(n.clone()), cloned_map) {
                        copy.borrow_mut().neighbors.push(cloned.clone());
                    }
                }
                Some(copy)
            }
        }
    }
}