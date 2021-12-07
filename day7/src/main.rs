use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(pos) = parse_input("./input") {
        println!("PART ONE:");
        println!("Fuel spent: {}", naive_algorithm(&pos, &cost_fnc));
        println!("PART TWO:");
        println!("Fuel spent: {}", naive_algorithm(&pos, &cost_fnc2));
    }
}

/// Naive optimization algorithm -> it is a search....
fn naive_algorithm(pos: &Vec<i32>, f: &dyn Fn(&Vec<i32>, i32) -> i32) -> i32 {
    let mut min = f(&pos, 0);
    for i in 1..*pos.iter().max().unwrap() {
        let cost = f(&pos, i);
        if cost < min {min = cost;}
    }
    min
}

// Cost function for part 1
fn cost_fnc(pos: &Vec<i32>, target: i32) -> i32{
    pos.iter().fold(0, |acc, &x| acc + (target - x).abs())
}

/// Cost function for part 2
fn cost_fnc2(pos: &Vec<i32>, target: i32) -> i32 {
    pos.iter().fold(0, |acc, &x| acc + ((target-x).abs()*((target-x).abs()+1))/2)
}

/// This returns a vector of ages, as given in the input filename and wrapped in Result
fn parse_input(filename: &str) -> io::Result<Vec<i32>>{
    let mut pos: Vec<i32> = Vec::new();
    match read_lines(filename){
        Result::Ok(lines) => {
            // Parse input lines
            for line in lines {
                pos.append( &mut line.expect("Error on reading line!")
                                    .split(",")
                                    .map(|x| x.parse::<i32>().unwrap())
                                    .collect::<Vec<i32>>());
            }
            return Ok(pos);
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