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

fn get_data(data: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    let orig = (data.len() as i32, data[0].len() as i32);
    ((data[(i%orig.0) as usize][(j%orig.1) as usize] + i/orig.0 + j/orig.1)-1)%9 + 1
}


/// Dijkstra's algorithm
fn dijkstra(data: &Vec<Vec<i32>>, inflation: i32) -> i32 {
    let size = (data.len()*inflation as usize, data[0].len()*inflation as usize);

    // Create a matrix of distances
    let mut distances: Vec<Vec<i32>> = vec![vec![i32::MAX; size.1]; size.0];
    distances[0][0] = 0;

    // Smarter queue
    let mut visited: Vec<Vec<bool>> = vec![vec![false; size.1]; size.0];
    let mut queue: Vec<(i32, i32)> = vec![];
    queue.push((0,0));

    // While nodes are still to be visited, repeat
    while queue.len() > 0 as usize {
        // Find current node with minimal distance
        let mut dist = i32::MAX;
        let mut current: (i32, i32) = (0, 0);
        for node in &queue {
            if distances[node.0 as usize][node.1 as usize] < dist {
                dist = distances[node.0 as usize][node.1 as usize];
                current = *node;
            }
        }

        // Update distances of the neighbour nodes, if needed, and add them to the queue
        for neighbour in NEIGHBOURS {
            let n = (current.0+neighbour.0, current.1+neighbour.1);
            // Verify validity
            if n.0 >= 0 && n.1 >= 0 && n.0 < size.0 as i32 && n.1 < size.1 as i32{
                // Check if not visited
                if !visited[n.0 as usize][n.1 as usize]{
                    let tmp = distances[current.0 as usize][current.1 as usize] + 
                                get_data(&data, n.0, n.1);
                    if tmp < distances[n.0 as usize][n.1 as usize] {
                        distances[n.0 as usize][n.1 as usize] = tmp;
                    }
                    // Add neighbour to the queue
                    if !queue.contains(&n) {queue.push(n);}
                }
            }
        }

        // Remove current node from unvisited and mark current visited
        queue.remove(*&queue.iter().position(|x| *x == current).unwrap());
        visited[current.0 as usize][current.1 as usize] = true;

        // If the destination node was visited, break -> we win
        if current == ((size.0-1) as i32, (size.1-1) as i32) {break;} 
    }
    distances[distances.len()-1][distances[0].len()-1]
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