use std::collections::HashMap;

pub fn part1(text: &str) -> String {
    let mut orbits = HashMap::new();
    for line in text.lines() {
        let mut split = line.split(')');
        orbits
            .entry(split.next().unwrap())
            .or_insert_with(Vec::new)
            .push(split.next().unwrap());
    }
    let mut paths = Vec::new();
    sum_orbits(&orbits, "COM", 0, &mut paths);
    paths.iter().sum::<usize>().to_string()
}

fn sum_orbits(orbits: &HashMap<&str, Vec<&str>>, node: &str, sum: usize, paths: &mut Vec<usize>) {
    paths.push(sum);
    if let Some(orbit) = orbits.get(node) {
        orbit.iter().for_each(|&o| {
            sum_orbits(orbits, o, sum + 1, paths);
        });
    }
}
pub fn part2(_: &str) -> String {
    "".to_string()
}
