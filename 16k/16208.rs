use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_u32(&mut input) as usize;

    let mut v: Vec<u32> = (0..n)
        .map(|_| next_u32(&mut input)).collect();

    v.sort();

    let mut ans: u64 = 0;
    let mut sum: u32 = v.iter().sum(); 
    sum -= v[0];

    for i in 0..n-1 {
        ans += v[i] as u64 * sum as u64;
        sum -= v[i+1];
    }

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
