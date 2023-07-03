use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! { s: Chars }
    let s: Vec<_> = s
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let mut ans = 0;
    for b in 0..1 << (s.len() - 1) {
        let mut tmp = s[0];
        let mut sum = 0;
        for i in 0..s.len() - 1 {
            if b & (1 << i) == 0 {
                tmp = tmp * 10 + s[i + 1];
            } else {
                sum += tmp;
                tmp = s[i + 1];
            }
        }
        sum += tmp;
        ans += sum;
    }

    println!("{}", ans);
}
