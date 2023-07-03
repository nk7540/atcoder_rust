use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    println!("{}", s.iter().filter(|&ss| *ss == '1').count());
}
