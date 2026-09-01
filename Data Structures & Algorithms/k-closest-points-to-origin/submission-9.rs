use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut min_heap = BinaryHeap::new();

        for p in points {
            let dist = p[0] * p[0] + p[1] * p[1];
            min_heap.push(Reverse((dist, p)));
        }

        for _ in 0..k {
            if let Some(Reverse((_, p))) = min_heap.pop() {
                res.push(p);
            }
        }

        res
    }
}
