use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data) = parse_input("./input") {
        println!("PART ONE");
        println!("Risk: {}", part_one(&data));
        println!("PART TWO");
        println!("Hash: {}", part_two(&data));
    }
}

/// Check if the point is minimum. Use get_safe function that returns 9 for out of bound
/// values so that the comparison resolves to true and decision goes to other conditions
fn is_minimum(data: &Vec<Vec<i32>>, i: i32, j: i32) -> bool{
    data[i as usize][j as usize] < get_safe(&data, i-1, j) && 
    data[i as usize][j as usize] < get_safe(&data, i+1, j) && 
    data[i as usize][j as usize] < get_safe(&data, i, j+1) && 
    data[i as usize][j as usize] < get_safe(&data, i, j-1)
}

/// Get value from the 2D vector. If index is out of bounds, return 9 (max value).
fn get_safe(data: &Vec<Vec<i32>>, i: i32, j:i32) -> i32 {
    if i > 0 || j > 0 {
        if let Some(val1) = data.get(i as usize){
            if let Some(val2) = val1.get(j as usize) {
                *val2
            }
            else { 9 }
        } else { 9 }
    } else { 9 }
}

/// Iterate over the whole table and check for each point if it is minimum
/// using the aux function is_minimum
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
    let mut sizes: Vec<i32> = Vec::new();
    let mut tmp = data.clone();
    for i in 0..=data.len()-1 {
        for j in 0..=data[0].len()-1 {
            if  is_minimum(&data, i as i32, j as i32){
                sizes.push(grow(data, &mut tmp, i as i32, j as i32));
            }
        }
    }
    sizes.sort_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
}

fn grow(o: &Vec<Vec<i32>>, d: &mut Vec<Vec<i32>>, i:i32, j:i32) -> i32 {
    let mut c = (0, 0, 0, 0, 0);
    if  d[i as usize][j as usize] !=-1 {c.0=1;}
    d[i as usize][j as usize] = -1;
    if get_safe(&o, i + 1, j) != 9 && get_safe(&o, i + 1, j) > get_safe(&o, i, j) {c.1 = grow(o, d, i+1, j);}
    if get_safe(&o, i - 1, j) != 9 && get_safe(&o, i - 1, j) > get_safe(&o, i, j) {c.2 = grow(o, d, i-1, j);}
    if get_safe(&o, i, j + 1) != 9 && get_safe(&o, i, j + 1) > get_safe(&o, i, j) {c.3 = grow(o, d, i, j+1);}
    if get_safe(&o, i, j - 1) != 9 && get_safe(&o, i, j - 1) > get_safe(&o, i, j) {c.4 = grow(o, d, i, j-1);}
    c.0 + c.1 + c.2 + c.3 + c.4
}

/// This returns a matrix of depths, as given in the input filename and wrapped in Result
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