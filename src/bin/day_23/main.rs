use anyhow::Result;
use aoc::io::read_stdin;
use aoc::point2d::{Point2d, Rect};
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use itertools::Itertools;
type Elf = Point2d<i64>;
type Board = HashSet<Elf>;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn prioritized(self) -> Vec<Direction> {
        let mut out = vec![self];
        while out.len() < 4 {
            let last = out.last().unwrap();
            out.push(last.next());
        }
        out
    }

    fn next(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => West,
            West => East,
            East => North,
        }
    }

    fn of(&self, point: &Elf) -> HashSet<Elf> {
        use Direction::*;
        match self {
            North => vec![point.northwest(), point.north(), point.northeast()],
            West => vec![point.northwest(), point.west(), point.southwest()],
            East => vec![point.northeast(), point.east(), point.southeast()],
            South => vec![point.southwest(), point.south(), point.southeast()],
        }
        .into_iter()
        .collect()
    }

    fn adjust(&self, elf: &Elf) -> Elf {
        use Direction::*;
        match self {
            North => elf.north(),
            West => elf.west(),
            East => elf.east(),
            South => elf.south(),
        }
    }
}

fn parse_board(input: &str) -> Board {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(move |(y, row)| {
            row.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '#' {
                    Some((x as i64, y as i64).into())
                } else {
                    None
                }
            })
        })
        .sorted()
        .collect()
}

fn next_board(board: &Board, current_dir: &Direction) -> Board {
    let mut claims_vec: Vec<_> = Vec::with_capacity(board.len());
    let mut claim_counter: HashMap<_, i32> = HashMap::default();
    let mut next_board: Board = Board::default();

    for elf in board {
        if board
            .intersection(&elf.around().into_iter().collect())
            .next()
            .is_some()
        {
            let mut claimed = false;
            for dir in current_dir.prioritized() {
                if board.intersection(&dir.of(elf)).count() == 0 {
                    let claim = dir.adjust(elf);
                    claims_vec.push(claim);
                    *claim_counter.entry(claim).or_default() += 1;
                    claimed = true;
                    break;
                }
            }
            if !claimed {
                claims_vec.push(*elf);
            }
        } else {
            claims_vec.push(*elf);
        }
    }

    assert_eq!(claims_vec.len(), board.len());

    for (claim, elf) in claims_vec.into_iter().zip(board.iter()) {
        if *claim_counter.get(&claim).unwrap_or(&0) <= 1 {
            // Grant it
            next_board.insert(claim);
        } else {
            next_board.insert(*elf);
        }
    }

    next_board
}

fn run_part_1(input: &str) -> i64 {
    let mut board = parse_board(input);
    let mut dir = Direction::North;
    for _ in 0..10 {
        board = next_board(&board, &dir);
        dir = dir.next();
    }
    let elves = board.len() as i64;
    let rect = Rect::bound(board.iter().cloned());
    rect.area() - elves
}

fn run_part_2(input: &str) -> i64 {
    let mut board = parse_board(input);
    let mut dir = Direction::North;
    let mut i = 1;
    loop {
        let next_board = next_board(&board, &dir);
        if next_board == board {
            return i;
        } else {
            board = next_board;
        }
        dir = dir.next();
        i += 1;
    }
}

fn main() -> Result<()> {
    let inp = read_stdin()?;
    let part_1 = run_part_1(inp.as_str());
    println!("{part_1}");
    let part_2 = run_part_2(inp.as_str());
    println!("{part_2}");

    Ok(())
}

#[cfg(test)]
pub mod tests {

    use crate::{next_board, parse_board, run_part_1, run_part_2, Direction};

    const EXAMPLE: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn test_parse_board() {
        let board = parse_board(EXAMPLE);
        assert!(board.contains(&(4, 0).into()));
        assert!(board.contains(&(3, 1).into()));
    }

    #[test]
    fn test_board_once() {
        let board = parse_board(EXAMPLE);
        let next_board = next_board(&board, &Direction::North);
        assert_eq!(board.len(), next_board.len());
    }

    #[test]
    fn test_part_1() {
        let empty_tiles = run_part_1(EXAMPLE);
        assert_eq!(empty_tiles, 110);
    }

    #[test]
    fn test_part_2() {
        let p2 = run_part_2(EXAMPLE);
        assert_eq!(p2, 20);
    }
}
