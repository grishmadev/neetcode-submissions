const A: u8 = b'A';
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut l = 0;
        let mut map = [0i32; 26];
        let mut maxf = 0;
        let mut max = 0;
        for (r, chr) in s.bytes().enumerate() {
            map[(chr - A) as usize] += 1;
            maxf = maxf.max(map[(chr - A) as usize]);
            let len = r - l + 1;
            if len as i32 - maxf > k {
                l += 1;
            }
            max = max.max(len);
        }

        max as i32
    }
}
