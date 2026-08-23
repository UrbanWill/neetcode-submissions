use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    k: usize
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut min_heap = BinaryHeap::new();

        for &num in nums.iter() {
            min_heap.push(Reverse(num));

            if min_heap.len() > k as usize {
                min_heap.pop();
            }
        }

        KthLargest {min_heap, k: k as usize}
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(Reverse(val));

        if self.min_heap.len() > self.k {
            self.min_heap.pop();
        }

        self.min_heap.peek().unwrap().0
    }
}
