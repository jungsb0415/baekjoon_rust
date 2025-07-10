use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_u32(&mut input);
    let m = next_u32(&mut input);

    let v: Vec<char> = input.next().unwrap().chars().collect();

    let mut alpha_cnt: Vec<u32> = vec![0; 26];

    for c in &v {
        alpha_cnt[c.to_digit(36).unwrap().saturating_sub(10) as usize] += 1;
    }

    let mut idx = 0usize;
    let mut tmp_sum = 0;

    while tmp_sum + alpha_cnt[idx] <= m {
        tmp_sum += alpha_cnt[idx];
        idx += 1;
    }

    let mut remove = m - tmp_sum;

    for c in v {
        if (c as u8 - 'a' as u8) < (idx as u8) {
            continue;
        } else if (c as u8 - 'a' as u8) == (idx as u8) {
            if remove == 0 {
                write!(output, "{}", c).unwrap();
            } else {
                remove -= 1;
            }
        } else {
            write!(output, "{}", c).unwrap();
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
