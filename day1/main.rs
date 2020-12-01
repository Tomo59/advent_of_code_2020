use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn p1(input_ref: &Vec<i32>) -> i32 {
    for i in input_ref {
        for j in input_ref {
            if (i+j) == 2020 {
                return i * j;
            }
        }
    }
    panic!("Didn't find answer to p1");
}

fn p2(input_ref: &Vec<i32>) -> i32 {
    for i in input_ref {
        for j in input_ref {
            for k in input_ref {
                if (i+j+k) == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    panic!("Didn't find answer to p2");
}

fn main() {
    let mut input: Vec<i32> = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(i) = line {
                input.push(i.parse().unwrap());
                //println!("{}", ip);
            }
        }
    }


    println!("part1: {}", p1(&input));
    println!("part2: {}", p2(&input));
}
