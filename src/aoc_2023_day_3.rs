//! # Advent of Code 2023 Day 2: Gear Ratios
//!
//! You and the Elf eventually reach a [gondola lift](https://en.wikipedia.org/wiki/Gondola_lift) station;
//! he says the gondola lift will take you up to the *water source*,
//! but this is as far as he can bring you. You go inside.
//!
//! It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
//!
//! "Aaah!"
//!
//! You turn around to see a slightly-greasy Elf with a wrench and a look of surprise.
//! "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now;
//! it'll still be a while before I can fix it." You offer to help.
//!
//! The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one.
//! If you can *add up all the part numbers* in the engine schematic, it should be easy to work out which part is missing.
//!
//! The engine schematic (your puzzle input) consists of a visual representation of the engine.
//! There are lots of numbers and symbols you don't really understand, but apparently *any number adjacent to a symbol*,
//! even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
//! Here is an example engine schematic:
//!
//! ```text
//! 467..114..
//! ...*......
//! ..35..633.
//! ......#...
//! 617*......
//! .....+.58.
//! ..592.....
//! ......755.
//! ...$.*....
//! .664.598..
//! ```
//!
//! In this schematic, two numbers are not part numbers because they are not adjacent to a symbol:
//! 114 (top right) and 58 (middle right).
//! Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
//! Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

use std::fs;

#[allow(unused)]
pub fn solve() {
    println!("-------------------------- advent of code 2023 day 3 --------------------------");

    match fs::read_to_string("./src/test_input") {
        Ok(file) => {
            let mut lines = file.lines();
            let mut previous    = lines.next().unwrap().chars().collect::<Vec<char>>();
            let mut current     = lines.next().unwrap().chars().collect::<Vec<char>>();
            let mut next        = lines.next().unwrap().chars().collect::<Vec<char>>();
            let mut on_number = false;
            let mut is_engine_part = false;
            let mut sum = 0_u32;
            let mut current_number = String::new();

            // iterate over lines
            for line in lines {

                // iterate of the characters of a line
                for (i, c) in current.iter().enumerate() {

                    // if the current charactr is a digit
                    if c.is_ascii_digit() {

                        // if this is the first digit of the number (previous character was not a
                        // digit)
                        // - clear the the string buffer used to collect the digit of the number
                        // - check if the previous character on the current line, previous line, or
                        // next line is a symbol (not period)
                        if i > 0 && !on_number {
                            is_engine_part =  current[i-1] != '.' || previous[i-1] != '.' || next[i-1] != '.';
                        }

                        is_engine_part =  previous[i] != '.' || next[i] != '.';
                        current_number.push(*c);
                        on_number = true;
                    }else {
                        if on_number {
                            if is_engine_part
                                || (previous[i] != '.' || current[i] != '.' || next[i] != '.') {
                                let n: u32 = current_number.parse().unwrap();
                                sum += n;
                                println!("✔ {}", n);
                            }else{
                                println!("✘ {}", current_number);
                            }
                            current_number.clear();
                        }

                        is_engine_part = false;
                        on_number = false;
                    }
                }

                previous = current;
                current = next;
                next = line.chars().collect();
            }

            println!("part 1: {}", sum);
        }
        Err(err) => {
            println!("could not read the file. error: {}", err);
        }
    }
}
