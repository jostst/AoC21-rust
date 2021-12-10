use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match read_lines("./input"){
        // Parse input lines
        Result::Ok(lines) => {
            let solution = solution(lines);
            println!("PART ONE");
            println!("Score is: {}", solution.0);
            println!("PART TWO");
            println!("Score is: {}", solution.1)
        },
        Result::Err(e) => {
            println!("File error!");
            println!("{:?}", e);
        },
    };
}

fn is_opening(c: &char) -> bool {
    if c == &'{' || c == &'(' || c == &'[' || c == &'<' {true} else {false}
}

fn is_pair(a: &char, b: &char) -> bool {
    if a == &'{' && b == &'}' {return true}
    if a == &'[' && b == &']' {return true}
    if a == &'(' && b == &')' {return true}
    if a == &'<' && b == &'>' {return true}
    false
}

fn score_mismatch(c: &char) -> i32 {
    if c == &')' {return 3}
    if c == &']' {return 57}
    if c == &'}' {return 1197}
    if c == &'>' {return 25137}
    0
}

fn score_complete(c: &char) -> u64 {
    if c == &'(' {return 1}
    if c == &'[' {return 2}
    if c == &'{' {return 3}
    if c == &'<' {return 4}
    0
}

fn score_remaining(rem: &Vec<char>) -> u64 {
    let mut score: u64 = 0;
    for c in rem.iter().rev() {
        score = 5*score + score_complete(&c);
    }
    score
}

fn solution(lines: io::Lines<io::BufReader<File>>) -> (i32, u64) {
    let mut score1: i32 = 0;
    let mut scores2: Vec<u64> = Vec::new();
    for line in lines {
        let ln: Vec<char> = line.expect("Not a valid line").chars().collect();
        let mut stack: Vec<char> = Vec::new();
        let mut score_new = 0;
        for c in ln {
            if is_opening(&c) { stack.push(c); }
            else if is_pair(&stack.pop().unwrap(), &c) { }
            else { score_new = score_mismatch(&c); break;}
        }
        score1 += score_new;

        // If line is just incomplete, score_new will be 0 at the end and
        // some elements will remain in the stack
        if score_new == 0 && stack.len() > 0{
            scores2.push(score_remaining(&stack));
        }
    }
    scores2.sort();
    (score1, scores2[scores2.len()/2])
}

/// This is a buffered file reader which opens a file at filename and resturns io::Result
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}