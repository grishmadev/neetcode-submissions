use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        for p in &points {
            let dist = p[0] * p[0] + p[1] * p[1];
            heap.push(Reverse((dist, p[0], p[1])));
        }
        let mut res = Vec::new();
        for _ in 0..k {
            if let Some(Reverse((_, x, y))) = heap.pop() {
                res.push(vec![x, y]);
            }
        }
        res
    }
}