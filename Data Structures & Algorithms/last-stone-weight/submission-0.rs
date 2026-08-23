use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();

        for s in stones {
            heap.push(s);
        }

        while heap.len() > 1 {
            let x = heap.pop().unwrap();
            let y = heap.pop().unwrap();

            if x > y {
                heap.push(x - y);
            } else {
                heap.push(y - x);
            }
                
        }

        *heap.peek().unwrap()
    }
}
