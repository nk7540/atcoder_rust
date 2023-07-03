use std::cmp::min;

use proconio::input;
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        n: usize,
        t: [u32; n], // time to grill
    }
    // for each combination (2^n)
    let mut min_time = u32::max_value();
    for b in 0..1 << n {
        // for each meat i (1~n)
        let mut time1 = 0;
        let mut time2 = 0;
        for i in 0..n {
            if b & 1 << i == 0 {
                time1 += t[i];
            } else {
                time2 += t[i];
            }
        }
        min_time = min(min_time, time1.max(time2));
    }
    println!("{}", min_time);
}
