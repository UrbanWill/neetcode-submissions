use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type NodePtr = Rc<RefCell<Node>>;
type WeakNodePtr = Weak<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    prev: Option<WeakNodePtr>,
    next: Option<NodePtr>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, NodePtr>,
    head: Option<NodePtr>,
    tail: Option<NodePtr>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let Some(node) = self.map.get(&key).cloned() else {
            return -1;
        };

        self.remove(&node);
        self.push_front(&node);

        node.borrow().value
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        // check if value exists and update it.
        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().value = value;

            self.remove(&node);
            self.push_front(&node);

            return;
        }

        // value does not exist, insert it.
        let node = Rc::new(RefCell::new(Node::new(key, value)));

        self.push_front(&node);
        self.map.insert(key, node);

        if self.map.len() > self.capacity {
            if let Some(lru) = self.pop_back() {
                let key = lru.borrow().key;

                self.map.remove(&key);
            }
        }
    }
    fn push_front(&mut self, node: &NodePtr) {
        let old_head = self.head.take();

        // update new node.
        {
            let mut node_ref = node.borrow_mut();
            node_ref.prev = None;
            node_ref.next = old_head.clone();
        }

        // update old head.
        match &old_head {
            Some(head) => {
                head.borrow_mut().prev = Some(Rc::downgrade(node));
            }
            None => {
                // Empty list, node is also the tail
                self.tail = Some(Rc::clone(node));
            }
        }

        self.head = Some(Rc::clone(node));
    }

    fn remove(&mut self, node: &NodePtr) {
        // Completely detach the node first.
        let (prev, next) = {
            let mut node_ref = node.borrow_mut();

            let prev = node_ref.prev.take();
            let next = node_ref.next.take();

            (prev, next)
        };

        let prev_node = prev.as_ref().and_then(Weak::upgrade);

        // Connect previous node to next node.
        match &prev_node {
            Some(prev_node) => {
                prev_node.borrow_mut().next = next.clone();
            }
            None => {
                // No prev node, must be the head.
                self.head = next.clone()
            }
        }

        match &next {
            Some(next_node) => {
                next_node.borrow_mut().prev = prev_node.as_ref().map(Rc::downgrade);
            }
            None => {
                // No next node, but be the tail.
                self.tail = prev_node;
            }
        }
    }

    fn pop_back(&mut self) -> Option<NodePtr> {
        let tail = self.tail.clone()?;

        self.remove(&tail);

        Some(tail)
    }
}
