impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, k as usize - 1);
        let mut res = Vec::with_capacity(nums.len() - k as usize + 1);
        let mut q = VecDeque::new();
        for r in 0..nums.len() {
            while let Some(&idx) = q.back() {
                if nums[idx] < nums[r] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(r);
            
            if l > *q.front().unwrap() {
                q.pop_front();
            }

            if r + 1 >= k as usize {
                res.push(nums[*q.front().unwrap()]);
            }
            
            
        }
        println!("deque: {q:?}");
        res
    }
}
