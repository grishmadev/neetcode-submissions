impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut map = [0usize; 128];
        let mut max = 0;

        for (i, ch) in s.bytes().enumerate() {
            l = l.max(map[ch as usize]);
            map[ch as usize] = i + 1;
            max = max.max(i - l + 1);
        }
        max as i32
    }
}
