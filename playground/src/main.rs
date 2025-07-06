use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let a: i32 = input.next().unwrap().parse().unwrap();
    let b: i32 = input.next().unwrap().parse().unwrap();

    let ans = a+b;
    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}