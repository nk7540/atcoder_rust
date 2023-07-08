#![feature(test)]
extern crate cli_test_dir;
extern crate test;

use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use cli_test_dir::{CommandExt, ExpectStatus, OutputExt, TestDir};
    use rand::Rng;
    use test::Bencher;

    #[test]
    fn test_solve() {
        let cases = [("", "")];

        let testdir = TestDir::new("", "");
        for (i, (stdin, stdout)) in cases.iter().enumerate() {
            let output = testdir.cmd().output_with_stdin(stdin).expect_success();
            assert_eq!(
                output.stdout_str(),
                format!("{}\n", stdout),
                "test case {}",
                i + 1
            );
            assert!(output.stderr_str().is_empty(), "test case {}", i + 1);
        }
    }

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let testdir = TestDir::new("", "");
        let mut cmd = testdir.cmd();
        b.iter(|| cmd.output_with_stdin());
    }
}
