use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NEIGHBOURS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    if let Ok(data) = parse_input("./input") {
        println!("PART ONE");
        println!("Cost: {}", dijkstra(&data, 1));
        println!("PART TWO");
        println!("Cost: {}", dijkstra(&data, 5));
    };
}

/// Dijkstra's algorithm
fn dijkstra(data: &Vec<Vec<i32>>, inflation: i32) -> i32 {
    let size = (data.len()*inflation as usize, data[0].len()*inflation as usize);

    // Create a matrix of distances
    let mut distances: Vec<Vec<i32>> = vec![vec![i32::MAX; size.1]; size.0];
    distances[0][0] = 0;

    // Smarter queue
    let mut visited: Vec<Vec<bool>> = vec![vec![false; size.1]; size.0];
    let mut queue: Vec<(usize, usize)> = vec![];
    queue.push((0,0));

    // While nodes are still to be visited, repeat
    while queue.len() > 0 as usize {
        // Find current node with minimal distance
        let mut dist = i32::MAX;
        let mut current: (usize, usize) = (0, 0);
        for node in &queue {
            if distances[node.0][node.1] < dist {
                dist = distances[node.0][node.1];
                current = *node;
            }
        }

        // Update distances of the neighbour nodes, if needed, and add them to the queue
        for neighbour in unvisited_neighbours(current, &visited) {
            let tmp = distances[current.0][current.1] + 
                        get_data(&data, neighbour.0, neighbour.1);
            if tmp < distances[neighbour.0][neighbour.1] {
                distances[neighbour.0][neighbour.1] = tmp;
            }
            // Add neighbour to the queue
            if !queue.contains(&neighbour) {queue.push(neighbour);}
        }

        // Remove current node from unvisited and mark current visited
        queue.remove(*&queue.iter().position(|x| *x == current).unwrap());
        visited[current.0][current.1] = true;

        // If the destination node was visited, break -> we win
        if current == (size.0-1, size.1-1) {break;} 
    }
    distances[distances.len()-1][distances[0].len()-1]
}

/// Get data. Takes data inflation for part 2 into account
fn get_data(data: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    // Cast all points to i32 and calculate original data size
    let i: i32 = i as i32;
    let j: i32 = j as i32;
    let orig = (data.len() as i32, data[0].len() as i32);
    // A clever way to inflate the data as described by the task
    ((data[(i%orig.0) as usize][(j%orig.1) as usize] + i/orig.0 + j/orig.1)-1)%9 + 1
}

/// Returns a vector of viable neighbours for Dijkstra. Eliminates invalid idxs and visited
/// neighbours
fn unvisited_neighbours(point: (usize, usize), visited: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    let size = (visited.len(), visited[0].len());

    for neighbour in NEIGHBOURS {
        // Generate next neighbour according to direction vectors
        let n: (i32, i32) = (point.0 as i32 + neighbour.0, point.1 as i32 + neighbour.1);
        // Check if valid
        if n.0 >= 0 && n.1 >= 0 && n.0 < size.0 as i32 && n.1 < size.1 as i32{
            // Check if not visited
            if !visited[n.0 as usize][n.1 as usize] {
                neighbours.push((n.0 as usize, n.1 as usize));
            }
        }
    }
    neighbours
}

/// Parse input and return 2D table of costs
fn parse_input(filename: &str) -> io::Result<Vec<Vec<i32>>>{
    let mut data: Vec<Vec<i32>> = vec![];
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