use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut level = 0;
        for c in line.unwrap().chars() {
            if c == '(' {
                level += 1
            } else if c == ')' {
                level -= 1
            }
        }
        println!("{}", level);
    }
}
