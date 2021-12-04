use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // We will read the data into these two mutable variables
    static SIZE: u32 = 5;

    match read_lines("./input"){
        Result::Ok(lines) => {
            // Parse input lines
            let (nums, mut boards) = parse_input(lines, SIZE);

            // Now that we have the data, let's play BINGO!
            part_one(&nums, &mut boards);
            part_two(&nums, &mut boards);
            
        },
        Result::Err(e) => {
            println!("File error!");
            println!("{:?}", e);
        },
    };
}

// This is the input parser
fn parse_input(lines: io::Lines<io::BufReader<File>>, size: u32) -> (Vec<u32>, Vec<BingoBoard>) {
    let mut nums: Vec<u32> = Vec::<u32>::new();
    let mut boards: Vec<BingoBoard> = Vec::<BingoBoard>::new();

    // Create temporary board that we will read line by line
    let mut board = BingoBoard::new(size);
    let mut idx = 0; // This is the new board row index

    // Iterate through file and read file into structures
    for (i, line) in lines.enumerate() {
        // Read current line into string
        let string = line.expect("No line!");

        // For first line we fill the guess numbers in nums
        if i == 0 {
            string
                .split(",")
                .for_each(|x| nums.push(x.parse::<u32>().expect("Not a number")));
            continue;
        }

        // Completely skip the second line
        if i == 1 {continue;}
        
        // If this is an empty line (not the first), push the board and initialize a new one
        if string.chars().count() == 0 && i != 1{
            board.order();
            boards.push(board);
            board = BingoBoard::new(size);
            idx = 0;
            continue;
        }

        // If we got to here, fill the board!
        string
            .split_whitespace()
            .enumerate()
            .for_each(|(j, x)| {
                board.add(BingoNumber{
                    x: j,
                    y: idx,
                    num: x.parse::<u32>().expect("Not a number"),
                    ..Default::default()
                })
            });
        // Increase the row index
        idx += 1;
    }
    return (nums, boards);
}

// This is PART ONE
// Determine, which board wins first
fn part_one(nums: &Vec<u32>, boards: &mut Vec<BingoBoard>) -> (){
    // Part one
    println!("PART ONE:");

    let mut score: u32 = 0;
    let mut num: u32 = 0;

    'outer: for n in nums {
        for i in 0..boards.len() {
            boards[i].call(*n);
            if boards[i].test() {
                score = boards[i].score();
                num = *n;
                break 'outer;
            }
        }
    }

    println!("Sum is: {} and number is {}", score, num);
    println!("Answer is thus: {}", score*num);
}

// This is PART TWO
// Determine, which board wins last
fn part_two(nums: &Vec<u32>, boards: &mut Vec<BingoBoard>) -> (){
    // Part two
    println!("PART TWO");

    let mut score = 0;
    let mut num = 0;

    for n in nums {
        let mut elim: Vec<usize> = Vec::new();
        for i in 0..boards.len(){
            boards[i].call(*n);
            if boards[i].test(){
                elim.push(i);
                score = boards[i].score();
                num = *n;
            }
        };

        elim.iter().rev().for_each(|&x| {
            boards.remove(x);
        });
    }

    println!("Sum is: {} and number is {}", score, num);
    println!("Answer is thus: {}", score*num);
}

// This is the data of a single number on the board
#[derive(Default)]
struct BingoNumber{
    num: u32,
    x: usize,
    y: usize,
    xed: bool,
}

// This are methods for the board
trait Bingo{
    fn call(&mut self, c: u32) -> ();
    fn test(&self) -> bool;
    fn score(&self) -> u32 ;
    fn add(&mut self, num: BingoNumber) -> ();
    fn order(&mut self) -> ();
}

// This is the data of the whole board
struct BingoBoard{
    n: u32,
    rows: Vec<u32>,
    cols: Vec<u32>,
    nums: Vec<BingoNumber>,
}

// BingoBoard specific methods
impl BingoBoard {
    pub fn new(m: u32) -> Self {
        Self{
            n: m,
            rows: vec![0; m.try_into().expect("Too large board")],
            cols: vec![0; m.try_into().expect("Too large board")],
            nums: Vec::<BingoNumber>::new(),
        }
    }
}

// Implement Bingo on BingoBoard
impl Bingo for BingoBoard {
    fn call(&mut self, c: u32) -> (){
        // TODO: this is ugly and slow O(n2)
        for i in 0..self.nums.len() {
            if c == self.nums[i].num {
                if !self.nums[i].xed{
                    self.rows[self.nums[i].y] += 1;
                    self.cols[self.nums[i].x] += 1;
                    self.nums[i].xed = true
                }
            }
        }
    }
    fn test(&self) -> bool{
        for i in 0..self.n as usize {
            if self.rows[i] == self.n || self.cols[i] == self.n {
                return true
            }
        }
        false
    }
    fn score(&self) -> u32{
        let mut sum = 0;
        for bn in &self.nums {
            if !bn.xed {
                sum += bn.num;
            }
        }
        sum
    }
    fn add(&mut self, num: BingoNumber) -> (){
        self.nums.push(num)
    }
    fn order(&mut self) -> (){
        self.nums.sort_by(|a, b| a.num.cmp(&b.num));
    }
}
    
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}