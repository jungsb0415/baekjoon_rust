use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let t = next_u32(&mut input);

    for c in 1..=t {
        let n = next_u32(&mut input);
        let mut v: Vec<u32> = vec![];
        for _ in 0..n {
            let k = next_u32(&mut input);
            v.push(k);
        }

        v.sort();

        while v.len() > 1 {
            if v[v.len() - 1] == v[v.len() - 2] {
                v.pop();
                v.pop();
            } else {
                write!(output, "Case #{c}: {}\n", v[v.len() - 1]).unwrap();
                break;
            }
        }

        if v.len() == 1 {
            write!(output, "Case #{c}: {}\n", v[0]).unwrap();
        }
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
