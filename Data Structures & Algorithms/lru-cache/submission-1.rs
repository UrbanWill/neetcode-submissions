use std::cell::RefCell;
use std::rc::{Rc, Weak};

type NodePtr = Rc<RefCell<Node>>;
type WeakNodePtr = Weak<RefCell<Node>>;





struct Node {
    key: i32,
    value: i32,
    prev: Option<WeakNodePtr>,
    next: Option<NodePtr>
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

        let value = node.borrow().value;

        self.remove(&node);
        self.push_front(&node);

        value
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        // Existing key: update it and mark it MRU
        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().value = value; 

            self.remove(&node);
            self.push_front(&node);

            return;
        }

        let node = Rc::new(RefCell::new(Node::new(key, value)));
        
        self.push_front(&node);
        self.map.insert(key, node);

        // Evit the least recenty used node

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
            let mut ref_node = node.borrow_mut();

            ref_node.prev = None;
            ref_node.next = old_head.clone();
        }

        match old_head {
            Some(head) => {
                head.borrow_mut().prev = Some(Rc::downgrade(node));
            },
            None => {
                self.tail = Some(Rc::clone(node));
            }
        }

        self.head = Some(Rc::clone(node));
    }

    fn remove(&mut self, node: &NodePtr) {
        let (prev, next) = {
            let mut ref_node = node.borrow_mut();

            let prev = ref_node.prev.take();
            let next = ref_node.next.take();

            (prev, next)
        };

        let prev_node = prev.as_ref().and_then(Weak::upgrade);

        match &prev_node {
            Some(prev_node) => {
                prev_node.borrow_mut().next = next.clone();
            },
            None => {
                self.head = next.clone();
            }
        }

        match &next {
            Some(next) => {
                next.borrow_mut().prev = prev_node.as_ref().map(Rc::downgrade);
            },
            None => {
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
