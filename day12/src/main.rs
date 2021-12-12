// Test solutions are:
//  - test1: 10 paths
//  - test2: 19 paths
//  - test3: 226 paths

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    if let Ok(data) = parse_input("./input") {
        println!("PART ONE:");
        println!("Paths found: {}", part_one(&data));
        println!("PART TWO:");
        println!("Paths found: {}", part_two(&data));
    }
}

fn part_two(data: &HashMap<String, Vec<String>>) -> i32 {
    let mut paths = vec![vec!["start".to_string()]];
    find_paths_two(&mut paths, &data);
    //for path in paths.clone() {println!("{:?}", path);};
    paths.len() as i32
}

fn find_paths_two(paths: &mut Vec<Vec<String>>, data: &HashMap<String, Vec<String>>) -> (){
    loop {
        let mut changes = 0;
        let mut i = 0;
        // Create a copy of previous iteration and empty paths
        let tmp = paths.clone();
        paths.retain(|_| false);

        // Now iterate over tmp and expand paths. Write new paths into paths and 
        // increment counter
        for path in tmp.clone() {
            // Check if we are at the end, in this case, just store path into output do not do anything
            if path.clone().last().unwrap() == "end" {
                paths.push(path);
            } else {
                let points: &Vec<String> = data.get(path.clone().last().unwrap()).unwrap();
                // count existing repetitions
                let mut repeat_old = 0;
                for pt in path.clone() {
                    repeat_old += path.clone().iter().fold(0, |acc, x| if x == &pt && x!="start" && x.chars().all(|c| matches!(c, 'a'..='z')) {acc + 1} else {acc});
                }
                repeat_old -= path.clone().iter().filter(|&x| x!="start" && x.chars().all(|c| matches!(c, 'a'..='z'))).count();
                for point in points.clone(){
                    // add "future" repetitions if any
                    let repeat_new= path.clone().iter().fold(0, |acc, x| if x == &point && x!="start" && x.chars().all(|c| matches!(c, 'a'..='z')) {acc + 1} else {acc});
                    if  repeat_new + repeat_old <= 2 && point != "start" {
                        changes += 1;
                        let mut new_path = path.clone();
                        new_path.push(point);
                        paths.push(new_path);
                    }
                }
            }
            i += 1;
        }

        // Break if no changes done in this iteration and place tmp back into paths
        if changes == 0 {
            //paths.append(&mut tmp);
            break;
        };
    }
}

fn part_one(data: &HashMap<String, Vec<String>>) -> i32 {
    let mut paths = vec![vec!["start".to_string()]];
    find_paths(&mut paths, &data);
    //for path in paths.clone() {println!("{:?}", path);};
    paths.len() as i32
}

fn find_paths(paths: &mut Vec<Vec<String>>, data: &HashMap<String, Vec<String>>) -> (){
    loop {
        let mut changes = 0;
        // Create a copy of previous iteration and empty paths
        let tmp = paths.clone();
        paths.retain(|_| false);

        // Now iterate over tmp and expand paths. Write new paths into paths and 
        // increment counter
        for path in tmp.clone() {
            // Check if we are at the end, in this case, just store path into output do not do anything
            if path.clone().last().unwrap() == "end" {
                paths.push(path);
            } else {
                let points: &Vec<String> = data.get(path.clone().last().unwrap()).unwrap();
                for point in points.clone(){
                    // check that point is not all lowercase and already in the path
                    if path.clone().iter().fold(0, |acc, x| if x == &point && x.chars().all(|c| matches!(c, 'a'..='z')) {acc + 1} else {acc}) == 0 {
                        changes += 1;
                        let mut new_path = path.clone();
                        new_path.push(point);
                        paths.push(new_path);
                    }
                }
            }
        }

        // Break if no changes done in this iteration and place tmp back into paths
        if changes == 0 {
            //paths.append(&mut tmp);
            break;
        };
    }
}

/// This returns a hash map that has all the caverns as keys and caverns it connects to 
/// in a vector of strings stored as the value of the key
fn parse_input(filename: &str) -> io::Result<HashMap<String, Vec<String>>>{
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    match read_lines(filename){
        Result::Ok(lines) => {            // Parse input lines
            for line in lines {
                let nl = line.expect("Invalid line")
                        .split("-")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                if let Some(old) = data.get_mut(&nl[0]) {
                    old.push(nl[1][..].to_string());
                } else {
                    data.insert(nl[0][..].to_string(), vec![nl[1][..].to_string()]);
                }
                if let Some(old) = data.get_mut(&nl[1]) {
                    old.push(nl[0][..].to_string());
                } else {
                    data.insert(nl[1][..].to_string(), vec![nl[0][..].to_string()]);
                }
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