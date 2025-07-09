use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_u32(&mut input);
    let m = next_u32(&mut input);
    let k = next_u32(&mut input);

    if n+m > k+1 {
        print!("NO");
        return;
    }

    write!(output, "YES\n").unwrap();
    for i in 0..n {
        for j in 0..m {
            write!(output, "{} ", i+j+1).unwrap();
        }
        write!(output, "\n").unwrap();
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