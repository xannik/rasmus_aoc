use std::env;
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
   let file = File::open(filename)?;
   Ok(io::BufReader::new(file).lines())
}

fn get_solution_part1(path: &str) -> i32 {
   let mut max_calories: i32 = 0;
   let mut current_calories: i32 = 0;
   if let Ok(lines) = read_lines(path) {
      for line in lines {
         let number : i32 = match line.unwrap().trim().parse::<i32>() {
            Ok(number) => number,
            Err(_e) => -1,
         };
         if number == -1 {
            if max_calories < current_calories {
               max_calories = current_calories;
               current_calories = 0
            }
         }else {
            current_calories += number
         }
      
      }
      
   }
   return max_calories;
}

fn main() {
   let part = match env::var("part") {
      Ok(val) => val,
      Err(_e) => "part1".to_string(),
   };

   if part == "part2" {
      println!("do part 2!");
   } else {
      println!("{}", get_solution_part1("input.txt"));
   }
}