use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    if let Ok(data) = parse_input("./input") {

    };
}

/// Parse input
fn parse_input(filename: &str) -> io::Result<Vec<char>>{
    let mut data: Vec<char> = vec![];
    // Define input parsing regex for instructions
    let re = Regex::new(r"([A-Z])([A-Z]) -> ([A-Z])").expect("Invalid regex!");
    match read_lines(filename){
        Result::Ok(lines) => {            // Parse input lines
            for (i, line) in lines.enumerate() {

            }
            return Ok(data);
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