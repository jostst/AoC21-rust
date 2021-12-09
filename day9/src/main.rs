use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data) = parse_input("./input") {
        println!("{:?}", data);
    }
}

/// This returns a vector of ages, as given in the input filename and wrapped in Result
fn parse_input(filename: &str) -> io::Result<Vec<Vec<i32>>>{
    let mut data: Vec<Vec<i32>> = Vec::new();
    match read_lines(filename){
        Result::Ok(lines) => {            // Parse input lines
            for (i, line) in lines.enumerate() {
                data.push(line.expect("Invalid line")
                                .chars()
                                .map(|x| x.to_string().parse::<i32>().expect("Cannot parse!"))
                                .collect::<Vec<i32>>());
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