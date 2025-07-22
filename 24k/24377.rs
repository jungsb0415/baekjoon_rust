use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let v: Vec<i32> = (0..4)
        .map(|_| next_i32(&mut input)).collect();

    let cnt_zero = v.iter().filter(|&&x| x == 0).count();

    if cnt_zero == 0 {
        write!(output, "{} {}\n", v[0], v[1]).unwrap();
    } else if cnt_zero == 1 {
        let idx = v.iter().position(|x| *x == 0).unwrap();
        write!(output, "{} {}\n", idx+1 ,10 - v.iter().sum::<i32>()).unwrap();
    } else {
        match v.iter().sum::<i32>() {
            3 => write!(output, "3 4\n").unwrap(),
            4 => write!(output, "2 4\n").unwrap(),

            6 => write!(output, "1 3\n").unwrap(),
            7 => write!(output, "1 2\n").unwrap(),
            _ => {
                if v.contains(&1) {
                    write!(output, "2 3\n").unwrap()
                } else {
                    write!(output, "1 4\n").unwrap()
                }
            }
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
