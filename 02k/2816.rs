use std::fmt::Write;
use std::io::{Read, stdin};

fn print(num: i32, cnt: i32) {
    for _ in 0..cnt {
        print!("{}", num);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);
    let mut idx1 = 0;
    let mut idx2 = 0;

    for i in 0..n {
        let name = input.next().unwrap().to_string();

        if name == "KBS1" {
            idx1 = i;
        }
        if name == "KBS2" {
            idx2 = i;
        }
    }

    if idx1 < idx2 {
        print(1, idx1);
        print(4, idx1);
        print(1, idx2);
        print(4, idx2 - 1);
    } else {
        print(1, idx1);
        print(4, idx1);
        print(1, idx2 + 1);
        print(4, idx2);
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
