use proconio::input;
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        n: usize,
        y: usize,
    }
    for i in 0..n + 1 {
        for j in 0..n - i + 1 {
            let k = n - i - j;
            if i * 10000 + j * 5000 + k * 1000 == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}
