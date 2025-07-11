use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    loop {
        let n: u32 = next_u32(&mut input);
        if n == 0 {
            break;
        }
        let mut a = 0;
        let mut b  = 0;
        for _ in 0..n {
            let k: u32 = next_u32(&mut input);
            if k%2 == 1 {
                a += 1;
            }
        }
        for _ in 0..n {
            let k: u32 = next_u32(&mut input);
            if k%2 == 1 {
                b += 1;
            }
        }

        let ans = (n-a).abs_diff(b);
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
