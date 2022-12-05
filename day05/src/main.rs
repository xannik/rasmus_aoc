use std::collections::{VecDeque};
use std::{env};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
/*

[J]             [F] [M]
[Z] [F]     [G] [Q] [F]
[G] [P]     [H] [Z] [S] [Q]
[V] [W] [Z] [P] [D] [G] [P]
[T] [D] [S] [Z] [N] [W] [B] [N]
[D] [M] [R] [J] [J] [P] [V] [P] [J]
[B] [R] [C] [T] [C] [V] [C] [B] [P]
[N] [S] [V] [R] [T] [N] [G] [Z] [W]
 1   2   3   4   5   6   7   8   9

*/

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn move_crates(stacks: &mut [VecDeque<u8>], amount_to_move: usize, from: usize, to: usize) {
    for _i in 1..amount_to_move + 1 {
        let item_to_move = stacks[from].pop_back().expect("none");
        println!("{}", item_to_move);
        stacks[to].push_back(item_to_move);
    }
    for stack in stacks {
        println!("current stack state: {:?}", stack);
    }
}

fn parse_and_store_cargo(stacks: &mut [VecDeque<u8>], line: String) -> () {
        let stack_indicies: [usize; 9] = [1, 5, 9, 13, 17, 21, 25, 29, 33];
        let mut store_at = 0;
        for index in stack_indicies {
            let create = line.chars().nth(index).unwrap();
            if create.is_ascii_uppercase() {
                stacks[store_at].push_front(create as u8);
            }
            store_at += 1;
        }
}

fn get_solution_part1(path: &str) -> String {
    let mut result: [char ; 9] = Default::default();
    if let Ok(lines) = read_lines(path) {
        // allocate the different stacks
        let mut stacks: [VecDeque<u8>; 9] = Default::default();
       
        //let mut moves: Vec<&str> = Vec::new();
        for line in lines {
            let current_line = String::from(line.unwrap());
            if current_line.contains("[") {
                parse_and_store_cargo(&mut stacks, current_line);

            }else if current_line.contains("move") {
                let mut commands = Vec::new();
                for subs in current_line.split(" ") {
                    match subs.parse::<i32>(){
                        Ok(number) => commands.push(number),
                        _ => continue,
                    };
                    
                }
                println!("{} {} {}", commands[0], commands[1], commands[2]);
                move_crates(&mut stacks, commands[0] as usize, commands[1] as usize - 1, commands[2] as usize - 1);

            }
            
        }
        for _i in 0..9 {
            result[_i] = (stacks[_i].pop_back().expect("empty queue")) as char;
        }
    }
    
    
    return String::from(result.iter().cloned().collect::<String>());
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };

    if part == "part2" {
        //println!("{}", get_solution_part2("input.txt"));
    } else {
        println!("{}", get_solution_part1("input.txt"));
    }
}

