use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut pre: [i32; 128] = [-1; 128];
        let mut left: i32 = -1;
        let mut ans: i32 = 0;
        for (i, v) in s.bytes().enumerate() {
            left = max(left, pre[v as usize]);
            ans = max(ans, (i as i32) - left);
            pre[v as usize] = i as i32;
        }
        ans
    }
}