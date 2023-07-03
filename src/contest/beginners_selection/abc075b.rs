use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    }
    let dx = [1, 0, -1, 0, 1, -1, -1, 1];
    let dy = [0, 1, 0, -1, 1, 1, -1, -1];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let c = dx
                .iter()
                .zip(dy.iter())
                .filter(|(&x, &y)| {
                    let iy = i as isize + y;
                    let jx = j as isize + x;
                    iy >= 0
                        && iy < h as isize
                        && jx >= 0
                        && jx < w as isize
                        && s[iy as usize][jx as usize] == '#'
                })
                .count();
            s[i][j] = char::from_digit(c as u32, 10).unwrap();
        }
    }
    for row in s {
        println!("{}", row.into_iter().collect::<String>());
    }
}
