use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

fn main() {
    input! {
        d: usize, // total length of ring road
        n: usize, // number of stores
        m: usize, // number of orders
        stores: [usize; n - 1],
        recipients: [usize; m],
    }
}
