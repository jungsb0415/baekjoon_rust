use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);
    let d = next_i32(&mut input);

    if n%2 == 0 {
        let a = d + 1;
        let b = d - 1;

        for i in 0..n/2 {
            write!(output, "{} ", a + i*2).unwrap();
            write!(output, "{} ", b - i*2).unwrap();
        }
    }
    else {
        let a = d + 2;
        let b = d - 2;

        for i in 0..n/2 {
            write!(output, "{} ", a + i*2).unwrap();
            write!(output, "{} ", b - i*2).unwrap();
        }
        write!(output, "{} ", d).unwrap();
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
