use std::cmp::min;

use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        t: usize,
        cases: [(usize, Chars); t],
    }
    // for each test case:
    for (_, s) in cases {
        let mut subs = vec![vec![s[0]]]; // set of substrings
        let mut res = false;
        for i in 1..s.len() {
            if (is_ordered(&s[..i], &s[i..])) {
                println!("Yes");
                res = true;
                break;
            }
        }
        if !res {
            println!("No")
        }
    }
}

fn is_ordered(sub0: &[char], sub1: &[char]) -> bool {
    let len0 = sub0.len();
    let len1 = sub1.len();
    len0 < len1 && sub0[..len0] == sub1[..len0]
        || (1..min(len0, len1) + 1)
            .any(|i| (i == 1 || sub0[..i - 1] == sub1[..i - 1]) && sub0[i - 1] < sub1[i - 1])
}
