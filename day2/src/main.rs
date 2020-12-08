use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate regex;

use regex::Regex;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub struct Pass {
    min: usize,
    max: usize,
    letter: char,
    password: String
}

fn valid_password_p1(p: &Pass) -> bool {
  let number_letter: usize = p.password.chars().filter(|&c| c == p.letter).count();
  return number_letter <= p.max && number_letter >= p.min;
}

fn valid_password_p2(p: &Pass) -> bool {
  let letter1: char = p.password.chars().nth(p.min-1).unwrap();
  let letter2: char = p.password.chars().nth(p.max-1).unwrap();
  return (letter1 == p.letter) ^ (letter2 == p.letter);
}

fn main() {
    let mut input: Vec<Pass> = Vec::new();
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(i) = line {
                let cap = re.captures(&i).unwrap();
                input.push(Pass {
                    min: cap.get(1).unwrap().as_str().parse().unwrap(),
                    max: cap.get(2).unwrap().as_str().parse().unwrap(),
                    letter: cap.get(3).unwrap().as_str().parse().unwrap(),
                    password: cap.get(4).unwrap().as_str().parse().unwrap()
                })
            }
        }
    }


    println!("part1: {}", input.iter().filter(|&n| valid_password_p1(n)).count());
    println!("part2: {}", input.iter().filter(|&n| valid_password_p2(n)).count());
}
