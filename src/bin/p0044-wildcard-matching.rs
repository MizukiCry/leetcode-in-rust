impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut sitr = s.chars().enumerate().skip(0).peekable();
        let mut pitr = p.chars().enumerate().skip(0).peekable();
        let mut star = false;
        let (mut ss, mut ps) = (0, 0);

        while sitr.peek().is_some() {
            match pitr.peek() {
                Some(&(_, '?')) => {
                    sitr.next();
                    pitr.next();
                },
                Some(&(_, '*')) => {
                    star = true;
                    while let Some(&(_, '*')) = pitr.peek() {
                        pitr.next();
                    }
                    if pitr.peek().is_none() {
                        return true;
                    }
                    ss = sitr.peek().unwrap().0;
                    ps = pitr.peek().unwrap().0;
                },
                _ => {
                    if pitr.peek().is_none() || sitr.peek().unwrap().1 != pitr.peek().unwrap().1 {
                        if !star {
                            return false;
                        }
                        ss += 1;
                        sitr = s.chars().enumerate().skip(ss).peekable();
                        pitr = p.chars().enumerate().skip(ps).peekable();
                    } else {
                        sitr.next();
                        pitr.next();
                    }
                }
            }
        }

        while let Some(&(_, '*')) = pitr.peek() {
            pitr.next();
        }
        pitr.peek().is_none()
    }
}