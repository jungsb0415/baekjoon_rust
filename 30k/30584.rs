use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);
    let m = next_i32(&mut input);

    if (n+m)%2 == 1 {
        write!(output, "Error\n").unwrap();
        print!("{output}");
        return;
    }

    if n > 1 && m > 1 {
        write!(output, "Undefined\n").unwrap();
    } else if n > 1 {
        write!(output, "{}\n0\n{}\n", n/2, m).unwrap();
    } else if m > 1 {
        write!(output, "0\n{}\n{}\n", m/2, n).unwrap();
    } else if n == 1{
        write!(output, "0\n0\n1\n").unwrap();
    } else {
        write!(output, "0\n0\n0\n").unwrap();
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
