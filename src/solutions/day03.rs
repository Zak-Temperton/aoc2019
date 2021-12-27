use std::collections::HashMap;

pub fn part1(text: &str) -> String {
    let mut lines = text.lines();
    let wires1 = parse_wires(&mut lines);
    let wires2 = parse_wires(&mut lines);
    let mut min = isize::MAX;
    for i in 0..wires1.len() {
        for j in 0..wires2.len() {
            if let Some(intersection) = wires1[i].intersection(&wires2[j]) {
                let manhattan = intersection.manhattan();
                if manhattan < min {
                    min = manhattan;
                }
            }
        }
    }
    min.to_string()
}

fn parse_wires(lines: &mut std::str::Lines) -> Vec<Wire> {
    let points = lines.next().unwrap().split(',');
    let mut last = Point { x: 0, y: 0 };
    let mut wires = Vec::new();
    for point in points {
        let offset = point[1..].parse::<isize>().unwrap();
        match &point[0..1] {
            "U" => {
                let mut new_point = last;
                new_point.y += offset;
                wires.push(Wire::new(last, new_point));
                last = new_point
            }
            "D" => {
                let mut new_point = last;
                new_point.y -= offset;
                wires.push(Wire::new(last, new_point));
                last = new_point
            }
            "L" => {
                let mut new_point = last;
                new_point.x -= offset;
                wires.push(Wire::new(last, new_point));
                last = new_point
            }
            "R" => {
                let mut new_point = last;
                new_point.x += offset;
                wires.push(Wire::new(last, new_point));
                last = new_point
            }
            _ => panic!(),
        }
    }
    wires
}

pub fn part2(text: &str) -> String {
    let mut lines = text.lines();
    let wires1 = parse_wires(&mut lines);
    let wires2 = parse_wires(&mut lines);
    let mut min = isize::MAX;
    for i in 0..wires1.len() {
        for j in 0..wires2.len() {
            if let Some(intersection) = wires1[i].intersection(&wires2[j]) {
                let mut steps = 0;
                for i in 0..i {
                    steps += wires1[i].manhattan_length();
                }
                steps += (wires1[i].start.x - intersection.x).abs()
                    + (wires1[i].start.y - intersection.y).abs();

                for j in 0..j {
                    steps += wires2[j].manhattan_length();
                }
                steps += (wires2[j].start.x - intersection.x).abs()
                    + (wires2[j].start.y - intersection.y).abs();
                if steps < min {
                    min = steps;
                }
            }
        }
    }
    (min).to_string()
}

#[derive(Clone, Copy)]
struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    pub fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

struct Wire {
    start: Point,
    end: Point,
}

impl Wire {
    pub fn new(start: Point, end: Point) -> Self {
        Wire { start, end }
    }

    pub fn intersection(&self, other: &Wire) -> Option<Point> {
        if self.start.y == self.end.y && other.start.x == other.end.x {
            let y = self.start.y;
            let x = other.start.x;
            let x_upper = self.start.x.max(self.end.x);
            let x_lower = self.start.x.min(self.end.x);
            let y_upper = other.start.y.max(other.end.y);
            let y_lower = other.start.y.min(other.end.y);
            if x > x_lower && x < x_upper && y > y_lower && y < y_upper {
                Some(Point::new(x, y))
            } else {
                None
            }
        } else if self.start.x == self.end.x && other.start.y == other.end.y {
            let x = self.start.x;
            let y = other.start.y;
            let x_upper = other.start.x.max(other.end.x);
            let x_lower = other.start.x.min(other.end.x);
            let y_upper = self.start.y.max(self.end.y);
            let y_lower = self.start.y.min(self.end.y);
            if x > x_lower && x < x_upper && y > y_lower && y < y_upper {
                Some(Point::new(x, y))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn manhattan_length(&self) -> isize {
        (self.start.x - self.end.x).abs() + (self.start.y - self.end.y).abs()
    }

    pub fn point_on_wire(&self, point: &Point) -> bool {
        true
    }
}
