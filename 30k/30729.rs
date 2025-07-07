use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let mut n: i64 = input.next().unwrap().parse().unwrap();
    let mut m: i64 = input.next().unwrap().parse().unwrap();

    if n < m {
        (n, m) = (m, n);
    }

    let ans = if n == m {
        n * n
    } else if n > 2*m {
        2 * m * m
    } else {
        m * m + (n - m) * (n - m)
    };


    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
