use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);
    let r = next_i32(&mut input);

    let mut one = 0;
    let mut two = 0;

    for _ in 0..n {
        let k = next_i32(&mut input);
        if k < r {
            one += 1;
        } else if k > r {
            two += 1;
        }
    }

    let ans = if one > two {
        1
    } else if two > one {
        2
    } else {
        0
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
