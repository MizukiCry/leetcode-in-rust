use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (index, value) in nums.iter().enumerate() {
            if let Some(&p) = map.get(&(target - value)) {
                return vec![p, index as i32];
            }
            map.insert(value, index as i32);
        }
        vec![]
    }
}

/* --- */

struct Solution {}

fn main() {
    assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0, 1]);
}