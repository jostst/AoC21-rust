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
    let data = datai.clone();
    data
        .iter()
        .fold(0, |acc, x| acc + x.clone().decode_value())
}

/// This structure holds all the data for a single display
#[derive(Clone)]
struct Display {
    combinations: Vec<String>,
    outputs: Vec<String>,
}

/// Methods enable creation and decoding of a single display
impl Display {
    pub fn new(cmbs: Vec<String>, outpts: Vec<String>) -> Self {
        Self {
            combinations: cmbs.to_owned(),
            outputs: outpts.to_owned(),
        }
    }

    fn decode_key(self) -> HashMap<String,i32>{
        let mut rev: HashMap<i32,String> = HashMap::new();
        let mut key: HashMap<String,i32> = HashMap::new();

        // First, decode "simple" numbers
        for comb in self.combinations.iter(){
            if comb.len() == 2 { rev.insert(1, comb.to_string());}
            if comb.len() == 3 { rev.insert(7, comb.to_string());}
            if comb.len() == 4 { rev.insert(4, comb.to_string());}
            if comb.len() == 7 { rev.insert(8, comb.to_string());}
        }

        // Store key for simple values
        key.insert(Display::sort_string(rev.get(&1).expect("No value")), 1);
        key.insert(Display::sort_string(rev.get(&4).expect("No value")), 4);
        key.insert(Display::sort_string(rev.get(&7).expect("No value")), 7);
        key.insert(Display::sort_string(rev.get(&8).expect("No value")), 8);

        // Construct remaining keys with 6 values
        for comb in self.combinations.iter(){
            if comb.len() == 6 {
                // 9 has 6 segments and contains all letters from 4
                if comb.chars().fold(0, |acc, c| if rev.get(&4).expect("error").to_string().contains(c) {acc + 1} else {acc}) == 4 {
                    rev.insert(9, comb.to_string());
                    key.insert(Display::sort_string(comb), 9);
                }
                // 0 has 6 segments and is missing one of the letters from 4 but has all segments from 1
                if comb.chars().fold(0, |acc, c| if rev.get(&4).expect("error").to_string().contains(c) {acc + 1} else {acc}) == 3 &&
                comb.chars().fold(0, |acc, c| if rev.get(&1).expect("error").to_string().contains(c) {acc + 1} else {acc}) == 2{
                    rev.insert(0, comb.to_string());
                    key.insert(Display::sort_string(comb), 0);
                }
                // 6 has 6 segments and is missing one of the letters from 1
                if comb.chars().fold(0, |acc, c| if rev.get(&1).expect("error").to_string().contains(c) {acc + 1} else {acc}) == 1 {
                    rev.insert(6, comb.to_string());
                    key.insert(Display::sort_string(comb), 6);
                }
            }
            if comb.len() == 5 {
                // 3 has 5 segments and contains all letters from 1
                if comb.chars().fold(0, |acc, c| if rev.get(&1).expect("error").to_string().contains(c) {acc + 1} else {acc}) == 2 {
                    rev.insert(3, comb.to_string());
                    key.insert(Display::sort_string(comb), 3);
                }
            }
        }

        // 2, 5 remain for the final pass, since we differentiate them based on 6!
        for comb in self.combinations.iter(){
            if comb.len() == 5 {
                // 5 is missing only one element from 6
                if comb.chars().fold(0, |acc, c| if rev.get(&6).expect("error").to_string().contains(c) {acc + 1} else {acc}) == 5 &&
                rev.len() >= 8{
                    rev.insert(5, comb.to_string());
                    key.insert(Display::sort_string(comb), 5);
                }
                // 2 is missing two elements from 6
                if comb.chars().fold(0, |acc, c| if rev.get(&6).expect("error").to_string().contains(c) {acc + 1} else {acc}) == 4 &&
                rev.len() >= 8 &&
                comb.chars().fold(0, |acc, c| if rev.get(&1).expect("error").to_string().contains(c) {acc + 1} else {acc}) != 2{
                    rev.insert(2, comb.to_string());
                    key.insert(Display::sort_string(comb), 2);
                }
            }
        }
        key
    }

    pub fn decode_value(self) -> i32 {
        let key = self.clone().decode_key();
        let mut out: i32 = 0;
        for i in 0..4{
            let digit = key.get(&Display::sort_string(&self.outputs[i])).expect("not present");
            out += digit*(i32::pow(10, 3-i as u32));
        }
        out
    }

    pub fn sort_string(input: &String) -> String {
        let mut tmp: Vec<char> = input[..].chars().collect();
        tmp.sort_by(|a, b| b.cmp(a));
        tmp.into_iter().collect()
    }

    pub fn count_simple(&self) -> i32 {
        self.outputs
            .iter()
            .fold(0, |acc, x| if x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7 {acc + 1} else {acc})
    }
}

/// This is a buffered file reader which opens a file at filename and resturns io::Result
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}