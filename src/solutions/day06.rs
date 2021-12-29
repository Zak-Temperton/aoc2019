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
    sum_orbits(&orbits, "COM", 0).to_string()
}

fn sum_orbits(orbits: &HashMap<&str, Vec<&str>>, node: &str, depth: usize) -> usize {
    let mut total = depth;
    if let Some(orbit) = orbits.get(node) {
        orbit.iter().for_each(|&o| {
            total += sum_orbits(orbits, o, depth + 1);
        });
    }
    total
}
pub fn part2(_: &str) -> String {
    "".to_string()
}
