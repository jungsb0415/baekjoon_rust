use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);

    for _ in 0..n {
        let k: u64 = input.next().unwrap().parse().unwrap();

        if k%8 != 0 {
            write!(output, "No\n").unwrap();
            continue;
        }

        let mut k = k/8;
        let mut flag = true;

        let mut v: Vec<u64> = vec![];

        while k != 0 {
            v.push(k%10);
            k /= 10;
        }

        for i in 1..v.len() {
            if v[i-1] < v[i] {
                flag = false;
            }
        }

        if v[0] > 8 {
            
            flag = false;
        }

        if flag {
            write!(output, "Yes\n").unwrap();
        } else {
            write!(output, "No\n").unwrap();
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
