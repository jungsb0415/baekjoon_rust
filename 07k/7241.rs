use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let a = next_i32(&mut input);
    let b = next_i32(&mut input);
    let c = next_i32(&mut input);

    let mut v = vec![a, b, c];
    v.sort();

    let n = next_i32(&mut input);

    let ans = if n/100 == v[0] {
        if n%10 == v[2] {
            1
        } else {
            2
        }
    } else if n/100 == v[1] {
        if n%10 == v[2] {
            3
        } else {
            4
        }
    } else {
        if n%10 == v[1] {
            5
        } else {
            6
        }
    };

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
