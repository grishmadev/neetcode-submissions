const A: u8 = b'A';
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut l = 0;
        let mut map = [0usize; 26];
        let mut maxfreq = 0;
        let mut res = 0;
        let mut cur_size = 0;
        let s = s.as_bytes();
        for r in 0..s.len() {
            let chr = s[r];
            let idx = (chr - A) as usize;
            map[idx] += 1;
            cur_size += 1;
            maxfreq = maxfreq.max(map[idx]);
            while r - l + 1 - maxfreq > k as usize {
                map[(s[l] - A) as usize] -= 1;
                l += 1;
                cur_size -= 1;
            }
            res = res.max(cur_size);
        }
        res as i32
    }
}
