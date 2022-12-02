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
            }
            current_calories = 0
         }else {
            current_calories += number
         }
      
      }
      
   }
   return max_calories;
}

fn get_solution_part2(path: &str) -> i32 {
   let mut max_calories: [i32; 3] = [0; 3];
   let mut current_calories: i32 = 0;
   if let Ok(lines) = read_lines(path) {
      for line in lines {
         let number : i32 = match line.unwrap().trim().parse::<i32>() {
            Ok(number) => number,
            Err(_e) => -1,
         };
         if number == -1 {
            for element in max_calories.iter_mut() {
               // calories has been found and stored
               if current_calories == *element {
                  break;
               }
               // found an elf with more calories
               if *element < current_calories {
                  let previous_val = *element;
                  *element = current_calories;
                  // assign current calories the old value and keep going
                  current_calories = previous_val;
               }
            }
            current_calories = 0
         }else {
            current_calories += number;
         }
      
      }
      
   }
   return max_calories[0] + max_calories[1] + max_calories[2];
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