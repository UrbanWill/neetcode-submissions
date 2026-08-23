use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let mut res = Vec::new();

        for p in points {
            let dist = p[0] * p[0] + p[1] * p[1];
            heap.push(Reverse((dist, p)));
        }

        for _ in 0..k {
            if let Some(Reverse((_, p))) = heap.pop() {
                res.push(p);
            }
        }

        res
    }
}
