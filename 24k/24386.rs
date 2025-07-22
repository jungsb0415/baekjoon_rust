use std::fmt::Write;
use std::io::{Read, stdin};

fn f(a: (i32, i32), b: (i32, i32)) -> i32 {
    if a.1 < b.0 || b.1 < a.0 {0} else {1}
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let v: Vec<(i32, i32)> = (0..3)
        .map(|_| (next_i32(&mut input), next_i32(&mut input))).collect();

    let max = v.iter()
        .map(|(a, b)| b-a)
        .max().unwrap();

    let min = v.iter()
        .map(|(a, b)| b-a)
        .min().unwrap();

    let tmp = f(v[0], v[1]) + f(v[1], v[2]) + f(v[2], v[0]);
    let ans = (3-tmp).max(1);

    write!(output, "{}\n{} {}", ans, min, max).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
