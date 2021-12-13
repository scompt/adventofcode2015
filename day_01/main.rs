use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut level = 0;
        for (i, c) in line.unwrap().chars().enumerate() {
            if c == '(' {
                level += 1
            } else if c == ')' {
                level -= 1
            }
            if level < 0 {
                println!("{}", i+1);
                return;
            }
        }
    }
}
