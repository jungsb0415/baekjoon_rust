use std::fmt::Write;
use std::io::{Read, stdin};

fn f(str: &str) -> Vec<usize> {
    let mut ret: Vec<usize> = vec![0; 26];
    for c in str.chars() {
        ret[(c as u8 - 'a' as u8) as usize] += 1;
    }
    ret
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let str= input.next().unwrap();
    let standard = f(str);

    let n = next_i32(&mut input);

    for _ in 0..n {
        let str= input.next().unwrap();
        let com = f(str);

        let mut flag = true;

        for i in 0..26 {
            if standard[i] < com[i] {
                flag = false;
            }
        }

        if flag {
            write!(output, "{}\n", str).unwrap();
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
