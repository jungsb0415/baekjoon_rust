use std::fmt::Write;
use std::io::{Read, stdin};

fn bit(mut n: i32) -> i32 {
    let mut ret = 0;
    while n != 0 {
        if n%2 == 1 {
            ret += 1;
        }
        n /= 2;
    }
    ret
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let t = next_i32(&mut input);

    for i in 1..=t {
        let n = next_i32(&mut input);

        let mut ans = 0;

        for a in 0..n/2+1 {
            let b = n-a;
            let tmp = bit(a) + bit(b);
            ans = ans.max(tmp);
        }
        write!(output, "Case #{i}: {}\n", ans).unwrap();
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
