impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let s = s.as_bytes();
        let mut map = HashMap::new();
        let mut max = 0;

        for r in 0..s.len() {
            if let Some(&idx) = map.get(&s[r]) {
                l = l.max(idx + 1);
            }
            map.insert(s[r], r);
            max = max.max(r - l + 1);
        }
        max as i32
    }
}
