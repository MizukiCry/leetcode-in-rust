fn dfs(
    n: usize,
    x: usize,
    ans: &mut Vec<Vec<String>>,
    res: &mut Vec<usize>,
    v1: &mut Vec<bool>,
    v2: &mut Vec<bool>,
    v3: &mut Vec<bool>,
) {
    if x == n {
        let mut tmp: Vec<String> = vec![];
        for i in 0..n {
            let mut s = vec![b'.'; n];
            s[res[i]] = b'Q';
            tmp.push(String::from_utf8(s).unwrap());
        }
        ans.push(tmp);
        return;
    }
    for i in 0..n {
        if !v1[i] && !v2[x + i] && !v3[x - i + n - 1] {
            v1[i] = true;
            v2[x + i] = true;
            v3[x - i + n - 1] = true;
            res[x] = i;
            dfs(n, x + 1, ans, res, v1, v2, v3);
            v1[i] = false;
            v2[x + i] = false;
            v3[x - i + n - 1] = false;
        }
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut ans: Vec<Vec<String>> = vec![];
        let mut res = vec![0_usize; n];
        let mut v1 = vec![false; n];
        let mut v2 = vec![false; n * 2 - 1];
        let mut v3 = vec![false; n * 2 - 1];
        dfs(n, 0, &mut ans, &mut res, &mut v1, &mut v2, &mut v3);
        ans
    }
}