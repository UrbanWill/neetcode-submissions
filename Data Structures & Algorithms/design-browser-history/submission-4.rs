use std::cmp::{max, min};
struct BrowserHistory {
    history: Vec<String>,
    cur: usize,
    len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            history: vec![homepage],
            cur: 0,
            len: 1,
        }
    }

    fn visit(&mut self, url: String) {
        if self.len <= self.cur + 1 {
            self.history.push(url);
        } else {
            self.history[self.cur + 1] = url;
        }

        self.cur += 1;
        self.len = self.cur + 1;
    }

    fn back(&mut self, steps: i32) -> String {
        self.cur = max(self.cur as i32 - steps, 0) as usize;

        self.history[self.cur].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.cur = min(self.cur as i32 + steps, self.len as i32 - 1) as usize;

        self.history[self.cur].clone()
    }
}
