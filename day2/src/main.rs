use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    // Define coordinates - part one
    let mut x = 0; // Horizontal coordinate
    let mut z = 0; // Vertical coordinate. This is depth!

    // Define aim and coordinates - part two
    let mut x2 = 0;
    let mut z2 = 0;
    let mut aim = 0;

    // Define the parser regex
    let re = Regex::new(r"(up|down|forward) (\d+)").unwrap();

    match read_lines("./input"){
        Result::Ok(lines) => {
            // Iterate through lines
            for line in lines {
                let directive = line.unwrap();
                let cap = re.captures(&directive).unwrap();

                // This calculates the part one coordinates
                match &cap[1] {
                    "forward" => x += &cap[2].parse::<i32>().unwrap(),
                    "up" => z -= &cap[2].parse::<i32>().unwrap(),
                    "down" => z += &cap[2].parse::<i32>().unwrap(),
                    _ => (),
                };

                // This calculates the part two coordinates
                match &cap[1] {
                    "up" => aim -= &cap[2].parse::<i32>().unwrap(),
                    "down" => aim += &cap[2].parse::<i32>().unwrap(),
                    "forward" => {
                        let step = &cap[2].parse::<i32>().unwrap();
                        x2 += step;
                        z2 += step * aim;
                    },
                    _ => (),
                };
            }

            // Output - part one
            println!("PART ONE:");
            println!("x: {}, depth: {}", x, z);
            println!("product: {}", x*z);

            // Putput - part two
            println!("PART TWO:");
            println!("x {}, depth: {}", x2, z2);
            println!("product: {}", x2*z2);
        },
        Result::Err(e) => {
            println!("File error!");
            println!("{:?}", e);
        },
    };
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}