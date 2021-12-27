use std::fs::read_to_string;

mod day01;
pub(crate) fn day01() {
    println!("day01:");
    let text = read_to_string("res/day01.txt").unwrap();
    println!("part1: {}", day01::part1(&text));
    println!("part2: {}", day01::part2(&text));
}
