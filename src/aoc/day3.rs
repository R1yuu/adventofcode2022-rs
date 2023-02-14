use std::fs::File;
use std::io::{BufReader, Lines};

pub fn task1(lines: Box<Lines<BufReader<File>>>) -> i32 {
    let mut res = 0;
    for line in lines {
        if let Ok(ip) = line {
            let (first, second) = ip.split_at(ip.len() / 2);
            for ch in first.chars() {
                if second.contains(ch) {
                    if ch.is_lowercase() {
                        res += ch as i32 - 96;
                    } else {
                        res += ch as i32 - 38;
                    }
                    break;
                }
            }
        }
    }
    res
}

pub fn task2(lines: Box<Lines<BufReader<File>>>) -> i32 {
    let mut res = 0;
    let mut line_buf: Vec<String> = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            line_buf.push(ip);
            if line_buf.len() == 3 {
                for ch in line_buf.get(0).unwrap().chars() {
                    if line_buf.get(1).unwrap().contains(ch) && line_buf.get(2).unwrap().contains(ch) {
                        if ch.is_lowercase() {
                            res += ch as i32 - 96;
                        } else {
                            res += ch as i32 - 38;
                        }
                        line_buf.clear();
                        break;
                    }
                }
            }
        }
    }
    res
}