use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let mut b = a.clone();
    b[0].rotate_right(1);
    b[n - 1].rotate_left(1);
    for i in 1..n {
        b[i][n - 1] = a[i - 1][n - 1];
    }
    for i in (0..n - 1) {
        b[i][0] = a[i + 1][0];
    }
    for i in 0..n {
        println!("{}", b[i].iter().collect::<String>());
    }
}
