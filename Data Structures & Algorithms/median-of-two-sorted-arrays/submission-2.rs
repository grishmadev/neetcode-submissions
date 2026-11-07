impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let half = (total + 1) / 2;
        let (a, b) = if nums1.len() < nums2.len() {(nums1, nums2)} else {(nums2, nums1)};
        let (mut l, mut r) = (0usize, a.len());
        loop {
            let m1 = l.midpoint(r);
            let m2 = half - m1;
            let a_left = if m1 > 0 {a[m1 - 1] as f64} else {f64::NEG_INFINITY};
            let a_right = if m1 < a.len() {a[m1] as f64} else {f64::INFINITY};
            let b_left = if m2 > 0 {b[m2 - 1] as f64} else {f64::NEG_INFINITY};
            let b_right = if m2 < b.len() {b[m2] as f64} else {f64::INFINITY};

            if a_left > b_right {
                l = m1 + 1;
            } else if b_left > a_right {
                r = m1 - 1;
            } else {
                let left_max = a_left.max(b_left);
                if total % 2 != 0 {
                    return left_max;
                }
                return (left_max + a_right.min(b_right)) / 2.0;
            }   
        }
    }
}
