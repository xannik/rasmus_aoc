use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

*/

fn get_solution_part2(path: &str) -> u32 {
    let mut result = 0;
    let mut rucksacks: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            rucksacks.push(line.unwrap().into_bytes());
        }
    }

    for _i in 0..(rucksacks.len() / 3) {
        let mut first_rucksack = rucksacks.pop().expect("error!");
        let mut second_rucksack = rucksacks.pop().expect("error!");
        let mut third_rucksack = rucksacks.pop().expect("error!");

        first_rucksack.sort_unstable();
        second_rucksack.sort_unstable();
        third_rucksack.sort_unstable();
        let mut matching_item = 0;
        let mut previous_item_f = 0;
        for item_f in first_rucksack {
            if previous_item_f != item_f {
                previous_item_f = item_f
            } else {
                continue;
            }
            if second_rucksack.contains(&item_f) {
                if third_rucksack.contains(&item_f) {
                    matching_item = item_f;
                }
            }
        }

        if matching_item == (255 as u8) {
            continue;
        }
        if matching_item >= 65 && matching_item <= 96 {
            result += (matching_item - 65 + 27) as u32;
        } else if matching_item >= 97 && matching_item <= 122 {
            result += (matching_item - 97 + 1) as u32;
        }
    }
    return result;
}

fn get_solution_part1(path: &str) -> u32 {
    let mut result = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let mut first_compartment = line.unwrap().into_bytes();
            let rucksack_length = first_compartment.len();
            let mut second_compartment = first_compartment.split_off(rucksack_length / 2);
            first_compartment.sort_unstable();
            second_compartment.sort_unstable();
            let mut matching_item = 0;
            for item_f in &first_compartment {
                for item_s in &second_compartment {
                    if *item_f == *item_s {
                        matching_item = *item_f;
                        break;
                    }
                }
            }
            // a = 97, A = 65
            if matching_item >= 65 && matching_item <= 96 {
                result += (matching_item - 65 + 27) as u32;
                println!("{}", matching_item - 65 + 27);
            } else if matching_item >= 97 && matching_item <= 122 {
                result += (matching_item - 97 + 1) as u32;
                println!("{}", matching_item - 97 + 1);
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
