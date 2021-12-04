use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // We will read the data into these two mutable variables
    let mut nums: Vec<u32> = Vec::<u32>::new();
    let mut boards: Vec<BingoBoard> = Vec::<BingoBoard>::new();

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

// This is the data of a single number on the board
struct BingoNumber{
    num: u32,
    x: usize,
    y: usize,
    xed: bool,
}

// Default values for the BingoNumber struct
impl Default for BingoNumber {
    fn default() -> BingoNumber {
        BingoNumber {
            num: 0,
            x: 0,
            y: 0,
            xed: false,
        }
    }
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
        // TODO this is ugly and slow O(n2)
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