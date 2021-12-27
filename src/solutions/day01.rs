pub fn part1(text: &str) -> String {
    text.lines()
        .flat_map(|line| line.parse::<u32>())
        .map(|mass| (mass / 3) - 2)
        .sum::<u32>()
        .to_string()
}

pub fn part2(text: &str) -> String {
    let mut sum = 0;
    for mut fuel in text
        .lines()
        .flat_map(|line| line.parse::<i32>())
        .map(|mass| (mass / 3) - 2)
    {
        while fuel > 0 {
            sum += fuel;
            fuel = fuel / 3 - 2;
        }
    }
    sum.to_string()
}
