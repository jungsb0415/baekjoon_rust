use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let one = next_i32(&mut input);
    let five = next_i32(&mut input);
    let ten = next_i32(&mut input);
    let twenty = next_i32(&mut input);
    let fifty = next_i32(&mut input);
    let hundred = next_i32(&mut input);

    let mut v = vec![one, five * 5, ten * 10, twenty * 20, fifty * 50, hundred * 100];

    v.sort();

    let max = v[5];

    let ans = if max == hundred * 100 {
        100
    } else if max == fifty * 50 {
        50
    } else if max == twenty * 20 {
        20
    } else if max == ten * 10 {
        10
    } else if max == five * 5 {
        5
    } else {
        1
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
