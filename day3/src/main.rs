use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    // Define variables
    let mut num: i32 = 0;
    let mut sum: [i32; 12] = [0; 12];

    match read_lines("./input"){
        Result::Ok(lines) => {
            // Calculate sum of individual elements
            for line in lines {
                let string = line.expect("Need a line");
                for i in 0..12 {
                    sum[i] += &string[i..i+1].parse::<i32>().expect("Cannot parse");
                }
                num += 1;
            }

            // Calculate gamma and epsilon rates
            let gamma: Vec<i32> = sum.to_vec().iter().map(|x| x/(num/2)).collect();
            let epsilon: Vec<i32> = gamma.iter().map(|x| 1 - x).collect();
            let gammai: u32 = gamma.iter().fold(0, |acc, &b| acc*2 + b as u32);
            let epsiloni: u32 = epsilon.iter().fold(0, |acc, &b| acc*2 + b as u32);

            println!("Sums: {}, {}, {}, {}, {}", sum[0], sum[1], sum[2], sum[3], sum[4]);
            println!("Num: {}", num);
            println!("Gamma rate: {:?} or {}", gamma, gammai);
            println!("Epsilon rate: {:?} or {}", epsilon, epsiloni);
            println!("Power (gr*er): {}", epsiloni*gammai);
        },
        Result::Err(e) => {
            println!("File error!");
            println!("{:?}", e);
        },
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}