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
        let mut santa_position = Point {x: 0, y: 0};
        let mut robot_position = Point {x: 0, y: 0};
        let mut visits = HashMap::new();

        *visits.entry(santa_position).or_insert(0) += 1;

        for (i, c) in line.unwrap().chars().enumerate() {
            if i%2 == 0 {
                match c {
                    '>' => santa_position.x += 1,
                    'v' => santa_position.y -= 1,
                    '<' => santa_position.x -= 1,
                    '^' => santa_position.y += 1,
                    _ => {}
                };
                *visits.entry(santa_position).or_insert(0) += 1
            } else {
                match c {
                    '>' => robot_position.x += 1,
                    'v' => robot_position.y -= 1,
                    '<' => robot_position.x -= 1,
                    '^' => robot_position.y += 1,
                    _ => {}
                };
                *visits.entry(robot_position).or_insert(0) += 1
            }
        }

        println!("{:?}", visits.len());
    }
}
