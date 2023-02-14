mod aoc;

use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use crate::aoc::*;

fn main() {
    println!("Day1-Task1: {}", day1::task1(read_lines("resources/day1")));
    println!("Day1-Task2: {}", day1::task2(read_lines("resources/day1")));
    println!("Day2-Task1: {}", day2::task1(read_lines("resources/day2")));
    println!("Day2-Task2: {}", day2::task2(read_lines("resources/day2")));
    println!("Day3-Task1: {}", day3::task1(read_lines("resources/day3")));
    println!("Day3-Task2: {}", day3::task2(read_lines("resources/day3")));
}

fn read_lines<P>(filename: P) -> Box<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename).expect("Could not open file.");
    Box::new(io::BufReader::new(file).lines())
}