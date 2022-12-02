use std::env;
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

/* 
    Opponent: 
    A -> Rock
    B -> Paper
    C -> Scissors
    My response:
    X -> Rock
    Y -> Paper
    Z -> Scissors

    total score = sum of score for each round

    score for a single run:
    1 for rock
    2 for Paper
    3 for Scissors
    +
    0 if lost
    3 if draw
    6 if won

    strategy:
    Win 
    loose
    draw


*/
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
 }

fn get_solution_part1(path: &str) -> i32 {
    let mut score = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            // read line by line and store letters in vector without white space
            let words: Vec<_> = line.unwrap().chars().filter(|c| !c.is_whitespace()).collect();
            // draw
            score += match words[..] {
                ['A','X'] => 3 + 1,
                ['A','Y'] => 6 + 2,
                ['A','Z'] => 3,
                ['B','X'] => 1,
                ['B','Y'] => 3 + 2,
                ['B','Z'] => 3 + 6,
                ['C','X'] => 6 + 1,
                ['C','Y'] => 2,
                ['C','Z'] => 3 + 3,
                _ => {
                    println!("not a valid combo");
                    0
                },
            }
                
        }
    }
    
    return score;
}

fn main() {
    let part = match env::var("part") {
       Ok(val) => val,
       Err(_e) => "part1".to_string(),
    };

    if part == "part2" {
        //println!("{}", get_solution_part1("input.txt"));
        println!("Part2");
    }else {
        println!("{}", get_solution_part1("input.txt"));
    }
}