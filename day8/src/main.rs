use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    if let Ok(data) = parse_input("./input") {
        println!("PART ONE");
        println!("{}", part_one(&data));

        println!("PART TWO");
        println!("{}", part_two(&data));
    }
}

/// This returns a vector of ages, as given in the input filename and wrapped in Result
fn parse_input(filename: &str) -> io::Result<Vec<Display>>{
    let mut data: Vec<Display> = Vec::new();
    match read_lines(filename){
        Result::Ok(lines) => {
            // Parse input lines
            for line in lines {
                let entries: Vec<String> = line.expect("Invalid line")
                                            .split(" ")
                                            .map(|x| x.to_string())
                                            .collect();
                data.push(Display::new(entries[0..10].to_vec(), entries[11..15].to_vec()));
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

/// Calculate verification hash for first part
fn part_one(datai: &Vec<Display>) -> i32{
    let data = datai.to_owned();
    data
        .iter()
        .fold(0, |acc, x| acc + x.count_simple())
}

/// Calculate verification hash for the second part
fn part_two(datai: &Vec<Display>) -> i32{
    let data = datai.to_owned();
    data
        .iter()
        .fold(0, |acc, x| acc + x.clone().decode_value())
}

/// This structure holds all the data for a single display
#[derive(Clone)]
struct Display {
    combinations: Vec<String>,
    outputs: Vec<String>,
    rev: HashMap<i32,String>,
    key: HashMap<String,i32>,
}

/// Methods enable creation and decoding of a single display
impl Display {
    pub fn new(cmbs: Vec<String>, outpts: Vec<String>) -> Self {
        Self {
            combinations: cmbs.to_owned(),
            outputs: outpts.to_owned(),
            rev: HashMap::new(),
            key: HashMap::new(),
        }
    }

    fn write_pair(&mut self, code: &String, num: i32) -> (){
        self.rev.insert(num, code.to_string());
        self.key.insert(Display::sort_string(code), num);
    }

    fn decode_key(&mut self){
        // First, decode "simple" numbers
        for comb in self.clone().combinations.iter(){
            if comb.len() == 2 { 
                self.write_pair(comb, 1);
            }
            if comb.len() == 3 {
                self.write_pair(comb, 7);
            }
            if comb.len() == 4 { 
                self.write_pair(comb, 4)
            }
            if comb.len() == 7 {
                self.write_pair(comb, 8);
            }
        }

        // Construct remaining keys with 6 values
        for comb in self.clone().combinations.iter(){
            if comb.len() == 6 {
                // 9 has 6 segments and contains all letters from 4
                if self.segment_mismatch(4, comb) == 0 {
                    self.write_pair(comb, 9);
                }
                // 6 has 6 segments and is missing one of the letters from 1
                if self.segment_mismatch(1, comb) == 1{
                    self.write_pair(comb, 6);
                }
                // 0 has 6 segments and is missing one of the letters from 4 but has all segments from 1
                if self.segment_mismatch(4, comb) == 1 && self.segment_mismatch(1, comb) == 0{
                    self.write_pair(comb, 0);
                }
            }
        }

        // 2, 5 remain for the final pass, since we differentiate them based on 6!
        for comb in self.clone().combinations.iter(){
            if comb.len() == 5 {
                // 3 has 5 segments and contains all letters from 1
                if self.segment_mismatch(1, comb) == 0{
                    self.write_pair(comb, 3);
                }
                // 5 is missing only one element from 6
                if self.segment_mismatch(6, comb) == 1 {
                    self.write_pair(comb, 5);
                }
                // 2 is missing two elements from 6 and one from 1
                if self.segment_mismatch(6, comb) == 2 && self.segment_mismatch(1, comb) == 1{
                    self.write_pair(comb, 2);
                }
            }
        }
    }

    fn decode_digit(&self, code: &String) -> i32{
        *self.key.get(&Display::sort_string(code)).expect("not present")
    }

    pub fn decode_value(&mut self) -> i32 {
        self.decode_key();
        let mut out: i32 = 0;
        for i in 0..4{
            let digit = self.decode_digit(&self.outputs[i]);
            out += digit*(i32::pow(10, 3-i as u32));
        }
        out
    }

    fn query_match(&self, reference: &i32, c: char) -> bool {
        self.rev
                .get(&reference)
                .expect("error")
                .to_string()
                .contains(c)
    }

    pub fn segment_mismatch(&self, reference: i32, input: &String) -> i32{
        let mtch = input
                        .chars()
                        .fold(0, |acc, c| if self.query_match(&reference, c) {acc + 1} else {acc});
        self.rev.get(&reference).expect("Error!").len() as i32 - mtch
    }

    pub fn sort_string(input: &String) -> String {
        let mut tmp: Vec<char> = input[..].chars().collect();
        tmp.sort_by(|a, b| b.cmp(a));
        tmp.into_iter().collect()
    }

    pub fn count_simple(&self) -> i32 {
        self.outputs
            .iter()
            .fold(0, |acc, x| if [2, 3, 4, 7].contains(&x.len()) {acc + 1} else {acc})
    }
}

/// This is a buffered file reader which opens a file at filename and resturns io::Result
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}