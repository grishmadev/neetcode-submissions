impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut l, mut r) = (0, 1);
        let s = s.as_bytes();
        let mut set = HashSet::<u8>::new();
        let mut max = 0;

        for r in 0..s.len() {
            let ch = s[r];
            if set.insert(ch) {
                max = max.max(set.len());
            } else {
                while set.contains(&ch) {
                    set.remove(&s[l]);
                    l += 1;
                }
            }
        }
        max as i32
    }
}
