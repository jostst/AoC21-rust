use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    if let Ok((data, instructions)) = parse_input("./input") {
        println!("PART ONE");
        println!("Elements: {}", part_one(&data, &instructions));
    }
}

fn part_one(data: &Vec<(i32, i32)>, instructions: &Vec<(FoldDirection, i32)>) -> i32 {
    let mut d = data.to_owned();
    fold(&mut d, instructions[0].0, instructions[0].1);
    for tmp in get_map(&d){
        println!("{:?}", tmp);
    }
    count_dots(&get_map(&d))
}

fn fold(data: &mut Vec<(i32, i32)>, direction: FoldDirection, idx: i32) -> (){
    // Basically, two directions
    for i in 0..data.len(){
        let old = data[i];
        match direction {
            FoldDirection::X => {
                if old.0 > idx {
                    data[i] = (idx - (old.0-idx),old.1);
                }
            },
            FoldDirection::Y => {
                if old.1 > idx {
                    data[i] = (old.0, idx - (old.1 - idx));
                }
            },
        };
    };
}

fn get_map(data: &Vec<(i32, i32)>) -> Vec<Vec<i32>> {
    // Get size of the map
    let mut size: (i32, i32) = (0,0);
    data.clone().iter().for_each(|&(i,j)| {
        let mut tmp = size;
        if i > tmp.1 {tmp.1 = i;}
        if j > tmp.0 {tmp.0 = j;}
        size = (tmp.0, tmp.1);
    });
    size = (size.0+1, size.1+1);
    // Initialize empty map (filled with 0)
    let mut map: Vec<Vec<i32>>= vec![vec![0;size.1 as usize];size.0 as usize];
    for point in data.clone() {
        map[point.1 as usize][point.0 as usize] += 1;
    }
    map
}

fn count_dots(data: &Vec<Vec<i32>>) -> i32 {
    let mut acc = 0;
    for column in data.clone(){
        for cell in column.clone(){
            if cell > 0 {acc += 1;};
        };
    };
    acc
}

/// Enum to store fold direction
#[derive(Debug, Clone, Copy)]
enum FoldDirection {
    X,
    Y
}

/// This returns a hash map that has all the caverns as keys and caverns it connects to 
/// in a vector of strings stored as the value of the key
fn parse_input(filename: &str) -> io::Result<(Vec<(i32,i32)>, Vec<(FoldDirection, i32)>)>{
    let mut data: Vec<(i32,i32)> = vec![];
    let mut instructions: Vec<(FoldDirection, i32)> = vec![];
    // Define input parsing regex for instructions
    let re = Regex::new(r"fold along ([x,y])=(\d+)").expect("Invalid regex!");
    match read_lines(filename){
        Result::Ok(lines) => {            // Parse input lines
            for line in lines {
                let vrstica = line.unwrap();
                if vrstica.len() > 0 && vrstica.len() < 12{
                    // Parse input data
                    let tmp = vrstica.split(",").map(|x| x.parse::<i32>().expect("Invalid number")).collect::<Vec<i32>>();
                    data.push((tmp[0], tmp[1]));
                } else if vrstica.len() > 12 {
                    // Parse fold directions
                    let cap = re.captures(&vrstica).unwrap();
                    if &cap[1] == "x" {
                        instructions.push((FoldDirection::X,cap[2].parse().unwrap()));
                    } else {
                        instructions.push((FoldDirection::Y,cap[2].parse().unwrap()));
                    }
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