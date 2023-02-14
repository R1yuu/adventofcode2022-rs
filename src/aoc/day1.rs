use std::fs::File;
use std::io::{BufReader, Lines};

pub fn task1(lines: Box<Lines<BufReader<File>>>) -> i32 {
    let mut top = 0;
    let mut curr = 0;
    for line in lines {
        if let Ok(ip) = line {
            if ip.is_empty() {
                if curr > top {
                    top = curr;
                }
                curr = 0;
            } else {
                curr += ip.parse::<i32>().unwrap();
            }
        }
    }
    top
}

pub fn task2(lines: Box<Lines<BufReader<File>>>) -> i32 {
    let mut top = Vec::new();
    let mut curr = 0;
    for line in *lines {
        if let Ok(ip) = line {
            if ip.is_empty() {
                top.push(curr);
                curr = 0;
            } else {
                curr += ip.parse::<i32>().unwrap();
            }
        }
    }
    top.sort();
    let mut top_rev = top.iter().rev();
    top_rev.next().unwrap() + top_rev.next().unwrap() + top_rev.next().unwrap()
}

