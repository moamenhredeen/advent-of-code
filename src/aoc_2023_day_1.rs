use regex::Regex;
use std::{collections::HashMap, fs};

pub fn solve() {
    let numbers = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "thr3ee"),
        ("four", "fo4ur"),
        ("five", "fi5ve"),
        ("six", "si6x"),
        ("seven", "sev7en"),
        ("eight", "ei8ght"),
        ("nine", "ni9ne"),
    ]);

    let re = Regex::new(r"\d").unwrap();
    let mut sum: u64 = 0;
    for line in fs::read_to_string("./src/aoc_2023_day_1").unwrap().lines() {
        let mut l: String = line.to_string();
        for (key, value) in &numbers {
            l = l.replace(key, value);
        }
        let first = re.find(l.as_str()).unwrap();
        let last = re.find_iter(l.as_str()).last().unwrap();
        println!("{}___{}___{}", l, first.as_str(), last.as_str());
        sum += format!("{}{}", first.as_str(), last.as_str())
            .parse::<u64>()
            .unwrap()
    }
    println!("sum: {}", sum)
}
