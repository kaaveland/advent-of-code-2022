use anyhow::{anyhow, Context, Result};
use aoc::io::read_stdin;
use itertools::Itertools;
use std::cmp::{max, min};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
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

fn face_back(direction: Direction) -> Direction {
    turn(turn(direction, Turn::L), Turn::L).rem_euclid(4)
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
    let squares = find_squares(&map).context("Unable to locate any squares!")?;
    let simple_edges = find_connected_edges(&squares);
    println!("{squares:?}");
    println!("{simple_edges:?}");
    Ok(())
}

fn lookup(map: &Map, point: (CoordSize, CoordSize)) -> Option<Tile> {
    if (0..map.len()).contains(&(point.1 as usize))
        && (0..map[0].len()).contains(&(point.0 as usize))
    {
        Some(map[point.1 as usize][point.0 as usize])
    } else {
        None
    }
}

fn next_loc(
    map: &Map,
    point: (CoordSize, CoordSize),
    dir: Direction,
) -> Option<(CoordSize, CoordSize)> {
    let (dx, dy) = heading(dir);
    let nx = point.0 + dx;
    let ny = point.1 + dy;
    if (0..map.len()).contains(&(ny as usize)) && (0..map[1].len()).contains(&(nx as usize)) {
        Some((nx, ny))
    } else {
        None
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Square {
    northwest: (CoordSize, CoordSize),
    dim: CoordSize,
}

// It's a cube, so there must be 6 faces
fn find_squares(map: &Map) -> Option<Vec<Square>> {
    // We know that there are 6 squares, so by finding the total area
    // and dividing it by 6, we know that we have the area of 1 square
    // Taking the sqrt gives us the size of a side
    let area: usize = map
        .iter()
        .map(|row| row.iter().filter(|&tile| *tile != Tile::Abyss).count())
        .sum();
    if area % 6 != 0 {
        return None;
    }

    let face_area = area / 6;
    let face_dim = (face_area as f64).sqrt() as usize;

    if face_dim * face_dim != face_area {
        return None;
    }
    if map.len() % face_dim != 0 || map[0].len() % face_dim != 0 {
        return None;
    }
    let map_height = map.len() / face_dim;
    let map_width = map[0].len() / face_dim;
    let origins = (0..map_height)
        .map(|y| (0..map_width).map(|x| (y, x)).collect_vec())
        .flatten()
        .filter(|(y, x)| map[y * face_dim][x * face_dim] != Tile::Abyss)
        .collect_vec();

    if origins.len() != 6 {
        return None;
    }
    let squares = origins
        .into_iter()
        .map(|(y, x)| Square {
            northwest: ((face_dim * x) as CoordSize, (face_dim * y) as CoordSize),
            dim: face_dim as CoordSize,
        })
        .collect_vec();

    return Some(squares);
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct EdgeType {
    direction: Direction,
    //normal: Nothing,
}

fn manhattan_dist(left: (CoordSize, CoordSize), right: (CoordSize, CoordSize)) -> CoordSize {
    (left.0 - right.0).abs() + (left.1 - right.1).abs()
}

impl Square {
    fn nw(&self) -> (CoordSize, CoordSize) {
        self.northwest
    }
    fn ne(&self) -> (CoordSize, CoordSize) {
        (self.northwest.0 + self.dim - 1, self.northwest.1)
    }
    fn sw(&self) -> (CoordSize, CoordSize) {
        (self.northwest.0, self.northwest.1 + self.dim - 1)
    }
    fn se(&self) -> (CoordSize, CoordSize) {
        (
            self.northwest.0 + self.dim - 1,
            self.northwest.1 + self.dim - 1,
        )
    }

    // Identify _simple_ connection -- using any of these won't change
    // the direction
    fn connection(&self, other: &Square) -> Option<Direction> {
        if manhattan_dist(self.nw(), other.ne()) <= 1 {
            Some(WEST)
        } else if manhattan_dist(self.nw(), other.sw()) <= 1 {
            Some(NORTH)
        } else if manhattan_dist(self.sw(), other.nw()) <= 1 {
            Some(SOUTH)
        } else if manhattan_dist(self.ne(), other.nw()) <= 1 {
            Some(EAST)
        } else {
            None
        }
    }
}

fn find_connected_edges(squares: &Vec<Square>) -> Vec<(usize, usize, Direction)> {
    // Start by finding all the simple connections
    squares
        .iter()
        .enumerate()
        .map(|(src, origin)| {
            squares
                .iter()
                .enumerate()
                .filter_map(|(dst, candidate)| {
                    origin.connection(candidate).map(|dir| (src, dst, dir))
                })
                .collect_vec()
        })
        .flatten()
        .collect_vec()
}

#[cfg(test)]
pub mod tests {
    use crate::Tile::*;
    use crate::{
        find_connected_edges, find_squares, next_position, parse, CoordSize, Square, Tile,
    };
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
    fn test_finds_simply_connected_edges_on_example() {
        let (map, _) = parse(EXAMPLE).unwrap();
        let squares = find_squares(&map).unwrap();
        let edges = find_connected_edges(&squares);
        assert!(!edges.is_empty());
        assert_eq!(edges.len() % 2, 0);
        println!("{edges:?}");
    }

    #[test]
    fn test_finds_cube_faces_on_example() {
        let (map, _) = parse(EXAMPLE).unwrap();
        assert!(find_squares(&map).is_some());
        let squares = find_squares(&map).unwrap();
        println!("{squares:?}");
        assert!(squares.contains(&Square {
            northwest: (8, 0),
            dim: 4
        }));
        assert!(squares.contains(&Square {
            northwest: (0, 4),
            dim: 4
        }));
        assert!(squares.contains(&Square {
            northwest: (4, 4),
            dim: 4
        }));
        assert!(squares.contains(&Square {
            northwest: (8, 4),
            dim: 4
        }));
        assert!(squares.contains(&Square {
            northwest: (8, 8),
            dim: 4
        }));
        assert!(squares.contains(&Square {
            northwest: (12, 8),
            dim: 4
        }));
    }

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
        let (map, hike) = parse(EXAMPLE).unwrap();
        let (row, column, direction) = super::hike(&map, &hike);
        assert_eq!(row, 6);
        assert_eq!(column, 8);
        assert_eq!(direction, 0);
    }
}
