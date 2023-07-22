use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let dxy = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut stack = vec![(1, 1)]; // (2, 2)
    let mut passed = vec![vec![false; m]; n];
    passed[1][1] = true;
    while let Some((x, y)) = stack.pop() {
        for (dx, dy) in dxy.iter() {
            let mut nx = x + dx;
            let mut ny = y + dy;
            if s[nx as usize][ny as usize] == '#' {
                continue;
            }
            loop {
                if s[(nx + dx) as usize][(ny + dy) as usize] == '#' {
                    if !passed[nx as usize][ny as usize] {
                        stack.push((nx, ny));
                        passed[nx as usize][ny as usize] = true;
                    }
                    break;
                }
                passed[nx as usize][ny as usize] = true;
                nx += dx;
                ny += dy;
            }
        }
    }
    println!(
        "{}",
        passed
            .iter()
            .map(|x| x.iter().filter(|&&y| y).count())
            .sum::<usize>()
    );
}
