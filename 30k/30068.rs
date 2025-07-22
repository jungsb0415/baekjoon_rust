use std::collections::HashMap;
use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);
    
    let mut hash: HashMap<i32, i32> = HashMap::new();

    for _ in 0..n {
        let t = next_i32(&mut input);
        let num = next_i32(&mut input);

        if hash.get(&num) == None {
            hash.insert(num, t);
        } else {
            let start = hash.get(&num).unwrap();
            write!(output, "{} {}\n", num, t - *start).unwrap();
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
