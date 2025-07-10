use std::fmt::Write;
use std::io::{Read, stdin};
use std::vec;

fn check(v: &Vec<Vec<u32>>) -> bool {

    let expected: Vec<u32> = (1..=9).collect();

    for i in 0..9 {
        let mut tmp: Vec<u32> = vec![];
        for j in 0..9 {
            tmp.push(v[i][j]);
        }
        tmp.sort();
        tmp.dedup();
        if tmp != expected {return false;}
    }

    for i in 0..9 {
        let mut tmp: Vec<u32> = vec![];
        for j in 0..9 {
            tmp.push(v[j][i]);
        }
        tmp.sort();
        tmp.dedup();
        if tmp != expected {return false;}
    }

    for i in 0..9 {
        let mut tmp: Vec<u32> = vec![];
        for j in 0..9 {
            tmp.push(v[(i/3)*3 + j/3][(i%3)*3 + j%3]);
        }
        tmp.sort();
        tmp.dedup();
        if tmp != expected {return false;}
    }

    true
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_u32(&mut input);

    for c in 1..=n {
        let mut v: Vec<Vec<u32>> = vec![vec![0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                let k = next_u32(&mut input);

                v[i][j] = k;
            }
        }

        let ans = if check(&v) {
            "CORRECT"
        } else {
            "INCORRECT"
        };

        write!(output, "Case {c}: {}\n", ans).unwrap();
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
