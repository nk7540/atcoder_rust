use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! { ss: Chars }
    let s: Vec<_> = ss.iter().map(|c| c.to_digit(10).unwrap() as i32).collect();

    // for each combination
    for b in 0..1 << (s.len() - 1) {
        let mut expr = ss[0].to_string();
        let mut res = s[0];
        for i in 0..s.len() - 1 {
            if b & 1 << i == 0 {
                expr.push('+');
                res += s[i + 1];
            } else {
                expr.push('-');
                res -= s[i + 1];
            }
            expr.push(ss[i + 1]);
        }
        if res == 7 {
            println!("{}=7", expr);
            break;
        }
    }
}
