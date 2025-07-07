use std::fmt::Write;
use std::io::{Read, stdin};

fn f(mut num: usize) -> bool {
    let mut last = num % 10;
    while num != 0 {
        num /= 10;
        if num % 10 > last {
            return false;
        }
        last = num % 10;
    }
    true
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();

    for i in 1..=n {
        let mut k: usize = input.next().unwrap().parse().unwrap();

        while !f(k) {
            k -= 1;
        }
        write!(output, "Case #{i}: {}\n", k).unwrap();
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
