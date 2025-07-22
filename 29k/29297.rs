use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);

    for _ in 0..n {
        let tmp = input.next().unwrap();
        let a: i32 = tmp[0..1].parse().unwrap();
        let b: i32 = tmp[2..3].parse().unwrap();

        let mut cnt_a = 0;
        let mut cnt_b = 0;

        for i in 0..10 {
            for j in 0..10 {
                if a + i > b + j {
                    cnt_a += 1;
                } else if a + i < b + j {
                    cnt_b += 1;
                } else {
                    if i > b {
                        cnt_a += 1;
                    } else if i < b {
                        cnt_b += 1;
                    }
                }
            }
        }

        write!(output, "{} {}\n", cnt_a, cnt_b).unwrap();
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
