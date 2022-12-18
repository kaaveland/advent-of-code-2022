use std::collections::{VecDeque};
use std::io::{Read, stdin};

use anyhow::{Context, Result};

fn parse_input(inp: &str) -> Result<(usize, usize, Vec<u8>)> {
    let height = inp.lines().filter(|line| !line.is_empty()).count();
    let blines: Vec<_> = inp.lines()
        .filter(|line| !line.is_empty())
        .map(str::as_bytes).flatten().cloned()
        .collect();
    Ok((blines.len() / height, height, blines))
}

fn find_ends(landscape: &Vec<u8>) -> Result<(usize, usize)> {
    let start = landscape.iter()
        .enumerate()
        .filter(| (_, place) | **place == 'S' as u8)
        .map(| (loc, _) | loc)
        .next().context("Unable to locate S")?;
    let end = landscape.iter()
        .enumerate()
        .filter(| (_, place) | **place == 'E' as u8)
        .map(| (loc, _) | loc)
        .next().context("Unable to locate E")?;
    Ok((start, end))
}

fn generate_moves(source: usize, width: usize, height: usize) -> Vec<usize> {
    let left = if source > 0 { Some(source - 1) } else { None };
    let right = if source < height * width - 1 { Some(source + 1) } else { None };
    let down = if source + width < height * width { Some(source + width) } else { None };
    let up = if source >= width { Some(source - width) } else { None };
    let choices = vec![left, right, down, up];
    choices.iter().filter_map(|choice| *choice).collect()
}

fn elevation(place: u8) -> u8 {
    if place as char == 'S' {
        'a' as u8
    } else if place as char == 'E' {
        'z' as u8
    } else {
        place
    }
}

fn move_cost(source: usize, dest: usize, landscape: &Vec<u8>) -> usize {
    let source_val = elevation(landscape[source]);
    let dest_val = elevation(landscape[dest]);

    if dest_val >= source_val {
        (dest_val - source_val) as usize
    } else {
        0
    }
}

fn filter_moves(source: usize, moves: &Vec<usize>, landscape: &Vec<u8>) -> Vec<usize> {
    let can_go = | loc: &&usize | {
        move_cost(source, **loc, landscape) <= 1
    };
    moves.iter().filter(can_go).cloned().collect()
}

fn bfs<F>(
    source: usize, is_dest: F, landscape: &Vec<u8>, width: usize, height: usize
) -> Option<usize>
where F: Fn(usize) -> bool
{
    let mut queue: VecDeque<usize> = vec![source].into();
    let mut costs: Vec<usize> = Vec::with_capacity(landscape.len());
    for _ in 0..landscape.len() {
        costs.push(0);
    }

    while !queue.is_empty() {
        let now = queue.pop_front().unwrap(); // Safe -- not empty
        let cost_here = costs[now];

        if is_dest(now) {
            return Some(cost_here);
        }

        let next = generate_moves(now, width, height);
        for next in filter_moves(now, &next, &landscape) {
            if next != source && costs[next] == 0 {
                queue.push_back(next);
                costs[next] = cost_here + 1;
            }

        }
    }
    None
}

#[cfg(test)]
mod tests {

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
    #[test]
    fn test_parse_input() {
        let (width, height, landscape) =
            super::parse_input(EXAMPLE).expect("Unable to parse input");
        assert_eq!(width, 8);
        assert_eq!(height, 5);
        assert_eq!(landscape[0] as char, 'S');
        assert_eq!(landscape[21] as char, 'E');
    }

    #[test]
    fn test_find_endpoints() {
        let (_, _, landscape) =
            super::parse_input(EXAMPLE).expect("Unable to parse input");
        assert_eq!(super::find_ends(&landscape).expect("Parse error"), (0, 21));
    }

    #[test]
    fn test_find_moves() {
        let (width, height) = (8, 5);
        let moves_from_origin = super::generate_moves(0, width, height);
        assert_eq!(moves_from_origin, vec![1, width]);
        let moves_from_1 = super::generate_moves(1, width, height);
        assert_eq!(moves_from_1, vec![0, 2, width + 1]);
        let moves_from_last = super::generate_moves(width * height - 1, width, height);
        assert_eq!(moves_from_last, vec![width * height - 2, width * (height - 1) - 1]);
    }

    #[test]
    fn test_filter_moves() {
        let landscape: Vec<_> = "aaccSbaEd".as_bytes().iter().cloned().collect();
        let candidates = vec![0, 1, 2, 3, 5, 6, 7, 8];
        let possible = super::filter_moves(4, &candidates, &landscape);
        assert_eq!(possible, vec![0, 1, 5, 6]);
    }

    #[test]
    fn test_bfs() {
        let (width, height, landscape) = super::parse_input(EXAMPLE).expect("Parse error");
        let (source, _) = super::find_ends(&landscape).expect("No S or E");
        let cost = super::bfs(
            source, |source| landscape[source] as char == 'E', &landscape, width, height
        );
        assert_eq!(cost.is_some(), true);
        assert_eq!(cost.unwrap_or(0), 31);
    }
}


fn main() -> Result<()> {
    let mut inp = String::new();
    stdin().read_to_string(&mut inp)?;

    let (width, height, landscape) = parse_input(inp.as_str())?;
    let (source, dest) = find_ends(&landscape)?;
    let cost = bfs(
        source, |source| landscape[source] as char == 'E', &landscape, width, height
    ).context("Unable to find path")?;

    println!("Path cost: {}", cost);

    let inverted: Vec<_> = landscape.iter()
        .map(| b | ('z' as u8) - elevation(*b) + ('a' as u8)).collect();
    let cost = bfs(
        dest, |place| landscape[place] as char == 'a', &inverted, width, height
    ).context("Unable to find path")?;

    Ok(println!("Path cost: {}", cost))
}