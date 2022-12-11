use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn extract_numbers(string_pair: Vec<&str>) -> Vec<u32> {
    let mut pair_vec: Vec<u32> = Vec::new();
    for pair in string_pair {
        let current_pair: Vec<&str> = pair.split('-').collect();
        for subs in current_pair {
            match subs.parse::<u32>() {
                Ok(number) => pair_vec.push(number),
                _ => continue,
            }
        }
    }
    return pair_vec;
}

fn check_pair_containes(pair_vec: &Vec<u32>) -> bool {
    let second_max = pair_vec[3];
    let second_min = pair_vec[2];
    let first_max = pair_vec[1];
    let first_min = pair_vec[0];

    if first_min >= second_min && first_max <= second_max {
        return true;
    } else if second_min >= first_min && second_max <= first_max {
        return true;
    } else {
        return false;
    }
}

fn check_pair_overlap(pair_vec: &Vec<u32>) -> bool {
    let second_max = pair_vec[3];
    let second_min = pair_vec[2];
    let first_max = pair_vec[1];
    let first_min = pair_vec[0];

    // first index
    if first_min >= second_min && first_min <= second_max {
        return true;
    } else if first_max >= second_min && first_max <= second_max {
        return true;
    } else if second_min >= first_min && second_min <= first_max {
        return true;
    } else if second_max >= first_min && second_max <= first_max {
        return true;
    } else {
        return false;
    }
}
fn get_solution_part2(path: &str) -> u32 {
    let mut result = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line = line.unwrap();
            let string_pair: Vec<&str> = line.split(',').collect();
            let pair = extract_numbers(string_pair);
            if check_pair_overlap(&pair) {
                result += 1;
            }
        }
    }
    return result;
}

fn get_solution_part1(path: &str) -> u32 {
    let mut result = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line = line.unwrap();
            let string_pair: Vec<&str> = line.split(',').collect();
            let pair = extract_numbers(string_pair);
            if check_pair_containes(&pair) {
                result += 1;
            }
        }
    }
    return result;
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
