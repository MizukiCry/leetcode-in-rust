impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            if nums[i] < 0 || nums[i] > n as i32 {
                nums[i] = 0;
            }
        }
        for i in 0..n {
            let t = nums[i] as usize & 0xfffff;
            if t != 0 {
                nums[t - 1] |= 1 << 20;
            }
        }
        for i in 0..n {
            if nums[i] >> 20 == 0 {
                return i as i32 + 1;
            }
        }
        n as i32 + 1
    }
}