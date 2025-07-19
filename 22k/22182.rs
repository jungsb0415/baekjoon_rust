use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    
    let n = next_i32(&mut input);
    let r = next_i32(&mut input) as f64;

    let v: Vec<i32> = (0..n)
        .map(|_| next_i32(&mut input)).collect();

    let tot: i32 = v.iter().sum();

    let pi = 3.14159265358979;

    for vi in v {
        let ans = pi * r * r * vi as f64 / tot as f64;
        write!(output, "{}\n", ans).unwrap();
    }


    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
