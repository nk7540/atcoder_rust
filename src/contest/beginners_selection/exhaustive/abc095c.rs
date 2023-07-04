use std::cmp::min;

use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
        y: u32,
    }
    // price of one A and one B
    let separate_price = a + b;
    let half_half_price = c * 2;
    // price of one A

    let ans = if separate_price <= half_half_price {
        a * x + b * y
    } else {
        // buy min(x, y) half-halfs
        // if a > half_half_price, buy
        if x <= y {
            half_half_price * x + min(b, half_half_price) * (y - x)
        } else {
            half_half_price * y + min(a, half_half_price) * (x - y)
        }
    };

    println!("{}", ans);
}
