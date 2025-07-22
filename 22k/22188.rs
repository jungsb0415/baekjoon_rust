use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);
    let m = next_i32(&mut input);

    let mut company = vec![];

    for _ in 0..n {
        let c = next_i32(&mut input);
        let t = next_i32(&mut input);
        let p = next_i32(&mut input);
        company.push((c, t, p));
    }

    let call_time: Vec<i32> = (0..m)
        .map(|_| next_i32(&mut input)).collect();

    let mut min = 1<<30;
    let mut ans = 0;

    for i in 0..n as usize {
        let (c, t, p) = company[i];
        let mut tmp = c * 100;
        
        for call in &call_time {
            if *call < t {
                continue;
            } else {
                let k = (call - 1) / t + 1;
                tmp += k * p;
            }
        }

        if min > tmp {
            min = tmp;
            ans = i+1;
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
