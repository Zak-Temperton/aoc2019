#![feature(test)]

extern crate test;

mod solutions;
use solutions::*;

use regex::Regex;

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("please choose a day to run");
        println!("cargo run <day>");
        return;
    }
    args.next(); //executable path
    let r_day = Regex::new(r"(?:([\d]+))").unwrap();
    let day = args.next().unwrap();
    let captures = r_day.captures(day.as_str()).unwrap();
    match captures.get(1).unwrap().as_str().parse::<u32>().unwrap() {
        1 => day01(),
        2 => day02(),
        3 => day03(),
        4 => day04(),
        5 => day05(),
        6 => day06(),
        7 => day07(),
        _ => {}
    }
}
