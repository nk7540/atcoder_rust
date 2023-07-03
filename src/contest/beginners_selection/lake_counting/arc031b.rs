use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        a: [Chars; 10],
    }
    // hold an initial count of land points
    // flip for j for i
    // check if dfs can traverse all (the initial count + 1)
    let initial_count: u32 = a
        .iter()
        .map(|s| s.iter().filter(|&c| *c == 'o').count() as u32)
        .sum();

    for i in 0..10 {
        for j in 0..10 {
            if a[i][j] == 'o' {
                continue;
            }
            // flip and dfs
            let mut aa = a.clone();
            aa[i][j] = 'o';
            let dxy = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            let mut stack = vec![(i, j)];
            let mut visited = vec![vec![false; 10]; 10];
            let mut count = 0;
            while let Some((x, y)) = stack.pop() {
                for (dx, dy) in dxy {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if !((0..10).contains(&nx) && (0..10).contains(&ny)) {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if !visited[nx][ny] && aa[nx][ny] != 'x' {
                        stack.push((nx, ny));
                        visited[nx][ny] = true;
                        count += 1;
                    }
                }
            }
            if count == initial_count + 1 {
                println!("YES");
                return;
            }
        }
    }

    println!("NO");
}
