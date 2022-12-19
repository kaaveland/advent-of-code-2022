use itertools::Itertools;
use std::cmp::max;

const SHAPES: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
";

type Shape = Vec<(i64, i64)>;

fn shapes() -> Vec<Shape> {
    vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],             // hline
        vec![(0, -1), (1, 0), (1, -1), (1, -2), (2, -1)], // cross
        vec![(0, 0), (1, 0), (2, 0), (2, -1), (2, -2)],   // reverse L
        vec![(0, 0), (0, -1), (0, -2), (0, -3)],          // vline
        vec![(0, 0), (0, -1), (1, 0), (1, -1)],           // square
    ]
}
#[derive(PartialEq, Eq, Debug)]
pub enum Jet {
    Left,
    Right,
}

fn shift(shape: &mut Shape, dir: (i64, i64)) {
    for (x, y) in shape.iter_mut() {
        *x += dir.0;
        *y += dir.1;
    }
}

type Jets = Vec<Jet>;
fn parse_jets(input: &str) -> Jets {
    input
        .chars()
        .filter(|&ch| ch == '<' || ch == '>')
        .map(|ch| if ch == '<' { Jet::Left } else { Jet::Right })
        .collect_vec()
}

const CHAMBER_WIDTH: i64 = 7;
const MAX_HEIGHT: usize = 10000;

fn in_bounds(shape: &Shape) -> bool {
    shape
        .iter()
        .all(|&(x, y)| x >= 0 && x < CHAMBER_WIDTH && y >= 0)
}

fn drop_rock(
    jets: &Jets,
    shape: &Shape,
    mut time: usize,
    max_heights: &[i64; CHAMBER_WIDTH as usize],
    grid: &mut [Vec<bool>],
) -> (usize, [i64; CHAMBER_WIDTH as usize]) {
    let height = max_heights.iter().max().unwrap();
    //  bottom left
    let mut shape = shape.clone();
    let mut y_low = shape.iter().map(|&(_, y)| y).min().unwrap();
    shift(&mut shape, (2, height + 3 + y_low.abs()));
    println!("Min shape y = {:?}", height + 3 + y_low.abs());

    loop {
        // Invariant: we're in bounds here, enforce it
        if !in_bounds(&shape) {
            panic!("Out of bounds {time} {shape:?}");
        }
        let jet = &jets[time % jets.len()];
        println! {"{jet:?}"};
        let (forward, backward) = match jet {
            Jet::Right => ((1, 0), (-1, 0)),
            Jet::Left => ((-1, 0), (1, 0)),
        };
        time += 1;
        let (down, up) = ((0, -1), (0, 1));

        shift(&mut shape, forward);

        if shape
            .iter()
            .any(|&(x, y)| x < 0 || x >= CHAMBER_WIDTH || grid[y as usize][x as usize])
        {
            // Preserve invariant
            shift(&mut shape, backward);
        }

        shift(&mut shape, down);
        if shape
            .iter()
            .any(|&(x, y)| y < 0 || grid[y as usize][x as usize])
        {
            // Preserve invariant
            shift(&mut shape, up);
            break;
        }
    }

    let mut max_height_out = *max_heights;

    for (x, y) in shape {
        grid[y as usize][x as usize] = true;
        max_height_out[x as usize] = max(y + 1, max_height_out[x as usize]);
    }

    (time, max_height_out)
}

fn drop_many_rocks(jets: &Jets, rocks_to_drop: usize) -> i64 {
    let shapes = shapes();
    let mut grid = vec![vec![false; CHAMBER_WIDTH as usize]; MAX_HEIGHT as usize];
    let mut max_heights = [0; CHAMBER_WIDTH as usize];
    let mut time = 0;
    for rock_number in 0..rocks_to_drop {
        let shape = &shapes[rock_number % shapes.len()];
        (time, max_heights) = drop_rock(jets, shape, time, &max_heights, &mut grid);
        println!("Time: {time} {max_heights:?}");
    }
    for row in (0..20).rev() {
        let ch: String = (0..CHAMBER_WIDTH)
            .map(|x| {
                if grid[row as usize][x as usize] {
                    '#'
                } else {
                    '.'
                }
            })
            .collect();
        println!("{ch}");
    }
    *max_heights.iter().max().unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::{drop_many_rocks, drop_rock, shapes, Jet};
    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn parse_jets() {
        let jets = super::parse_jets(EXAMPLE);
        assert_eq!(jets[0], Jet::Right);
        assert_eq!(jets[3], Jet::Left);
    }

    #[test]
    fn print_shapes() {
        let mut i = 0;
        for mut shape in shapes() {
            let xmin = shape.iter().map(|&(x, _)| x).min().unwrap();
            let ymin = shape.iter().map(|&(_, y)| y).min().unwrap();
            super::shift(&mut shape, (-xmin, -ymin));
            let mut disp = [[false; 5]; 5];
            for (x, y) in shape {
                disp[y as usize][x as usize] = true;
            }
            println!("Shape: {i}");
            i += 1;
            for row in (0..5).rev() {
                let ch: String = (0..5)
                    .map(|r| if disp[row][r] { '#' } else { '.' })
                    .collect();
                println!("{ch}");
            }
        }
    }

    #[test]
    fn test_drop_many_rocks() {
        let jets = super::parse_jets(EXAMPLE);
        let answer = drop_many_rocks(&jets, 3);
        assert_eq!(answer, 3068);
    }
}
fn main() {}
