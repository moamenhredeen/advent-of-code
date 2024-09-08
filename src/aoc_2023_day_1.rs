use regex::Regex;
use std::fs;

pub fn solve() {
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut sum = 0;
    for line in fs::read_to_string("./src/aoc_2023_day_1").unwrap().lines() {
        let (first, ..., last) = re.captures_iter(line).map(|c| c.extract()).collect();
        println!("first {:?}", first);
    }
}
