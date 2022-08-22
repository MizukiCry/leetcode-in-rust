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
