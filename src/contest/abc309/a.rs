use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let ans = if b - a == 1 && a != 3 && a != 6 {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
