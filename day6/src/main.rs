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

/// Define number of age classes
const CLASSNUM:usize = 9;

/// Program entry point
fn main() {
    if let Ok(ages) = parse_input("./input"){
        println!("PART ONE:");
        println!("Number after 80 days: {}", naive_solution(&ages, 80));
        println!("PART TWO:");
        println!("Number afte 256 days: {}", better_solution(&ages, 256));
    }
}

/// This is a better solution that uses age groups as cache
fn better_solution(agesb: &Vec<i32>, epochs: i32) -> u64 {
    // Variables used by the function
    let ages = agesb.to_owned();
    let mut groups: [u64; CLASSNUM] = [0; CLASSNUM];

    // Parse groups for day 0
    for i in 0..CLASSNUM {
        groups[i] = ages.iter().fold(0, |acc, &x| acc + if x==i as i32 {1} else {0})
    }

    for _ in 0..epochs {
        // Check how many spawns we need
        let spawn: u64 = groups[0];
        // Decrement ages
        for k in 0..CLASSNUM-1 {
            groups[k] = groups[k+1];
        }
        // Move parents to group 6
        groups[6] += spawn;
        // Spawn new
        groups[CLASSNUM-1] = spawn;
    }

    groups.iter().sum::<u64>()
}

/// This is a naive solution that uses a list of ages as cache
fn naive_solution(agesb: &Vec<i32>, epochs: i32) -> i32{
    let mut ages = agesb.to_owned();
    // PART ONE
    for _ in 0..epochs {
        // First check how many children we need to add at the end
        let spawn: i32 = ages.iter().fold(0, |acc, &x| acc + if x == 0 {1} else {0});
        // Decrement ages
        ages = ages.iter().map(|&x| if x == 0 {6} else {x-1}).collect();
        // Spawn offspring
        ages.append(&mut vec![8;spawn as usize]);
    }
    ages.len() as i32
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