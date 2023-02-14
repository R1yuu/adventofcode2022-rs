use std::fs::File;
use std::io::{BufReader, Lines};
use crate::aoc::day2::RPS::{PAPER, ROCK, SCISSORS};

#[derive(Clone, Copy)]
enum RPS {
    ROCK,
    PAPER,
    SCISSORS
}

pub fn task1(lines: Box<Lines<BufReader<File>>>) -> i32 {
    let mut score = 0;

    let calc_closure = |you: RPS, them: RPS| -> i32 {
        match you {
            ROCK => 1 + match them {
                ROCK => 3,
                PAPER => 0,
                SCISSORS => 6
            },
            PAPER => 2 + match them {
                ROCK => 6,
                PAPER => 3,
                SCISSORS => 0
            },
            SCISSORS => 3 + match them {
                ROCK => 0,
                PAPER => 6,
                SCISSORS => 3
            },
        }
    };

    for line in lines {
        if let Ok(ip) = line {
            let mut choice: RPS = ROCK;
            let split = ip.split(" ");
            for pick in split {
                match pick {
                    "A" => choice = ROCK,
                    "B" => choice = PAPER,
                    "C" => choice = SCISSORS,
                    "X" => score += calc_closure(ROCK, choice),
                    "Y" => score += calc_closure(PAPER, choice),
                    "Z" => score += calc_closure(SCISSORS, choice),
                    _ => {}
                }
            }
        }
    }
    score
}

pub fn task2(lines: Box<Lines<BufReader<File>>>) -> i32 {
    let mut score = 0;
    for line in lines {
        if let Ok(ip) = line {
            let n0 = ip.chars().nth(0);
            let n2 = ip.chars().nth(2);
            if n2 == Some('Y') { score += 3 + n0.unwrap() as i32 - 64; }
            else if n2 == Some('Z') { score += 6; }
            if n0 == Some('A') && n2 == Some('X') { score += 3; }
            else if n0 == Some('A') && n2 == Some('Z') { score += 2; }
            else if n0 == Some('B') && n2 == Some('X') { score += 1; }
            else if n0 == Some('B') && n2 == Some('Z') { score += 3; }
            else if n0 == Some('C') && n2 == Some('X') { score += 2; }
            else if n0 == Some('C') && n2 == Some('Z') { score += 1; }
        }
    }
    score
}