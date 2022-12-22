use anyhow::{anyhow, Context, Result};
use aoc::io::read_stdin;
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
    let _rects = find_all_rects(&map);

    Ok(())
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Rect {
    north_west: (CoordSize, CoordSize),
    south_east: (CoordSize, CoordSize),
}

impl Rect {
    fn dimensions(&self) -> (CoordSize, CoordSize) {
        (
            self.south_east.0 - self.north_west.0 + 1,
            self.south_east.1 - self.north_west.1 + 1,
        )
    }
    fn contains(&self, point: (CoordSize, CoordSize)) -> bool {
        (self.north_west.0..=self.south_east.0).contains(&point.0)
            && (self.north_west.1..=self.south_east.1).contains(&point.1)
    }
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

fn find_north_west(map: &Map) -> (CoordSize, CoordSize) {
    let mut loc = (0, 0);
    while Some(Tile::Abyss) == lookup(map, loc) {
        match next_loc(map, loc, EAST) {
            None => {
                return loc;
            }
            Some(next) => {
                loc = next;
            }
        }
    }
    loc
}

fn scan(map: &Map, mut source: (CoordSize, CoordSize), dir: Direction) -> (CoordSize, CoordSize) {
    while Some(Tile::Abyss) != lookup(map, source) {
        match next_loc(map, source, dir) {
            None => {
                return source;
            }
            Some(next) => {
                source = next;
            }
        }
    }
    let back = face_back(dir);
    next_loc(map, source, back).unwrap()
}

fn find_first_rect(map: &Map) -> Rect {
    let nw = find_north_west(map);
    let ne = scan(map, nw, EAST);
    let se = scan(map, ne, SOUTH);
    let sw = scan(map, nw, SOUTH);
    let y = min(se.1, sw.1);
    let x = max(ne.0, se.0);

    Rect {
        north_west: nw,
        south_east: (x, y),
    }
}

fn expand_rectangle(
    map: &Map,
    source_corner: (CoordSize, CoordSize),
    look_dir: Direction,
    iter_dir: Direction,
) -> Rect {
    let opposite_edge = scan(map, source_corner, look_dir);
    let opposite_iter = scan(map, opposite_edge, iter_dir);
    let nw_x = min(opposite_edge.0, min(opposite_iter.0, source_corner.0));
    let nw_y = min(opposite_edge.1, min(opposite_iter.1, source_corner.1));
    let se_x = max(opposite_edge.0, max(opposite_iter.0, source_corner.0));
    let se_y = max(opposite_edge.1, max(opposite_iter.1, source_corner.1));
    Rect {
        north_west: (nw_x, nw_y),
        south_east: (se_x, se_y),
    }
}

fn find_neighbours(map: &Map, rect: &Rect, rects: &mut Vec<Rect>) {
    let nw = rect.north_west;
    let se = rect.south_east;
    let sw = (nw.0, se.1);
    let ne = (se.0, nw.1);

    let scans = vec![
        (nw, ne, EAST, NORTH),
        (nw, sw, SOUTH, WEST),
        (sw, se, EAST, SOUTH),
        (ne, se, SOUTH, EAST),
    ];

    for (mut source, dest, iter_dir, look_dir) in scans {
        if next_loc(&map, source, look_dir).is_none() {
            continue;
        }
        while source != dest {
            // Safe by invariant: already checked above
            let maybe_edge = next_loc(&map, source, look_dir).unwrap();
            if lookup(map, maybe_edge) != Some(Tile::Abyss)
                && !rects.iter().any(|rect| rect.contains(maybe_edge))
            {
                let rect = expand_rectangle(map, source, look_dir, iter_dir);
                if !rects.contains(&rect) {
                    rects.push(rect.clone());
                    find_neighbours(map, &rect, rects);
                }
                break;
            } else {
                // Safe by invariant: It's not dest
                source = next_loc(map, source, iter_dir).unwrap();
            }
        }
    }
}

fn find_all_rects(map: &Map) -> Vec<Rect> {
    let first = find_first_rect(&map);
    let mut acc = vec![first.clone()];
    find_neighbours(map, &first, &mut acc);
    acc
}

fn find_cube_dim(rects: &Vec<Rect>) -> CoordSize {
    rects
        .iter()
        .map(|rect| {
            let dim = rect.dimensions();
            min(dim.0, dim.1)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::Tile::*;
    use crate::{
        find_cube_dim, find_first_rect, find_neighbours, find_north_west, next_position, parse,
        scan, CoordSize, Tile,
    };
    use crate::{EAST, NORTH, SOUTH, WEST};
    use itertools::Itertools;

    #[test]
    fn test_cube_dim() {
        let (map, _) = parse(EXAMPLE).unwrap();
        let rects = super::find_all_rects(&map);
        assert_eq!(find_cube_dim(&rects), 4);
    }
    #[test]
    fn test_find_all_rects() {
        let map: Vec<Vec<_>> = Vec::from(EX_MAP).into_iter().map(Vec::from).collect_vec();
        let rects = super::find_all_rects(&map);
        for x in 0..map[0].len() {
            for y in 0..map.len() {
                assert!(rects
                    .iter()
                    .any(|rect| rect.contains((x as CoordSize, y as CoordSize))
                        || map[y][x] == Abyss));
            }
        }
        let (map, _) = parse(EXAMPLE).unwrap();
        let rects = super::find_all_rects(&map);
        for x in 0..map[0].len() {
            for y in 0..map.len() {
                assert!(rects
                    .iter()
                    .any(|rect| rect.contains((x as CoordSize, y as CoordSize))
                        || map[y][x] == Abyss));
            }
        }
    }

    #[test]
    fn test_north_west() {
        let map: Vec<Vec<_>> = Vec::from(EX_MAP).into_iter().map(Vec::from).collect_vec();
        let nw = find_north_west(&map);
        assert_eq!(nw, (2, 0));
    }

    #[test]
    fn test_scan() {
        let map: Vec<Vec<_>> = Vec::from(EX_MAP).into_iter().map(Vec::from).collect_vec();
        let nw = find_north_west(&map);
        let ne = scan(&map, nw, EAST);
        assert_eq!(ne, (4, 0));
        let sw = scan(&map, nw, SOUTH);
        assert_eq!(sw, (2, 5));
        let se = scan(&map, sw, EAST);
        assert_eq!(se, (4, 5));
    }

    #[test]
    fn test_first_rect() {
        let map: Vec<Vec<_>> = Vec::from(EX_MAP).into_iter().map(Vec::from).collect_vec();
        let rect = find_first_rect(&map);
        assert_eq!(rect.north_west, (2, 0));
        assert_eq!(rect.south_east, (4, 1));
        let (map, _) = parse(EXAMPLE).unwrap();
        let rect = find_first_rect(&map);
        assert_eq!(rect.north_west, (8, 0));
        assert_eq!(rect.south_east, (11, 11));
    }

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
        let (map, hike) = parse(EXAMPLE).unwrap();
        let (row, column, direction) = super::hike(&map, &hike);
        assert_eq!(row, 6);
        assert_eq!(column, 8);
        assert_eq!(direction, 0);
    }
}
