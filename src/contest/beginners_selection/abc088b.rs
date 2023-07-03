use proconio::input;
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort_unstable_by(|x, y| y.cmp(x));
    let ans = a.iter().enumerate().fold(
        0,
        |acc, (i, cur)| {
            if i % 2 == 0 {
                acc + cur
            } else {
                acc - cur
            }
        },
    );
    println!("{}", ans);
}
