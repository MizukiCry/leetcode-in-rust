impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let [mut left, mut right] = [0, height.len() - 1];
        let [mut max_left, mut max_right, mut ans] = [0, 0, 0];
        while left < right {
            if max_left < max_right {
                ans += 0.max(max_left - height[left]);
                max_left = max_left.max(height[left]);
                left += 1;
            } else {
                ans += 0.max(max_right - height[right]);
                max_right = max_right.max(height[right]);
                right -= 1;
            }
        }
        ans + 0.max(max_left.min(max_right) - height[left])
    }
}