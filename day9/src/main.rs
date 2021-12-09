use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data) = parse_input("./input") {
        println!("PART ONE");
        println!("Risk: {}", part_one(&data));
        println!("PART TWO");
        println!("Risk: {}", part_two(&data));
    }
}

fn is_minimum(data: &Vec<Vec<i32>>, i: i32, j: i32) -> bool{
    data[i as usize][j as usize] < get_safe(&data, i-1, j) && 
    data[i as usize][j as usize] < get_safe(&data, i+1, j) && 
    data[i as usize][j as usize] < get_safe(&data, i, j+1) && 
    data[i as usize][j as usize] < get_safe(&data, i, j-1)
}

fn get_safe(data: &Vec<Vec<i32>>, i: i32, j:i32) -> i32 {
    if i > 0 || j > 0 {
        if let Some(val1) = data.get(i as usize){
            if let Some(val2) = val1.get(j as usize) {
                *val2
            }
            else {
                10
            }
        } else {
            10
        }
    } else {
        10
    }
}

fn part_one(data: &Vec<Vec<i32>>) -> i32 {
    let ln = (data.len()-1, data[0].len()-1);
    let mut risk = 0;
    for i in 0..=ln.0 {
        for j in 0..=ln.1 {
            if  is_minimum(&data, i as i32, j as i32){
                risk += 1 + data[i][j];
            }
        }
    }
    risk
}

fn part_two(data: &Vec<Vec<i32>>) -> i32{
    0
}

/// This returns a vector of ages, as given in the input filename and wrapped in Result
fn parse_input(filename: &str) -> io::Result<Vec<Vec<i32>>>{
    let mut data: Vec<Vec<i32>> = Vec::new();
    match read_lines(filename){
        Result::Ok(lines) => {            // Parse input lines
            for line in lines {
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