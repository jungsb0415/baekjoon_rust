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
        let m = next_u32(&mut input);

        let v: Vec<u32> = (0..n)
            .map(|_| next_u32(&mut input)).collect();

        let mut i = 0usize;
        let mut j = n as usize - 1;
        let mut ans = 0;

        while i < j {
            if v[i] + v[j] == m {
                ans += 1;
                i += 1;
                j -= 1;
            } else if v[i] + v[j] > m {
                j -= 1;
            } else {
                i += 1;
            }
        }

        write!(output, "Case #{c}: {}\n", ans).unwrap();
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
