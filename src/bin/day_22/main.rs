use anyhow::{anyhow, Context, Result};
use aoc::io::read_stdin;

#[derive(Eq, PartialEq, Debug)]
pub enum Tile {
    Abyss,
    Wall,
    Open,
}
type Map = Vec<Vec<Tile>>;
impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        use Tile::*;
        match value {
            ' ' => Ok(Abyss),
            '#' => Ok(Wall),
            '.' => Ok(Open),
            _ => Err(anyhow!("Illegal tile: {}", value)),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Turn {
    R,
    L,
}

impl TryFrom<char> for Turn {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'L' => Ok(Turn::L),
            'R' => Ok(Turn::R),
            _ => Err(anyhow!("Illegal turn: {value}")),
        }
    }
}
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Step {
    Orient(Turn),
    Forward(u32),
}
type Direction = i64;
const NORTH: Direction = 3;
const EAST: Direction = 0;
const SOUTH: Direction = 1;
const WEST: Direction = 2;
type CoordSize = i64;
type Heading = (CoordSize, CoordSize);
const HEADINGS: [Heading; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
type Position = (CoordSize, CoordSize);

fn turn(direction: Direction, turn: Turn) -> Direction {
    match turn {
        Turn::L => direction - 1,
        Turn::R => direction + 1,
    }
}

fn heading(direction: Direction) -> Heading {
    HEADINGS[direction.rem_euclid(HEADINGS.len() as Direction) as usize]
}

fn next_position(map: &Map, position: Position, direction: Direction) -> Position {
    let h = heading(direction);
    let (x, y) = position;
    let (dx, dy) = h;

    let (mut nx, mut ny) = (
        (x + dx).rem_euclid(map[0].len() as CoordSize),
        (y + dy).rem_euclid(map.len() as CoordSize),
    );
    while map[ny as usize][nx as usize] == Tile::Abyss {
        nx = (nx + dx).rem_euclid(map[0].len() as CoordSize);
        ny = (ny + dy).rem_euclid(map.len() as CoordSize);
    }
    match map[ny as usize][nx as usize] {
        Tile::Open => (nx, ny),
        _ => (x, y),
    }
}

fn hike(map: &Map, steps: &Vec<Step>) -> (CoordSize, CoordSize, Direction) {
    use Step::*;

    let mut pos: (CoordSize, CoordSize) = (0, 0);
    // Find top left
    while map[pos.1 as usize][pos.0 as usize] != Tile::Open {
        pos = (pos.0 + 1, pos.1);
    }
    let mut dir = EAST;
    for step in steps {
        println!("At {pos:?} take {step:?}");
        match *step {
            Orient(t) => {
                dir = turn(dir, t);
            }
            Forward(mut steps) => {
                let mut next_pos = next_position(&map, pos, dir);

                while pos != next_pos && steps > 0 {
                    steps -= 1;
                    pos = next_pos;
                    next_pos = next_position(&map, pos, dir);
                }
            }
        }
    }

    (
        pos.1 + 1,
        pos.0 + 1,
        dir.rem_euclid(HEADINGS.len() as Direction),
    )
}

fn parse_board(input: &str) -> Result<Map> {
    let mut out = vec![];
    for line in input.lines().filter(|line| !line.is_empty()) {
        let mut row = vec![];
        for char in line.chars() {
            let tile = Tile::try_from(char)?;
            row.push(tile);
        }
        out.push(row);
    }
    let max_len = out.iter().map(|row| row.len()).max().context("Empty map")?;
    for row in &mut out {
        while row.len() < max_len {
            row.push(Tile::Abyss);
        }
    }
    Ok(out)
}

fn parse_hike(input: &str) -> Result<Vec<Step>> {
    let mut out = vec![];
    let mut gather = String::new();

    for char in input.chars() {
        if char.is_numeric() {
            gather.push(char);
        } else if !gather.is_empty() {
            let num = gather.parse()?;
            out.push(Step::Forward(num));
            gather.clear();
        }
        if !char.is_numeric() {
            let turn = Turn::try_from(char)?;
            out.push(Step::Orient(turn));
        }
    }
    if !gather.is_empty() {
        let num = gather.parse()?;
        out.push(Step::Forward(num));
    }
    Ok(out)
}

fn parse(input: &str) -> Result<(Map, Vec<Step>)> {
    let mut parts = input.split("\n\n");
    let map_part = parts.next().context("Bad input")?;
    let map = parse_board(map_part)?;
    let hike_part = parts.next().context("Bad input")?;
    let hike = parse_hike(hike_part.trim())?;
    Ok((map, hike))
}

fn main() -> Result<()> {
    let input = read_stdin()?;
    let (map, steps) = parse(input.as_str())?;
    let (row, col, face) = hike(&map, &steps);
    println!(
        "{row} * 1000 + {col} * 4 + {face} = {}",
        row * 1000 + col * 4 + face
    );
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::Tile::*;
    use crate::{next_position, Tile};
    use crate::{EAST, NORTH, SOUTH, WEST};
    use itertools::Itertools;

    const EX_MAP: [[Tile; 6]; 6] = [
        [Abyss, Abyss, Open, Wall, Open, Abyss],
        [Abyss, Abyss, Open, Open, Open, Abyss],
        [Open, Open, Wall, Open, Abyss, Abyss],
        [Open, Open, Open, Open, Abyss, Abyss],
        [Abyss, Abyss, Wall, Open, Open, Abyss],
        [Abyss, Abyss, Open, Open, Open, Abyss],
    ];

    #[test]
    fn test_next_position_stopped_by_wall() {
        let map: Vec<Vec<_>> = Vec::from(EX_MAP).into_iter().map(Vec::from).collect_vec();
        let pos = (3, 1);
        assert_eq!(next_position(&map, pos, NORTH), pos);
        assert_eq!(next_position(&map, (pos.0, pos.1 + 1), NORTH), pos);
        assert_eq!(next_position(&map, pos, SOUTH), (pos.0, pos.1 + 1));
        assert_eq!(next_position(&map, (2, 1), SOUTH), (2, 1));
        assert_eq!(next_position(&map, pos, WEST), (pos.0 - 1, pos.1));
        assert_eq!(
            next_position(&map, (pos.0, pos.1 + 1), WEST),
            (pos.0, pos.1 + 1)
        );
        assert_eq!(next_position(&map, pos, EAST), (pos.0 + 1, pos.1));
        assert_eq!(next_position(&map, (2, 0), EAST), (2, 0));
    }

    #[test]
    fn test_next_position_wraps_around() {
        let map: Vec<Vec<_>> = Vec::from(EX_MAP).into_iter().map(Vec::from).collect_vec();
        assert_eq!(next_position(&map, (2, 0), WEST), (4, 0));
        assert_eq!(next_position(&map, (4, 0), EAST), (2, 0));
        assert_eq!(next_position(&map, (0, 2), NORTH), (0, 3));
        assert_eq!(next_position(&map, (0, 3), SOUTH), (0, 2));
    }

    #[test]
    fn test_next_position_wraps_around_at_wall() {
        let map: Vec<Vec<_>> = Vec::from(EX_MAP).into_iter().map(Vec::from).collect_vec();
        assert_eq!(next_position(&map, (4, 4), EAST), (4, 4));
    }

    #[test]
    fn test_parse_hike() {
        use super::Step::*;
        use super::Turn::*;
        let ex = "10R5L5R10L4R5L5";
        let hike = super::parse_hike(ex).unwrap();
        assert_eq!(
            hike,
            vec![
                Forward(10),
                Orient(R),
                Forward(5),
                Orient(L),
                Forward(5),
                Orient(R),
                Forward(10),
                Orient(L),
                Forward(4),
                Orient(R),
                Forward(5),
                Orient(L),
                Forward(5)
            ]
        );
    }

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn test_example() {
        let (map, hike) = super::parse(EXAMPLE).unwrap();
        let (row, column, direction) = super::hike(&map, &hike);
        assert_eq!(row, 6);
        assert_eq!(column, 8);
        assert_eq!(direction, 0);
    }
}
