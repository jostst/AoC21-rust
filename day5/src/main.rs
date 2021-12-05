use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::cmp;

fn main() {
    // Define the results grid
    const SIZE: usize = 1000;
    let mut state: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut xeshv: u32 = 0;
    let mut xesal: u32 = 0;

    // Define input parsing regex with 4 capture groups
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").expect("Invalid regex!");

    match read_lines("./input"){
        Result::Ok(lines) => {
            // Parse input lines
            for line in lines {
                // parse current line
                // TODO: this is ugly, fix it, when finished!
                let string = line.expect("Not a valid line!)");
                let cap = re.captures(&string).unwrap();
                let x1: usize = cap[1].parse().unwrap();
                let y1: usize = cap[2].parse().unwrap();
                let x2: usize = cap[3].parse().unwrap();
                let y2: usize = cap[4].parse().unwrap();

                // PART ONE & TWO
                // check for horizontal line
                if x1 == x2 || y1 == y2 {
                    for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                        for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                            if state[y][x] == 1 {
                                xeshv += 1;
                                xesal += 1;
                            }
                            state[y][x] += 1;
                        }
                    }
                } else {
                    // This is then a diagonal line
                    // TODO: shouldn't this be a part of standard library?
                    let xs: Vec<usize> = if x1 > x2 {(x2..=x1).rev().collect()} else {(x1..=x2).collect()};
                    let ys: Vec<usize> = if y1 > y2 {(y2..=y1).rev().collect()} else {(y1..=y2).collect()};

                    xs.iter().enumerate().for_each(|(i,&x)|{
                        let y = ys[i];
                        if state[y][x] == 1 {xesal += 1;}
                        state[y][x] += 1;
                    })
                }
            }
            
            println!("PART ONE:");
            println!("Crossings: {}", xeshv);
            println!("PART TWO:");
            println!("Crossings: {}", xesal)
        },
        Result::Err(e) => {
            println!("File error!");
            println!("{:?}", e);
        },
    };
}


// Buffered reader
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}