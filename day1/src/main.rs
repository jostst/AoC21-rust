use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut prev = 0;
    let mut larger = -1;
    let mut i = 0;
    let mut largerf = -1;
    let mut prevf = 0;
    let mut window: [i32; 3] = [0; 3];

    // File input must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(depth) = line {
                // Get current depth
                let current = depth.parse::<i32>().unwrap();
                
                // With window
                if i < 2{
                    // in first two iterations just fill the window
                    window[i%3] = current;
                } else {
                    window[i%3] = current;
                    let currentf = window[0] + window[1] + window[2];
                    if currentf > prevf {
                        largerf += 1;
                    }
                    prevf = currentf;
                }
                i += 1;

                // Without window
                if current > prev{
                    larger += 1;
                }
                prev = current;
            }
        }
    }

    println!("Result is: {}", larger);
    println!("Filtered result is: {}", largerf);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}