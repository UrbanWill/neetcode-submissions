use std::collections::VecDeque;

struct MyStack {
    stack: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack {
            stack: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.stack.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.stack.pop_back().unwrap()
    }

    pub fn top(&self) -> i32 {
        self.stack[self.stack.len() -1]
    }

    pub fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}
