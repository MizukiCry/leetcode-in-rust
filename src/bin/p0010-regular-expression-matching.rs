impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let mut f = [[false; 25]; 35];
        f[0][0] = true;
        for i in 1..p.len() + 1 {
            for j in 0..s.len() + 1 {
                if j > 0 && (p[i - 1] == '.' || p[i - 1] == s[j - 1]) {
                    f[i][j] = f[i - 1][j - 1];
                } else if p[i - 1] == '*' {
                    if p[i - 2] == '.' {
                        for k in 0..j + 1 {
                            f[i][j] |= f[i - 2][k];
                        }
                    } else {
                        f[i][j] = f[i - 2][j];
                        let mut k = j;
                        while k > 0 && p[i - 2] == s[k - 1] {
                            f[i][j] |= f[i - 2][k - 1];
                            k -= 1;
                        }
                    }
                }
            }
        }
        f[p.len()][s.len()]
    }
}