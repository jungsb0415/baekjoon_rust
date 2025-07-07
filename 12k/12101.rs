use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let mut n: u32 = next_u32(&mut input);
    let mut k: u32 = next_u32(&mut input);

    let mut method: Vec<u32> = vec![0, 1, 2, 4];

    for i in 4..35 {
        let val = method[i-1] + method[i-2] + method[i-3];
        method.push(val);
    }

    if k > method[n as usize] {
        print!("-1");
        return;
    }

    loop {
        if n == 1 {
            if k == 1 {write!(output, "1").unwrap();}
            break;
        } else if n == 2 {
            if k == 1 {write!(output, "1+1").unwrap();}
            if k == 2 {write!(output, "2").unwrap();}
            break;
        } else if n == 3 {
            if k == 1 {write!(output, "1+1+1").unwrap();}
            if k == 2 {write!(output, "1+2").unwrap();}
            if k == 3 {write!(output, "2+1").unwrap();}
            if k == 4 {write!(output, "3").unwrap();}
            break;
        }

        if k <= method[n as usize - 1] {
            write!(output, "1+").unwrap();
            n -= 1;
        } else if k <= method[n as usize - 1] + method[n as usize - 2] {
            write!(output, "2+").unwrap();
            k -= method[n as usize - 1];
            n -= 2;
        } else {
            write!(output, "3+").unwrap();
            k -= method[n as usize - 1] + method[n as usize - 2];
            n -= 3;
        }
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}"); 
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}
