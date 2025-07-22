use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let area = next_i32(&mut input);
    let peri = next_i32(&mut input);

    let mut flag = false;

    for i in 1..=peri {
        if area % i == 0 && (i + area / i) * 2 == peri {
            flag = true;
            write!(output, "{} {}\n", i.max(area/i), i.min(area/i)).unwrap();
            break;
        }
    }

    if !flag {
        write!(output, "-1\n").unwrap();

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
