use itertools::Itertools;
use proconio::input;
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        n: usize, // the number of members
        m: usize, // the number of relationships
        xy: [(u32, u32); m], // relationships
    }
    let mut max_members = 0;
    // search all possible combinations
    // check if the combination is possible (all two-fold relationships are included in xy)
    // find the maximum possible combination
    for b in 0..1 << n {
        let party: Vec<_> = (0..n)
            .enumerate()
            .filter(|(_, ni)| b & 1 << ni != 0)
            .map(|(i, _)| i as u32)
            .collect();
        let count = party.clone().len();
        if b == 120 {
            dbg!();
        }
        let can_form = party
            .iter()
            .combinations(2)
            .all(|r| xy.contains(&(*r[0] + 1, *r[1] + 1)));
        if can_form {
            max_members = max_members.max(count);
        }
    }
    println!("{}", max_members);
}
