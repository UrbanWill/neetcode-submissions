use std::cmp::min;
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: vec![],
            min_stack: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() {
            self.min_stack.push(val);
        } else {
            let new_min = min(self.min_stack[self.min_stack.len() - 1], val);
            self.min_stack.push(new_min);
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    pub fn get_min(&self) -> i32 {
        if self.min_stack.len() > 0 {
            self.min_stack[self.min_stack.len() - 1]
        } else {
            123
        }
    }
}
