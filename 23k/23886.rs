use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let v: Vec<i32> = input.next().unwrap().chars()
        .map(|x| ((x as u32) - ('0' as u32)) as i32).collect();

    let mut d = v[1] - v[0];

    let mut p = d > 0;

    for i in 2..v.len() {
        let next_d = v[i] - v[i-1];

        if d * next_d < 0 {
            d = next_d;
            continue;
        }
        if d != next_d || d == 0 {
            p = false;
            break;
        }
    }

    if d > 0 {
        p = false;
    }

    let ans = if p {"ALPSOO"} else {"NON ALPSOO"};

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}