struct MyLinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            head: None,
            size: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index as usize >= self.size {
            return -1;
        }
        let mut cur = &self.head;
        for _ in 0..index {
            cur = &cur.as_ref().unwrap().next;
        }
        cur.as_ref().unwrap().val
    }

    fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val);
    }

    fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.size as i32, val);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index as usize > self.size {
            return;
        }

        let mut cur = &mut self.head;

        for _ in 0..index {
            cur = &mut cur.as_mut().unwrap().next;
        }

        let new_node = Box::new(Node {
            val: val,
            next: (*cur).take(),
        });

        *cur = Some(new_node);

        self.size += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        if index as usize >= self.size {
            return;
        }

        let mut cur = &mut self.head;

        for _ in 0..index {
            cur = &mut cur.as_mut().unwrap().next;
        }

        let removed = cur.take().unwrap();
        *cur = removed.next;

        self.size -= 1;
    }
}
