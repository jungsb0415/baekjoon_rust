// use std::fmt::Write;
use std::io::{Read, stdin};

#[derive(Clone)]
struct Node {
    left: usize,
    right: usize,
}

impl Node {
    fn new() -> Self {
        Node { left: 0, right: 0 }
    }
}

fn print(node_vec: &Vec<Node>, cur: usize) {
    if node_vec[cur].left != 0 {
        print(node_vec, node_vec[cur].left);
    }
    if node_vec[cur].right != 0 {
        print(node_vec, node_vec[cur].right);
    }
    print!("{cur}\n");
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    // let mut output = String::new();

    let n = 1000000usize;
    let mut node_vec: Vec<Node> = vec![Node::new(); n];

    let head: usize = input.next().unwrap().parse().unwrap();

    while let Some(num) = input.next() {
        let num: usize = num.parse().unwrap();
        let mut cur = head;

        loop {
            if num < cur {
                if node_vec[cur].left == 0 {
                    node_vec[cur].left = num;
                    break;
                } else {
                    cur = node_vec[cur].left;
                }
            } else {
                if node_vec[cur].right == 0 {
                    node_vec[cur].right = num;
                    break;
                } else {
                    cur = node_vec[cur].right;
                }
            }
        }
    }

    print(&node_vec, head);

    // write!(output, "{}\n", ans).unwrap();
    // print!("{output}");
}
