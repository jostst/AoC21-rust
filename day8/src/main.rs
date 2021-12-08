use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    if let Ok(data) = parse_input("./input") {
        println!("PART ONE");
        println!("{}", part_one(&data));
    }
}

/// This returns a vector of ages, as given in the input filename and wrapped in Result
fn parse_input(filename: &str) -> io::Result<Vec<Display>>{
    let mut data: Vec<Display> = Vec::new();
    match read_lines(filename){
        Result::Ok(lines) => {
            // Parse input lines
            for line in lines {
                let entries: Vec<String> = line.expect("Invalid line")
                                            .split(" ")
                                            .map(|x| x.to_string())
                                            .collect();
                data.push(Display::new(entries[0..10].to_vec(), entries[11..15].to_vec()));
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

fn part_one(data: &Vec<Display>) -> i32{
    data
        .iter()
        .fold(0, |acc, x| acc + x.count_simple())
}

struct Display {
    combinations: Vec<String>,
    outputs: Vec<String>,
    key: HashMap<String,i32>,
}

impl Display {
    pub fn new(cmbs: Vec<String>, outpts: Vec<String>) -> Self {
        Self {
            combinations: cmbs,
            outputs: outpts,
            key: HashMap::new(),
        }
    }

    pub fn count_simple(&self) -> i32 {
        self.outputs
            .iter()
            .fold(0, |acc, x| if x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7 {acc + 1} else {acc})
    }
}

/// This is a buffered file reader which opens a file at filename and resturns io::Result
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}