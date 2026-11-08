const CAP_A: u8 = b'A';
const MAP_CAPACITY: usize = (b'z' - CAP_A) as usize + 1;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut l = 0;
        let mut have = 0;
        let mut need = 0;
        let mut last_len = usize::MAX;
        let (mut f_l, mut f_r) = (0, 0);
        let mut m1 = [0; MAP_CAPACITY as usize];
        for ch in t.bytes() {
            let idx = (ch - CAP_A) as usize;
            m1[idx] += 1;
            if m1[idx] == 1 {
                need += 1;
            }
        }
        let mut map = [0; MAP_CAPACITY as usize];
        let s = s.as_bytes();
        for r in 0..s.len() {
            let idx = (s[r] - CAP_A) as usize;
            map[idx] += 1;
            if map[idx] == m1[idx] {
                have += 1;
            }
            while have == need {
                let now_len = r - l + 1;
                if now_len < last_len {
                    f_l = l;
                    f_r = r;
                    last_len = now_len;
                }
                let lidx = (s[l] - CAP_A) as usize;
                if m1[lidx] == map[lidx] {
                    have -= 1;
                }
                map[lidx] -= 1;
                l += 1;
            }
        }
        if last_len == usize::MAX {
            String::new()
        } else {
            String::from_utf8(s[f_l..=f_r].to_vec()).unwrap()
        }
    }
}
