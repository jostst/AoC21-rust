use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    // Define variables
    let mut num: i32 = 0;
    let mut sum: [i32; 12] = [0; 12];
    let mut data: Vec<[i32; 12]> = Vec::new();

    match read_lines("./input"){
        Result::Ok(lines) => {
            // Calculate sum of individual elements
            for line in lines {
                let string = line.expect("Need a line");
                // Define temporary  structure
                let mut tmp: [i32; 12] = [0; 12];

                // Iterate over all the numbers
                for i in 0..12 {
                    let n = &string[i..i+1].parse::<i32>().expect("Cannot parse");
                    sum[i] += n;
                    tmp[i] = *n;
                }
                num += 1;
                
                // Push current tmp data to vector of all values
                data.push(tmp);
            }

            // Calculate gamma and epsilon rates
            let gamma = sum
                .iter()
                .map(|x| 2*x/(num))
                .collect::<Vec<i32>>();
            let epsilon = gamma
                .iter()
                .map(|x| 1 - x)
                .collect::<Vec<i32>>();
            let gammai = to_dec(&gamma);
            let epsiloni = to_dec(&epsilon);

            // Iterate over all 12 numbers to obtain the o2 and co2 rates
            let mut candidates_o2 = data.clone();
            let mut candidates_co2 = data.clone();
            for i in 0..12 {
                // Filter vectors
                if candidates_o2.len() > 1 {
                    let ones_o2: i32 = candidates_o2
                        .iter()
                        .fold(0, |acc, &b| acc + b[i]);
                    let t = if 2*ones_o2 >= candidates_o2.len() as i32 {1} else {0};
                    candidates_o2 = candidates_o2
                        .into_iter()
                        .filter(|x| x[i] == t)
                        .collect();
                };
                if candidates_co2.len() > 1 {
                    let ones_co2: i32 = candidates_co2
                        .iter()
                        .fold(0, |acc, &b| acc + b[i]);
                    let t = if 2*ones_co2 < candidates_co2.len() as i32 {1} else {0};
                    candidates_co2 = candidates_co2
                        .into_iter()
                        .filter(|x| x[i] == t)
                        .collect();
                };
            }

            // Calculate environment rates
            let gen = candidates_o2[0].to_vec();
            let scr = candidates_co2[0].to_vec();
            let geni = to_dec(&gen);
            let scri = to_dec(&scr);

            println!("Gamma rate: \t\t{:?} or {}", gamma, gammai);
            println!("Epsilon rate: \t\t{:?} or {}", epsilon, epsiloni);
            println!("Power (gr*er): \t\t{}", epsiloni*gammai);

            println!("O2 generator rating: \t{:?} or {}", gen, geni);
            println!("CO2 scrubber rating: \t{:?} or {}", scr, scri);
            println!("Life support rating: \t{}", geni * scri);
        },
        Result::Err(e) => {
            println!("File error!");
            println!("{:?}", e);
        },
    }
}

fn to_dec(binary: &Vec<i32>) -> u32 
{
    binary.iter().fold(0, |acc, &b| acc*2 + b  as u32)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}