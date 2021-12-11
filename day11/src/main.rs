use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NEIGHBOURS: [(i32,i32); 8] = [(-1,-1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn main() {
    if let Ok(mut data) = parse_input("./input") {
        println!("PART ONE");
        let mut flashes = 0;
        for _ in 0..100 {
            flashes += propagate_epoch(&mut data);;
        };
        println!("Flashes: {}", flashes);
        
        println!("PART TWO");
        let mut i = 100;
        loop {
            let new_flashes = propagate_epoch(&mut data);
            i += 1;
            if new_flashes == (data.len() as i32)*(data[0].len() as i32) {
                println!("First sync at: {}", i);
                break;
            }
        }
    };
}

/// Propagate the energy level matrix for one epoch and return number of flashes
/// It follows strictly the algorithm described in the task, so 3 passes
fn propagate_epoch(data: &mut Vec<Vec<i32>>) -> i32 {
    // A state array for the epoch
    let mut flashed = vec![vec![false; data.len()]; data[0].len()];
    // Number of flashes happened
    let mut flashes = 0;

    // First, increase all energy levels by one
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            data[i][j] += 1;
        };
    };

    // Second, flash all above 9
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            flashes += flash(data, &mut flashed, i, j);
        };
    };

    // Third, reset to 0 if above 9
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] > 9 {
                data[i][j] = 0;
            };
        };
    };

    return flashes;
}

fn safe_increase(d: &mut Vec<Vec<i32>>, i: i32, j: i32) -> () {
    if i >= 0 && j >= 0 && i < d.len() as i32 && j < d[0].len() as i32 {
        let i = i as usize;
        let j = j as usize;
        d[i][j] += 1;
    }
}

fn safe_flash(d: &mut Vec<Vec<i32>>, s: &mut Vec<Vec<bool>>, i: i32, j: i32) -> i32{
    if i >= 0 && j >= 0 && i < d.len() as i32 && j < d[0].len() as i32 {
        flash(d, s, i as usize, j as usize)
    } else {
        0
    }
}

/// Flashes the octopus. d are energy levels, s is state, i and j are coordinates
/// Returns number of flashes performed
fn flash(d: &mut Vec<Vec<i32>>, s: &mut Vec<Vec<bool>>, i: usize, j: usize) -> i32{
    let mut flashes = 0;
    // Check if flash
    if !s[i][j] && d[i][j] > 9 {
        s[i][j] = true;
        flashes += 1;
        // Increase naighours
        for neighbour in NEIGHBOURS {
            safe_increase(d, i as i32 + neighbour.0, j as i32 + neighbour.1);
        }
        // Recursively flash naighbours
        for neighbour in NEIGHBOURS {
            flashes += safe_flash(d, s, i as i32 + neighbour.0, j as i32 + neighbour.1);
        }
    }
    return flashes;
}

/// This returns a matrix of energy levels, as given in the input filename and wrapped in Result
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