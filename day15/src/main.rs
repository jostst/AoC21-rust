use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

const NEIGHBOURS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    if let Ok(data) = parse_input("./input") {
        println!("PART ONE");
        println!("Cost: {}", dijkstra(&data));
    };
}

/// Dijkstra's algorithm
fn dijkstra(data: &Vec<Vec<i32>>) -> i32 {
    // Create a set of unvisited nodes
    let mut unvisited: Vec<(i32,i32)> = vec![];
    for i in 0..data.len() { for j in 0..data[0].len() {unvisited.push((i as i32,j as i32));};};

    // Create a matrix of tantative distances
    let mut distances: Vec<Vec<i32>> = vec![vec![i32::MAX; data[0].len()]; data.len()];
    distances[0][0] = 0;

    // While nodes are still to be visited, repeat
    while unvisited.len() > 0 as usize {
        // Find current node with minimal distance
        let mut dist = i32::MAX;
        let mut current: (i32, i32) = (0, 0);
        for node in &unvisited {
            if distances[node.0 as usize][node.1 as usize] < dist {
                dist = distances[node.0 as usize][node.1 as usize];
                current = *node;
            }
        }
        println!("Current: ({},{})", current.0, current.1);

        // Update distances of the neighbour nodes, if needed
        for neighbour in NEIGHBOURS {
            // Check if neighbour is unvisited
            if let Some(n) = &unvisited.iter().position(|x| *x == (current.0+neighbour.0, current.1+neighbour.1)){
                let tmp = distances[current.0 as usize][current.1 as usize] + 
                            data[unvisited[*n].0 as usize][unvisited[*n].1 as usize];
                if tmp < distances[unvisited[*n].0 as usize][unvisited[*n].1 as usize] {
                    distances[unvisited[*n].0 as usize][unvisited[*n].1 as usize] = tmp;
                }
            }
        }

        // Remove current node from unvisited - e.g. mark current visited
        unvisited.remove(*&unvisited.iter().position(|x| *x == current).unwrap());

        // If the destination node was visited, break -> we win
        if current == ((distances.len()-1) as i32, (distances[0].len()-1) as i32) {break;} 
    }
    distances[distances.len()-1][distances[0].len()-1]
}

/// Recursive DFS visiting only valid paths
/// This is too slow
fn explore(data: &Vec<Vec<i32>>, v: &Vec<Vec<bool>>, i: i32, j: i32) -> i32 {
    let mut visited = v.to_owned();
    // Paths from this point costs
    let mut costs: Vec<i32> = vec![];
    // If this is the final point (eg. right bottom corner) return move cost
    if i == data.len() as i32 && j == data[i as usize].len() as i32 {return data[i as usize][j as usize];};
    // Else, mark as visited
    visited[i as usize][j as usize] = true;
    // If not at the end, pass control to neighbours
    for n in NEIGHBOURS {
        if i+n.0 >= 0 && j+n.1 >= 0 && i+n.0 < data.len() as i32 && j+n.1 < data[0].len() as i32 {
            if !visited[(i+n.0) as usize][(j+n.1) as usize]{
                costs.push(explore(&data, &visited, i+n.0, j+n.1));
            }
        }
    }
    // Return sum of future moves and this element
    if let Some(new) = costs.iter().min() {
        return data[i as usize][j as usize] + new;
    } else {
        return data[i as usize][j as usize]
    }
}

/// Parse input and return 2D table of costs
fn parse_input(filename: &str) -> io::Result<Vec<Vec<i32>>>{
    let mut data: Vec<Vec<i32>> = vec![];
    // Define input parsing regex for instructions
    let re = Regex::new(r"([A-Z])([A-Z]) -> ([A-Z])").expect("Invalid regex!");
    match read_lines(filename){
        Result::Ok(lines) => {            // Parse input lines
            for line in lines {
                data.push(line.unwrap().chars().map(|x| x.to_digit(10).expect("Not a number!") as i32).collect())
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