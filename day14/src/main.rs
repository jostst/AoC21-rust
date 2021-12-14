use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    if let Ok((data, instructions)) = parse_input("./input") {
        println!("PART ONE:");
        println!("Score is: {}", part_one(&data, &instructions));

        println!("PART TWO:");
        println!("Score is: {}", part_two(&data, &instructions));
    }
}

/// This is part one function
fn part_one(data: &Vec<char>, instructions: &Vec<(char, char, char)>) -> i32 {
    let mut polymer = data.to_owned();
    
    // Iterate 10 times
    for _ in 0..10 {
        polymer = grow_polymer(&polymer, &instructions);
    }

    calculate_hash(&polymer)
}

/// This is part one function
fn part_two(data: &Vec<char>, instructions: &Vec<(char, char, char)>) -> i32 {
    let mut polymer = data.to_owned();
    
    // Iterate 10 times
    for i in 0..40 {
        polymer = grow_polymer(&polymer, &instructions);
        println!("Iteration: {}", i);
    }

    calculate_hash(&polymer)
}

/// This grows polymer according to the instructions
fn grow_polymer(polymer: &Vec<char>, instructions: &Vec<(char, char, char)>) -> Vec<char> {
    let mut new = vec![];
    
    // Iterate over the current polymer
    for i in 0..polymer.len()-1 {
        new.push(polymer[i]);
        // If instruction exists for i,i+1 pair perform the insertion
        for instruction in instructions {
            if polymer[i] == instruction.0 && polymer[i+1] == instruction.1 {
                new.push(instruction.2);
                break;
            }
        }
    }
    new.push(*polymer.last().unwrap());

    return new;
}

/// This function calculates the hash
fn calculate_hash(data: &Vec<char>) -> i32 {
    // calculate occurances
    let mut occurances: Vec<i32> = vec![0;25];
    for c in data {
        occurances[(*c as i32 - 65) as usize] += 1;
    }
    let max = occurances.clone().into_iter().max().unwrap();
    let mut min = max;
    for i in occurances {
        if i > 0 && i < min {min = i;};
    }
    max - min
}

/// This returns a hash map that has all the caverns as keys and caverns it connects to 
/// in a vector of strings stored as the value of the key
fn parse_input(filename: &str) -> io::Result<(Vec<char>, Vec<(char, char, char)>)>{
    let mut data: Vec<char> = vec![];
    let mut instructions: Vec<(char,char,char)> = vec![];
    // Define input parsing regex for instructions
    let re = Regex::new(r"([A-Z])([A-Z]) -> ([A-Z])").expect("Invalid regex!");
    match read_lines(filename){
        Result::Ok(lines) => {            // Parse input lines
            for (i, line) in lines.enumerate() {
                let string = line.unwrap();
                // Parse template in the zeroth row as a vector oc chars
                if i == 0 {
                    data = string.chars().collect();
                }
                
                // Starting from row idx 2, parse substitutions using RegEx
                if i > 1 {
                    let cap = re.captures(&string).unwrap();
                    instructions.push((
                        cap[1].chars().last().unwrap(), 
                        cap[2].chars().last().unwrap(), 
                        cap[3].chars().last().unwrap()
                    ));
                }
            }
            return Ok((data, instructions));
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