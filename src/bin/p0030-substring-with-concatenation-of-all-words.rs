use std::{collections::HashMap, cell::RefCell, rc::Rc};

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        const HASH_BASE: i64 = 131;
        const HASH_MOD: i64 = 998244353;

        let s = s.as_bytes();
        let mut s_hash: Vec<i64> = Vec::with_capacity(s.len());
        for i in 0..s.len() {
            s_hash.push((s[i] as i64 + if i == 0 { 0 } else { s_hash[i - 1] as i64 } * HASH_BASE) % HASH_MOD);
        }

        let word_len = words.first().unwrap().len();
        let total_len = word_len * words.len();
        let mut words_hash: HashMap<i64, i32> = HashMap::with_capacity(words.len());
        for w in words {
            let mut hash_val = 0;
            for c in w.bytes() {
                hash_val = (hash_val * HASH_BASE + c as i64) % HASH_MOD;
            }
            words_hash.entry(hash_val).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut hash_pow: i64 = 1;
        for _ in 0..word_len {
            hash_pow = hash_pow * HASH_BASE % HASH_MOD;
        }

        let unmatch_num = Rc::new(RefCell::new(words_hash.len()));
        let mut modify = |hash_val: i64, num: i32| {
            let t = unmatch_num.clone();
            if let Some(i) = words_hash.get_mut(&hash_val) {
                if *i == 0 {
                    *t.borrow_mut() += 1;
                }
                *i += num;
                if *i == 0 {
                    *t.borrow_mut() -= 1;
                }
            }
        };

        let calc_hash = |pos: usize| -> i64 {
            (s_hash[pos + word_len - 1] - (if pos == 0 { 0 } else { s_hash[pos - 1] }) * hash_pow % HASH_MOD + HASH_MOD) % HASH_MOD
        };

        let mut ans: Vec<i32> = vec![];

        for i in 0..word_len {
            let mut j = i;
            while j <= s.len() + total_len - word_len {
                if j + word_len <= s.len() {
                    modify(calc_hash(j), -1);
                }
                if j >= total_len {
                    modify(calc_hash(j - total_len), 1);
                }
                if *unmatch_num.borrow() == 0 {
                    ans.push((j - total_len + word_len) as i32);
                }
                j += word_len;
            }
        }

        ans
    }
}