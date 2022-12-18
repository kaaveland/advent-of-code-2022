use std::collections::HashSet;
use std::io;
use std::io::Read;
use std::time::Instant;
use anyhow::{Context, Result};

use regex::Regex;
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Location(i32, i32);
impl Location {
    fn x(&self) -> i32 { match self { Location(x, _) => *x }}
    fn y(&self) -> i32 { match self { Location(_, y) => *y }}
}

fn manhattan_dist(left: &Location, right: &Location) -> i32
{
    (left.x() - right.x()).abs() + (left.y() - right.y()).abs()
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Input(Location, Location);
type Map = Vec<Input>;

impl From<(i32, i32)> for Location {
    fn from(tup: (i32, i32)) -> Self { match tup { (x, y) => Location(x, y) } }
}
impl From<Location> for (i32, i32) {
    fn from(loc: Location) -> Self {
        (loc.x(), loc.y())
    }
}

fn parse_lines<T: AsRef<str>>(input: T) -> Result<Map> {
    let re = Regex::new(r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)")?;

    input.as_ref().lines()
        .filter(|l| !l.is_empty())
        .map(| l | {
            let caps = re.captures(l).context("Expected match")?;
            let x1s = caps.get(1).context("Expected x1")?;
            let y1s = caps.get(2).context("Expected y1")?;
            let x2s = caps.get(3).context("Expected x2")?;
            let y2s = caps.get(4).context("Expected y2")?;
            let x1 = x1s.as_str().parse()?;
            let y1 = y1s.as_str().parse()?;
            let x2 = x2s.as_str().parse()?;
            let y2 = y2s.as_str().parse()?;
            Ok(Input(Location(x1, y1), Location(x2, y2)))
        }).collect()
}

fn solve_problem_one(inputs: &Map, row: i32) -> usize {
    let mut points = HashSet::new();
    for Input(sensor, beacon) in inputs.iter() {
        let distance = manhattan_dist(sensor, beacon);
        let remaining = distance - (sensor.y() - row).abs();
        let intersect_x = sensor.x();

        if remaining <= 0 {
            continue;
        }
        for i in 0..=remaining {
            points.insert(Location(intersect_x - i, row));
            points.insert(Location(intersect_x + i, row));
        }
    }
    for Input(sensor, beacon) in inputs.iter() {
        points.remove(beacon);
        points.remove(sensor);
    }
    points.len()
}

fn find_distress_beacon(map: &Map) -> Option<Location> {
    let mut candidate_locations = HashSet::new();
    let (xmin, xmax) = map.iter()
        .fold((0, 0), |(xmin, xmax), input| {
            match input {
                Input(Location(x, _), _) if *x < xmin => { (*x, xmax) }
                Input(Location(x, _), _) if *x > xmax => { (xmin, *x) }
                _ => (xmin, xmax)
            }
        });
    let (ymin, ymax) = map.iter()
        .fold((0, 0), |(ymin, ymax), input| {
            match input {
                Input(Location(_, y), _) if *y < ymin => { (*y, ymax) }
                Input(Location(_, y), _) if *y > ymax => { (ymin, *y) }
                _ => (ymin, ymax)
            }
        });
    
    // Must be 1 unit outside sensor/beacon distance
    // Generate those possible places
    for Input(sensor, beacon) in map.iter() {
        let dist = manhattan_dist(sensor, beacon);
        let outside = dist + 1;

        for i in 0..=outside {
            let nw = Location(sensor.x() - i, sensor.y() + (outside - i));
            if nw.y() >= ymin && nw.y() <= ymax && nw.x() >= xmin && nw.x() <= xmax {
                candidate_locations.insert(nw);
            }
            let ne = Location(sensor.x() + i, sensor.y() + (outside - i));
            if ne.y() >= ymin && ne.y() <= ymax && ne.x() >= xmin && ne.x() <= xmax {
                candidate_locations.insert(ne);
            }            
            let sw = Location(sensor.x() - i, sensor.y() - (outside - i));
            if sw.y() >= ymin && sw.y() <= ymax && sw.x() >= xmin && sw.x() <= xmax {
                candidate_locations.insert(sw);
            }            
            let se = Location(sensor.x() + i, sensor.y() - (outside - i));
            if se.y() >= ymin && se.y() <= ymax && se.x() >= xmin && se.x() <= xmax {
                candidate_locations.insert(se);
            }            
        }
    }

    candidate_locations = candidate_locations.iter().filter(| &Location(x, y) | {
        *x >= xmin && *x <= xmax && *y >= ymin && *y <= ymax
    }).cloned().collect();


    for Input(sensor, beacon) in map.iter() {
        let dist = manhattan_dist(sensor, beacon);
        candidate_locations = candidate_locations.iter()
            .filter(|&loc| manhattan_dist(sensor, loc) > dist)
            .cloned()
            .collect();
    }

    println!("Found {} after filter", candidate_locations.len());

    if candidate_locations.len() != 1 {
        None
    } else {
        candidate_locations.iter().next().cloned()
    }
}

fn tuning_distance(loc: &Location) -> i64 {
    let x = loc.x() as i64;
    let y = loc.y() as i64;
    x * 4000000 + y
}

#[cfg(test)]
mod tests {
    use crate::{find_distress_beacon, Input, Location, manhattan_dist, parse_lines, solve_problem_one};

    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_manhattan_dist() {
        let origin = Location(0, 0);
        let left = Location(-1, 0);
        let diag_right = Location(1, 1);

        assert_eq!(
            manhattan_dist(&origin, &left), 1
        );
        assert_eq!(
            manhattan_dist(&diag_right, &origin), 2
        );
    }

    #[test]
    fn test_parsing() {
        let inputs = parse_lines(EXAMPLE).unwrap();
        let first = &inputs[0];
        assert_eq!(
            first, &Input(Location(2, 18), Location(-2, 15))
        );
        assert_eq!(
            &inputs[1], &Input(Location(9, 16), Location(10, 16))
        );
    }

    #[test]
    fn test_solve_problem_1_example() {
        let map = parse_lines(EXAMPLE).unwrap();
        let score = solve_problem_one(&map, 10);
        assert_eq!(score, 26);
    }

    #[test]
    fn test_solve_problem_2_example() {
        let map = parse_lines(EXAMPLE).unwrap();
        let loc = find_distress_beacon(&map).unwrap();
        assert_eq!(loc, Location(14, 11));
    }
}

fn main() -> Result<()> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;
    let map = parse_lines(content)?;
    let start = Instant::now();
    let solution = solve_problem_one(&map, 2000000);
    println!("{solution} in {}ms", start.elapsed().as_millis());
    let start = Instant::now();
    let distress_beacon = find_distress_beacon(&map).context("Unable to find 1 point")?;
    let solution_part_2 = tuning_distance(&distress_beacon);
    println!("{solution_part_2} in {}ms", start.elapsed().as_millis());
    Ok(())
}