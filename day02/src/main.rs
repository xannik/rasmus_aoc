use std::env;
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

/* 
 part 1
    Opponent: 
    A -> Rock
    B -> Paper
    C -> Scissor
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

/* 
 part 2
    Opponent: 
    A -> Rock
    B -> Paper
    C -> Scissors
    My response:
    X -> lose
    Y -> draw
    Z -> win

    total score = sum of score for each round

    score for a single run:
    1 for rock
    2 for PaCper
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

static WIN: i32 = 6;
static LOOSE: i32 = 0;
static DRAW: i32 = 3;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
 }

fn fetch_score_part1(p1: char, p2: char) -> i32 {
    let score;
    let used_move = match p2 {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => {
            println!("unknown");
            -1
        },
    };
    if p1 == p2 {
        score = DRAW;
    }else {
        score = match p1 {
            'A' => {
                if p2 == 'C' {
                    LOOSE
                }else {
                    WIN
                }
            },
            'B' => {
                if p2 == 'A' {
                    LOOSE
                }else {
                    WIN  
                }
            },
            'C' => {
                if p2 == 'B' {
                    LOOSE
                }else {
                    WIN   
                }
            },
            _=> {
                println!("unknown");
                -1
            },
        }
    }
    return score + used_move;
} 

fn fetch_move_value(used_move: char) -> i32 {
    let result = match used_move {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => {
            println!("unknown");
            -1
        },
    };
    return result;
}

fn fetch_score_part2(p1: char, p2: char) -> i32 {
    let score;
    let mut used_move_value = 0;
    if p2 == 'B' {
        used_move_value = fetch_move_value(p1);
        score = DRAW
    }else {
        score = match p2 {
            'A' => {
                match p1 {
                    'A' => {
                        used_move_value = fetch_move_value('C');
                        LOOSE
                    },
                    'B' => {
                        used_move_value = fetch_move_value('A');
                        LOOSE
                    }
                    'C' => {
                        used_move_value = fetch_move_value('B');
                        LOOSE
                    }
                    _ => {
                        println!("unknown");
                        -1
                    }
                }
            },
            'C' => {
                match p1 {
                    'A' => {
                        used_move_value = fetch_move_value('B');
                        WIN
                    },
                    'B' => {
                        used_move_value = fetch_move_value('C');
                        WIN
                    },
                    'C' => {
                        used_move_value = fetch_move_value('A');
                        WIN
                    },
                    _ => {
                        println!("unknown");
                        -1
                    },
                }
            },
            _ => {
                println!("unknown");
                -1
            },
        }
    }
   return score + used_move_value;
}
fn get_solution_part2(path: &str) -> i32 {
    let mut score = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            // read line by line and store letters in vector without white space
            let words: Vec<_> = line.unwrap().chars().filter(|c| !c.is_whitespace()).collect();
            // There is always 2 characters
            let p1_move = words[0];
            let p2_move = match words[1] {
                'X' => 'A',
                'Y' => 'B',
                'Z' => 'C',
                _ => {
                    println!("unknown");
                   'D' 
                },
                 
            };
            score += fetch_score_part2(p1_move, p2_move);
        }
    }
    
    return score;
}



fn get_solution_part1(path: &str) -> i32 {
    
    let mut score = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            // read line by line and store letters in vector without white space
            let words: Vec<_> = line.unwrap().chars().filter(|c| !c.is_whitespace()).collect();
            // There is always 2 characters
            let p1_move = words[0];
            let p2_move = match words[1] {
                'X' => 'A',
                'Y' => 'B',
                'Z' => 'C',
                _ => {
                    println!("unknown");
                   'D' 
                },
                 
            };
            score += fetch_score_part1(p1_move, p2_move);
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
        println!("{}", get_solution_part2("input.txt"));
    }else {
        println!("{}", get_solution_part1("input.txt"));
    }
}