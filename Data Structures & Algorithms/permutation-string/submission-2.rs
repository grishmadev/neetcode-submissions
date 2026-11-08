const A: u8 = b'a';
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut m1 = [0; 26];
        let mut m2 = [0; 26];
        let mut l = 0;
        for ch in s1.bytes() {
            m1[(ch - A) as usize] += 1;
        }
        let s2 = s2.as_bytes();
        for r in 0..s2.len() {
            let chr = s2[r];
            m2[(chr - A) as usize] += 1;
            while r - l >= s1.len() {
                m2[(s2[l] - A) as usize] -= 1;
                l += 1;
            }
            if m1 == m2 {
                return true;
            }
        }
        false
    }
}
