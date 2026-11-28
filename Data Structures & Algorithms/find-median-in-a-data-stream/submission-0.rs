struct MedianFinder {
    heap: BinaryHeap<i32>,
    size: usize,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            size: 0,
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.heap.push(num);
        self.size += 1;
    }

    pub fn find_median(&self) -> f64 {
        let vals = self.heap.clone().into_sorted_vec();
        let half = self.size / 2;
        if self.size == 0 {
            0.0
        } else if self.size % 2 == 0 {
            (vals[half - 1] + vals[half]) as f64 / 2.0
        } else {
            vals[half] as f64
        }
    }
}