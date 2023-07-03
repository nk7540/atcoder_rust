use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]

fn main() {
    let patterns: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"]
        .iter()
        .map(|s| s.chars().rev().collect())
        .collect();
    input! { s: Chars }
    let s: Vec<char> = s.into_iter().rev().collect();
    let mut s = &s[..];

    let mut res = true;
    while !s.is_empty() {
        let matched = patterns.iter().find(|&p| s.starts_with(p));
        if let Some(p) = matched {
            s = &s[p.len()..];
        } else {
            res = false;
            break;
        }
    }
    println!("{}", if res { "YES" } else { "NO" });
}
