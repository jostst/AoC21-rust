// Test solutions are:
//  - test1: 10 paths
//  - test2: 19 paths
//  - test3: 226 paths

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    if let Ok(data) = parse_input("./test1") {
        println!("{:?}", data);
    }
}

/// This returns a hash map that has all the caverns as keys and caverns it connects to 
/// in a vector of strings stored as the value of the key
fn parse_input(filename: &str) -> io::Result<HashMap<String, Vec<String>>>{
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    match read_lines(filename){
        Result::Ok(lines) => {            // Parse input lines
            for line in lines {
                let nl = line.expect("Invalid line")
                        .split("-")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                if let Some(old) = data.get_mut(&nl[0]) {
                    old.push(nl[1][..].to_string());
                } else {
                    data.insert(nl[0][..].to_string(), vec![nl[1][..].to_string()]);
                }
                if let Some(old) = data.get_mut(&nl[1]) {
                    old.push(nl[0][..].to_string());
                } else {
                    data.insert(nl[1][..].to_string(), vec![nl[0][..].to_string()]);
                }
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