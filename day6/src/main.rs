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
        let mut groups: [u64; 9] = [0;9];
        // Parse groups for day 0
        for i in 0..9 {
            groups[i] = ages.iter().fold(0, |acc, &x| acc + if x==i as i32 {1} else {0})
        }

        // PART ONE
        for i in 0..80 {
            // First check how many children we need to add at the end
            let spawn: i32 = ages.iter().fold(0, |acc, &x| acc + if x == 0 {1} else {0});
            
            // Decrement ages
            ages = ages.iter().map(|&x| if x == 0 {6} else {x-1}).collect();

            // Spawn offspring
            ages.append(&mut vec![8;spawn as usize]);
        }

        // PART TWO
        for i in 0..256 {
            // Check how many spawns we need
            let spawn: u64 = groups[0];
            // Decrement ages
            for k in 0..8 {
                groups[k] = groups[k+1];
            }
            // Move parents to group 6
            groups[6] += spawn;
            // Spawn new
            groups[8] = spawn;
        }

        println!("PART ONE:");
        println!("Number after 80 days: {}", ages.len());
        println!("PART TWO:");
        println!("Number afte 256 days: {}", groups[0]+groups[1]+groups[2]+groups[3]+groups[4]+groups[5]+groups[6]+groups[7]+groups[8]);
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