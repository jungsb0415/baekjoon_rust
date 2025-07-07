use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let v: Vec<char> = input.next().unwrap().chars().collect();

    let mut sum = 0;
    let mut idx = 0;

    for i in 0..v.len() {
        let c: char = v[i];
        if c != '*' {
            sum += (c as usize - '0' as usize) * (2 * (i%2) + 1);
        } else {
            idx = i;
        }
    }

    for i in 0..10 {
        if (sum + (2 * (idx%2) + 1) * i) % 10 == 0 {
            write!(output, "{}\n", i).unwrap();
            break;
        }
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
