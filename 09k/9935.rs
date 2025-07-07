use std::fmt::Write;
use std::io::{Read, stdin};

fn is_c4(a: &Vec<char>, b: &Vec<char>) -> bool {
    if b.len() > a.len() {
        return false;
    }

    let a_suffix = &a[a.len() - b.len()..];
    a_suffix == b
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let string: Vec<char> = input.next().unwrap().chars().collect();
    let c4: Vec<char> = input.next().unwrap().chars().collect();

    let mut stack: Vec<char> = vec![];

    for c in string {
        stack.push(c);

        while is_c4(&stack, &c4) {
            for _ in 0..c4.len() {
                stack.pop();
            }
        }
    }

    if stack.len() == 0 {
        write!(output, "FRULA\n").unwrap();
    } else {
        for c in stack {
            write!(output, "{}", c).unwrap();
        }
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
