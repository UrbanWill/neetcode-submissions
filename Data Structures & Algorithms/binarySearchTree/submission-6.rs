struct TreeNode {
    key: i32,
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(key: i32, val: i32) -> Self {
        TreeNode {
            val,
            key,
            left: None,
            right: None,
        }
    }
}

struct TreeMap {
    root: Option<Box<TreeNode>>,
}

impl TreeMap {
    pub fn new() -> Self {
        TreeMap {root: None}
    }

    pub fn insert(&mut self, key: i32, val: i32) {
        Self::insert_helper(&mut self.root, key, val);
    }

    fn insert_helper(node: &mut Option<Box<TreeNode>>, key: i32, val: i32) {
        match node {
            None => {
                *node = Some(Box::new(TreeNode::new(key, val)));
            }
            Some(n) => {
                if key < n.key {
                    Self::insert_helper(&mut n.left, key, val);
                } else if key > n.key {
                    Self::insert_helper(&mut n.right, key, val);
                } else {
                    n.val = val;
                }
            }
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        Self::get_helper(&self.root, key)
    }

    fn get_helper(node: &Option<Box<TreeNode>>, key: i32) -> i32 {
        match node {
            None => -1,
            Some(n) => {
                if key < n.key {
                    Self::get_helper(&n.left, key)
                } else if key > n.key {
                    Self::get_helper(&n.right, key)
                } else {
                    n.val
                }
            }
        }
    }

    pub fn get_min(&self) -> i32 {
        match Self::find_min(&self.root) {
            None => -1,
            Some(n) => n.val,
        }
    }

    fn find_min(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        match node {
            None => None,
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::find_min(&n.left)
                }
            }
        }
    }

    pub fn get_max(&self) -> i32 {
        match Self::find_max(&self.root) {
            None => -1,
            Some(n) => n.val,
        }
    }

    fn find_max(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        match node {
            None => None,
            Some(n) => {
                if n.right.is_none() {
                    Some(n)
                } else {
                    Self::find_max(&n.right)
                }
            }
        }
    }

    pub fn remove(&mut self, key: i32) {
        self.root = Self::remove_helper(self.root.take(), key);
    }

    fn remove_helper(node: Option<Box<TreeNode>>, key: i32) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut n) => {
                if key < n.key {
                    n.left = Self::remove_helper(n.left.take(), key);
                    Some(n)
                } else if key > n.key {
                    n.right = Self::remove_helper(n.right.take(), key);
                    Some(n)
                } else {
                    // Found the node to remove
                    if n.left.is_none() {
                        return n.right;
                    } else if n.right.is_none() {
                        return n.left;
                    } else {
                        // Node has two children
                        let (min_key, min_val) = Self::get_min_key_val(&n.right);
                        n.key = min_key;
                        n.val = min_val;
                        n.right = Self::remove_helper(n.right.take(), min_key);
                        Some(n)
                    }
                }
            }
        }
    }

    fn get_min_key_val(node: &Option<Box<TreeNode>>) -> (i32, i32) {
        match node {
            None => panic!("Node should not be None"),
            Some(n) => {
                if n.left.is_some() {
                    Self::get_min_key_val(&n.left)
                } else {
                    return (n.key, n.val);
                }
            }
        }
    }

    pub fn get_inorder_keys(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::inorder_traversal(&self.root, &mut result);
        result
    }

    fn inorder_traversal(node: &Option<Box<TreeNode>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            Self::inorder_traversal(&n.left, result);
            result.push(n.key);
            Self::inorder_traversal(&n.right, result);
        }
    }
}
