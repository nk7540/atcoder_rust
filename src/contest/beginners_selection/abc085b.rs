use proconio::input;
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        n: usize,
        mut d: [usize; n],
    }
    d.sort_unstable();
    d.dedup();
    println!("{}", d.len());
}
