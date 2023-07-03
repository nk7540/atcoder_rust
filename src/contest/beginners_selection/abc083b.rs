use proconio::input;
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let ans = (1..n + 1)
        .filter(|x| {
            let digit_sum = x
                .to_string()
                .chars()
                .map(|c| (c as u8 - b'0') as usize)
                .sum::<usize>();
            a <= digit_sum && digit_sum <= b
        })
        .sum::<usize>();
    println!("{}", ans);
}
