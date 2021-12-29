use std::fs::read_to_string;

mod day01;
pub(crate) fn day01() {
    use day01::*;
    println!("day01:");
    let text = read_to_string("res/day01.txt").unwrap();
    println!("part1: {}", part1(&text));
    println!("part2: {}", part2(&text));
}
mod day02;
pub(crate) fn day02() {
    use day02::*;
    println!("day02:");
    let text = read_to_string("res/day02.txt").unwrap();
    println!("part1: {}", part1(&text));
    println!("part2: {}", part2(&text));
}
mod day03;
pub(crate) fn day03() {
    use day03::*;
    println!("day03:");
    let text = read_to_string("res/day03.txt").unwrap();
    println!("part1: {}", part1(&text));
    println!("part2: {}", part2(&text));
}
mod day04;
pub(crate) fn day04() {
    use day04::*;
    println!("day04:");
    let text = read_to_string("res/day04.txt").unwrap();
    println!("part1: {}", part1(&text));
    println!("part2: {}", part2(&text));
}
mod day05;
pub(crate) fn day05() {
    use day05::*;
    println!("day05:");
    let text = read_to_string("res/day05.txt").unwrap();
    println!("part1: {}", part1(&text));
    println!("part2: {}", part2(&text));
}
mod day06;
pub(crate) fn day06() {
    use day06::*;
    println!("day06:");
    let text = read_to_string("res/day06.txt").unwrap();
    println!("part1: {}", part1(&text));
    println!("part2: {}", part2(&text));
}
