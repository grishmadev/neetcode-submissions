struct MedianFinder {
    small: BinaryHeap<i32>,
    big: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            small: BinaryHeap::new(),
            big: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.small.push(num);
        if let Some(max_small) = self.small.pop() {
            self.big.push(Reverse(max_small));
        }
        if self.small.len() < self.big.len() {
            self.small.push(self.big.pop().unwrap().0);
        }
    }

    pub fn find_median(&self) -> f64 {
        let sml = self.small.len();
        let bgl = self.big.len();
        if bgl > sml {
            self.big.peek().unwrap().0 as f64
        } else if bgl < sml {
            *self.small.peek().unwrap() as f64
        } else {
            let val = self.small.peek().unwrap() + self.big.peek().unwrap().0;
            val as f64 / 2.0
        }
    }
}
