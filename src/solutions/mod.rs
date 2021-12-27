use std::fs::read_to_string;

mod day01;
pub(crate) fn day01() {
    println!("day01:");
    let text = read_to_string("res/day01.txt").unwrap();
    println!("part1: {}", day01::part1(&text));
    println!("part2: {}", day01::part2(&text));
}
mod day02;
pub(crate) fn day02() {
    println!("day02:");
    let text = read_to_string("res/day02.txt").unwrap();
    println!("part1: {}", day02::part1(&text));
    println!("part2: {}", day02::part2(&text));
}
