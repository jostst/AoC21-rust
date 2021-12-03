use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match read_lines("./input"){
        Result::Ok(lines) => {
            // Iterate through file
            
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