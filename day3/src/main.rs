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
                data.push(tmp);
            }

            // Calculate gamma and epsilon rates
            let gamma: Vec<i32> = sum.to_vec().iter().map(|x| 2*x/(num)).collect();
            let epsilon: Vec<i32> = gamma.iter().map(|x| 1 - x).collect();
            let gammai: u32 = gamma.iter().fold(0, |acc, &b| acc*2 + b as u32);
            let epsiloni: u32 = epsilon.iter().fold(0, |acc, &b| acc*2 + b as u32);

            // Iterate over all 12 numbers to obtain the o2 and co2 rates
            let mut candidates_o2 = data.clone();
            let mut candidates_co2 = data.clone();
            for i in 0..12 {
                // Filter vectors
                if candidates_o2.len() > 1 {
                    let mut t = 0;
                    let ones_o2: i32 = candidates_o2
                        .iter()
                        .fold(0, |acc, &b| acc + b[i]);
                    if 2*ones_o2 >= candidates_o2.len() as i32 {t = 1;};
                    candidates_o2 = candidates_o2
                        .into_iter()
                        .filter(|x| x[i] == t)
                        .collect();
                };
                if candidates_co2.len() > 1 {
                    let mut t = 0;
                    let ones_co2: i32 = candidates_co2
                        .iter()
                        .fold(0, |acc, &b| acc + b[i]);
                    if 2*ones_co2 < candidates_co2.len() as i32 {t = 1;};
                    candidates_co2 = candidates_co2
                        .into_iter()
                        .filter(|x| x[i] == t)
                        .collect();
                };
            }
            let gen = candidates_o2[0];
            let scr = candidates_co2[0];
            let geni: u32 = gen.iter().fold(0, |acc, &b| acc*2 + b as u32);
            let scri: u32 = scr.iter().fold(0, |acc, &b| acc*2 + b as u32);

            println!("Gamma rate: {:?} or {}", gamma, gammai);
            println!("Epsilon rate: {:?} or {}", epsilon, epsiloni);
            println!("Power (gr*er): {}", epsiloni*gammai);

            println!("O2 generator rating: {:?} or {}", gen, geni);
            println!("CO2 scrubber rating: {:?} or {}", scr, scri);
            println!("Life support rating: {}", geni * scri);
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