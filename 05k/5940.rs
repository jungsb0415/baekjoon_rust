use std::fmt::Write;
use std::io::{Read, stdin};

fn first_digit(mut a: i64) -> i32 {
    while a >= 10 {
        a /= 10;
    }
    a as i32
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let a = next_i32(&mut input);
    let b = next_i32(&mut input);

    let mut a = a as u32;

    while a < 63 {
        a += 1;
        if first_digit(2i64.pow(a)) == b {
            break;
        }
    }

    let ans = if a == 63 {
        0
    } else {
        a
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
