use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Neighbours to visit on events
const NEIGHBOURS: [(i32,i32); 8] = [(-1,-1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

/// Main part of the solution. It works, if first synchronization happens after 100 epochs
fn main() {
    if let Ok(mut data) = parse_input("./input") {

        println!("PART ONE");
        let flashes = (0..100).fold(0, |acc, _| acc + propagate_epoch(&mut data));
        println!("Flashes: {}", flashes);
        
        println!("PART TWO");
        let elems = (data.len() as i32)*(data[0].len() as i32);
        let mut i = 100;
        loop {
            i += 1;
            if propagate_epoch(&mut data) ==  elems{
                println!("First sync at: {}", i);
                 if i > 100 {break;};
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
    // Size of the data
    let n = (data.len(), data[0].len());

    // First, increase all energy levels by one
    (0..n.0).for_each(|i| (0..n.1).for_each(|j| data[i][j] += 1));

    // Second, flash all above 9
    (0..n.0).for_each(|i| (0..n.1).for_each(|j| flashes += flash(data, &mut flashed, i, j)));

    // Third, reset to 0 if above 9
    (0..n.0).for_each(|i| (0..n.1).for_each(|j| if data[i][j] > 9 {data[i][j] = 0;}));

    return flashes;
}

/// safe checks if the desired neighbour is safe to use (i.e. not out of bounds). It returns an
/// Option with Some(new coordinates as usize) for safe indices and None if out of bounds
fn safe(d: &mut Vec<Vec<i32>>, i: usize, j: usize, n: (i32, i32)) -> Option<(usize, usize)> {
    let ni: i32 = i as i32 + n.0;
    let nj: i32 = j as i32 + n.1;
    if ni >= 0 && nj >= 0 && ni < d.len() as i32 && nj < d[0].len() as i32 {
        Some((ni as usize, nj as usize))
    } else {
        None
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
            if let Some((ni,nj)) = safe(d, i, j, neighbour){d[ni][nj] += 1;};
        }
        // Recursively flash naighbours
        for neighbour in NEIGHBOURS {
            if let Some((ni, nj)) = safe(d, i, j, neighbour) {flashes += flash(d, s, ni, nj);};
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