//! AoC21, Day 6
//! So, suppose you have a lanternfish with an internal timer value of 3:
//!
//! - After one day, its internal timer would become 2.
//! - After another day, its internal timer would become 1.
//! - After another day, its internal timer would become 0.
//! - After another day, its internal timer would reset to 6, and it would create a new lanternfish with an internal timer of 8.
//! - After another day, the first lanternfish would have an internal timer of 5, and the second lanternfish would have an internal timer of 7.
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(mut ages) = parse_input("./input"){
        println!("{:?}", ages);
    }
}

/// This returns a vector of ages, as given in the input filename and wrapped in Result
fn parse_input(filename: &str) -> io::Result<Vec<i32>>{
    let mut ages: Vec<i32> = Vec::new();
    match read_lines(filename){
        Result::Ok(lines) => {
            // Parse input lines
            for line in lines {
                ages.append( &mut line.expect("Error on reading line!")
                                    .split(",")
                                    .map(|x| x.parse::<i32>().unwrap())
                                    .collect::<Vec<i32>>());
            }
            return Ok(ages);
        },
        Result::Err(e) => {
            println!("File error!");
            println!("{:?}", e);
            return Err(e);
        },
    };
}

/// This is a buffered file reader which opens a file at filename and resturns io::Result
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}