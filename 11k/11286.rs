use std::fmt::Write;
use std::io::{Read, stdin};

fn cmp(a: i32, b: i32) -> bool {
    if a.abs() < b.abs() {
        true
    } else if a.abs() > b.abs() {
        false
    } else if a < b {
        true
    } else {
        false
    }
}

fn push(heap: &mut Vec<i32>, num: i32, idx: &mut usize) {

    *idx += 1;
    heap[*idx] = num;

    let mut i = *idx;

    while i != 1 {
        if cmp(heap[i], heap[i/2]) {
            heap.swap(i, i/2);
            i /= 2;
        } else {
            break;
        }
    }
}

fn pop(heap: &mut Vec<i32>, idx: &mut usize) -> i32 {
    
    if *idx == 0 {
        return 0;
    }

    let ret = heap[1];
    heap[1] = heap[*idx];
    *idx -= 1;
    
    let mut i = 1;

    loop {
        let left = i * 2;
        let right = i * 2 + 1;
        let mut best = i;

        if left <= *idx && cmp(heap[left], heap[best]) {
            best = left;
        }
        if right <= *idx && cmp(heap[right], heap[best]) {
            best = right;
        }

        if best == i {
            break;
        }

        heap.swap(i, best);
        i = best;
    }

    ret
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let mut heap: Vec<i32> = vec![0; 400000];
    let mut idx = 0;

    let n = next_i32(&mut input);

    for _ in 0..n {
        let n = next_i32(&mut input);
        if n == 0 {
            write!(output, "{}\n", pop(&mut heap, &mut idx)).unwrap();
        } else {
            push(&mut heap, n, &mut idx);
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
