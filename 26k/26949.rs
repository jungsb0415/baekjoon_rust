use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let pa = next_i32(&mut input);
    let ka = next_i32(&mut input);
    let pb = next_i32(&mut input);
    let kb = next_i32(&mut input);
    let n = next_i32(&mut input);

    let mut cnt_a = -1;
    let mut cnt_b = -1;
    let mut cost = 1<<30;
    
    for a in 0..=n {
        let remain = n - ka * a;
        if remain < 0 {
            break;
        }
        let b = (remain + kb - 1) / kb;
        let tmp = a * pa + b * pb;
        if tmp < cost {
            cost = tmp;
            cnt_a = a;
            cnt_b = b;
        }
    }
    
    write!(output, "{} {} {}\n", cnt_a, cnt_b, cost).unwrap();

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
