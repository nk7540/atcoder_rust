use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let zeros = (0..n).map(|i| a[i].trailing_zeros());
    println!("{}", zeros.min().unwrap());
}
