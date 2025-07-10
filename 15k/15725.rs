use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let str = input.next().unwrap();

    if str.contains('x') {
        let mut idx = 0;
        if str.chars().nth(0).unwrap() == 'x' {
            write!(output, "1\n").unwrap();
        } else if str.len() > 1 && str.chars().nth(0).unwrap() == '-' && str.chars().nth(1).unwrap() == 'x' { 
            write!(output, "-1\n").unwrap();
        } else {
            while str.chars().nth(idx).unwrap() != 'x' {
                idx += 1;
            }
            let num: &i32 = &str[0..idx].parse().unwrap();
            write!(output, "{}\n", num).unwrap();
        }
    } else {
        write!(output, "0\n").unwrap();
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
