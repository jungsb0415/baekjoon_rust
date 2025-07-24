use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let mut case = 1;

    loop {
        let ideal = next_i32(&mut input);
        let mut cur = next_i32(&mut input);

        if ideal == 0 && cur == 0 {
            break;
        }

        let mut alive = true;

        loop {

            let ch: char = input.next().unwrap().parse().unwrap();
            let num = next_i32(&mut input);

            if ch == '#' {
                break;
            }

            if ch == 'E' {
                cur -= num;
            } else {
                cur += num;
            }

            if cur <= 0 {
                alive = false;
            }
        }

        let ans = if !alive {
            "RIP"
        } else if ideal < cur * 2 && cur < ideal * 2 {
            ":-)"
        } else {
            ":-("
        };

        write!(output, "{} {}\n", case, ans).unwrap();
        case += 1;
    }


    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}