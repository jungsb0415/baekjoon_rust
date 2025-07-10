use std::fmt::Write;
use std::io::{Read, stdin};

fn f(v: &Vec<u32>, left: usize, right: usize) -> Option<usize> {
    if (right - left) % 2 == 0 {
        return None;
    }

    let half = (right - left + 1) / 2;

    let mut left_sum = 0;
    let mut right_sum = 0;

    for i in 0..half {
        left_sum += v[left + i];
        right_sum += v[right - i];
    }

    if left_sum == right_sum {
        Some(right - left + 1)
    } else {
        None
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let v: Vec<u32> = input.next().unwrap().chars()
        .map(|x| x.to_digit(10).unwrap()).collect();

    let mut ans = 0usize;

    for i in 0..v.len() {
        for j in i+1..v.len() {
            if let Some(k) = f(&v, i, j) {
                ans = ans.max(k);
            }
        }
    }

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
