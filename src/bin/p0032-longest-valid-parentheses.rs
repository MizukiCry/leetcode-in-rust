impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut ans = 0;
        let mut base = 0;
        let mut stack: Vec<usize> = Vec::new();
        for (i, v) in s.iter().enumerate() {
            if *v == '(' {
                stack.push(i + 1);
            } else {
                if stack.is_empty() {
                    base = i + 1;
                } else {
                    stack.pop();
                }
                ans = ans.max(i - stack.last().unwrap_or(&base) + 1);
            }
        }
        ans as i32
    }
}