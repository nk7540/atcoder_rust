use proconio::input;
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        n: usize, // next #
        m: usize, // edge #
        e: [(usize, usize); m],
    }
    // start from 1
    // dfs, add count
    // don't add count when tree detected (edge to already-visited)
    let mut ans = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        visited[i] = true;

        // dfs from i
        let mut stack = vec![(None, i)];
        let mut is_tree = true;
        while let Some((prev, j)) = stack.pop() {
            // search reachable edges
            for &(s, t) in e.iter() {
                let s = s - 1;
                let t = t - 1;
                let mut search = |next: usize| {
                    if visited[next] {
                        // if next is visited and not prev, it forms a cycle
                        if let Some(p) = prev {
                            if p != next {
                                is_tree = false;
                            }
                        }
                        return;
                    }
                    visited[next] = true;
                    stack.push((Some(j), next));
                };
                if s == j {
                    search(t);
                } else if t == j {
                    search(s);
                }
            }
        }

        if is_tree {
            ans += 1;
        }
    }

    println!("{}", ans);
}
