use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let k = next_i32(&mut input);

    for i in 1..=k {
        let n = next_i32(&mut input);
        let w_persons = next_i32(&mut input);
        let e_persons = next_i32(&mut input);

        let mut ans = 0;

        for _ in 0..n {
            let lww = next_i32(&mut input);
            let lwe = next_i32(&mut input);
            let lew = next_i32(&mut input);
            let lee = next_i32(&mut input);
            ans += (w_persons * lww + e_persons * lew).max(w_persons * lwe + e_persons * lee); 
        }

        write!(output, "Data Set {}:\n{}\n\n", i, ans).unwrap();
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
