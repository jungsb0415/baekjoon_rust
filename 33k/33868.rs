use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);

    let mut min = 1<<30;
    let mut max = 0;

    for _ in 0..n {
        let a = next_i32(&mut input);
        let b = next_i32(&mut input);

        max = max.max(a);
        min = min.min(b);
    }

    let ans = (min * max) % 7 + 1;

    write!(output, "{}\n", ans).unwrap();

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
