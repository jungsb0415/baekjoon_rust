use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();

    let mut v: Vec<usize> = vec![0; n + m + 1];
    let mut max = 0;

    for i in 1..=n {
        for j in 1..=m {
            v[i + j] += 1;
            max = max.max(v[i + j]);
        }
    }

    for k in 1..=n + m {
        if v[k] == max {
            write!(output, "{k}\n").unwrap();
        }
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
