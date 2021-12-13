use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut total_paper = 0;
    for line in stdin.lock().lines() {
        let dims = line.expect("woot").split("x").map(|x| x.parse()).collect::<Result<Vec<u32>, _>>().expect("woot");
        let sides: Vec<u32> = vec![dims[0]*dims[1], dims[1]*dims[2], dims[2]*dims[0]];
        
        let min_side = sides.iter().min().unwrap();
        let square_feet = 2*sides.iter().sum::<u32>() + min_side;

        total_paper += square_feet;
    }
    println!("{}", total_paper);
}
