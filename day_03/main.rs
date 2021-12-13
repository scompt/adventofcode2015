use std::io::{self, BufRead};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut position = Point {x: 0, y: 0};
        let mut visits = HashMap::new();

        *visits.entry(position).or_insert(0) += 1;

        for c in line.unwrap().chars() {
            match c {
                '>' => position.x += 1,
                'v' => position.y -= 1,
                '<' => position.x -= 1,
                '^' => position.y += 1,
                _ => {}
            };
            *visits.entry(position).or_insert(0) += 1
        }

        println!("{:?}", visits.len());
    }
}
