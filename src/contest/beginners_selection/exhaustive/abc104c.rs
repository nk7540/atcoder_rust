use std::cmp::min;

use proconio::input;
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        d: usize, // dimension
        g: u32, // goal
        pc: [(u32, u32); d] // (number of 100i-problems, bonus)
    }
    let mut res = u32::max_value();
    // for each combination
    for b in 0..1 << d {
        let mut trials = 0;
        let mut sum = 0;
        let mut largest_unsummed = usize::max_value();
        for i in 0..d {
            let (p, c) = pc[i];
            if b & 1 << i != 0 {
                trials += p;
                sum += p * 100 * (i + 1) as u32 + c;
            } else {
                largest_unsummed = i;
            }
        }
        // it is assumed this case (111..) achieves the goal
        if !(largest_unsummed < d) || sum >= g {
            res = min(res, trials);
            continue;
        }

        // when the goal is reached, compare/update minimum trial count
        let (p, _) = pc[largest_unsummed];
        let largest_unsummed_point = 100 * (largest_unsummed + 1) as u32;
        let additional_trials_needed =
            (g - sum + (largest_unsummed_point - 1)) / largest_unsummed_point;
        if additional_trials_needed <= (p - 1) {
            trials += additional_trials_needed;
            res = min(res, trials);
        }
    }
    println!("{}", res);
}
