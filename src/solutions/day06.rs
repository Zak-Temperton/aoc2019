use std::collections::HashMap;

pub fn part1(text: &str) -> String {
    let orbits = parse_orbits(text);
    sum_orbits(&orbits, "COM", 0).to_string()
}

fn parse_orbits(text: &str) -> HashMap<&str, Vec<&str>> {
    let mut orbits = HashMap::new();
    for line in text.lines() {
        let mut split = line.split(')');
        orbits
            .entry(split.next().unwrap())
            .or_insert_with(Vec::new)
            .push(split.next().unwrap());
    }
    orbits
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
pub fn part2(text: &str) -> String {
    let orbits = parse_orbits_rev(text);
    let mut a = "YOU";
    let mut b = "SAN";
    let mut dist_from_a = HashMap::new();
    let mut dist = 0usize;
    while let Some(&o) = orbits.get(a) {
        a = o;
        dist_from_a.insert(a, dist);
        dist += 1;
    }
    dist = 0;
    while let Some(&o) = orbits.get(b) {
        b = o;
        if let Some(&d) = dist_from_a.get(b) {
            dist += d;
            break;
        }
        dist += 1;
    }
    dist.to_string()
}

fn parse_orbits_rev(text: &str) -> HashMap<&str, &str> {
    let mut orbits = HashMap::new();
    for line in text.lines() {
        let mut split = line.split(')');
        let root = split.next().unwrap();
        orbits.insert(split.next().unwrap(), root);
    }
    orbits
}
