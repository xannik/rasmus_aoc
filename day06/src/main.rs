use std::collections::VecDeque;
use std::{env};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
//use arrayvec::ArrayVec;

/* fn generate_combinations() -> Vec<Vec<u8>> {
    let characters = "abcdefghijklmnopqrstuvwxzy";
    let combinations = characters.bytes().combinations(4).collect();
    return combinations;
} */

fn get_solution_part2(path: &str) -> i32 { 
    let mut index = 0;
    if let Ok(lines) = read_lines(path) {


       for line in lines {
            index = find_message(14, line.unwrap().as_bytes());
        }  
    }

    return index;
}

fn get_solution_part1(path: &str) -> i32 { 
    let mut index = 0;
    if let Ok(lines) = read_lines(path) {


       for line in lines {
            index = find_message(4, line.unwrap().as_bytes());
        }  
    }

    return index;
}

fn find_message(size: i32, stream: &[u8]) -> i32 {
    let mut unique: VecDeque<u8> = VecDeque::new();
    let mut index = 0;
    for _i in 0..stream.len() {
        if !unique.contains(&stream[_i]) {
            unique.push_back(stream[_i]);
            if unique.len() == size as usize {
                index = _i + 1;
                unique.clear();
                break;
            }
        }else {
            while unique.front() != Some(&stream[_i]) {
                unique.pop_front();
            }
            unique.pop_front();
            unique.push_back(stream[_i]);
        } 
    }
    return index.try_into().unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };

    if part == "part2" {
        println!("{}", get_solution_part2("input.txt"));
    } else {
        println!("{}", get_solution_part1("input.txt"));
    }
}