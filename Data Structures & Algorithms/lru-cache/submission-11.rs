use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type NodePtr = Rc<RefCell<Node>>;
type WeakNodePtr = Weak<RefCell<Node>>;

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

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity.max(0) as usize,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let Some(node) = self.map.get(&key).cloned() else {
            return -1;
        };

        self.remove(&node);
        self.push_front(&node);

        node.borrow().value
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().value = value;

            self.remove(&node);
            self.push_front(&node);

            return;
        }

        // new node

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

        {
            let mut node_ref = node.borrow_mut();
            node_ref.prev = None;
            node_ref.next = old_head.clone();
        }

        match &old_head {
            Some(old_head) => old_head.borrow_mut().prev = Some(Rc::downgrade(node)),
            None => self.tail = Some(node.clone()),
        }

        self.head = Some(node.clone());
    }

    fn remove(&mut self, node: &NodePtr) {
        let (prev, next) = {
            let mut node_ref = node.borrow_mut();
            let prev = node_ref.prev.take();
            let next = node_ref.next.take();

            (prev, next)
        };

        let prev_node = prev.as_ref().and_then(Weak::upgrade);

        match &prev_node {
            Some(prev_node) => prev_node.borrow_mut().next = next.clone(),
            None => {
                // node was the head
                self.head = next.clone();
            }
        }

        match &next {
            Some(next_node) => {
                next_node.borrow_mut().prev = prev_node.as_ref().map(Rc::downgrade);
            }
            // Node was the tail
            None => self.tail = prev_node,
        }
    }

    fn pop_back(&mut self) -> Option<NodePtr> {
        let tail = self.tail.take()?;

        self.remove(&tail);

        Some(tail)
    }
}
