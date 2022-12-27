use anyhow::Result;
use aoc::io::read_stdin;
use itertools::Itertools;

fn score(c: char) -> u32 {
    let ordinal = if c.is_uppercase() {
        c as u8 - b'A' + 27
    } else {
        c as u8 - b'a' + 1
    };
    ordinal as u32
}

fn main() -> Result<()> {
    let inp = read_stdin()?;
    let lines: Vec<_> = inp.as_str().lines().collect();

    let part_1: u32 = inp
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mid = line.len() / 2;
            let item = &line[0..mid]
                .chars()
                .find(|ch| line[mid..line.len()].chars().contains(ch))
                .unwrap();
            score(*item)
        })
        .sum();

    println!("{part_1}");

    let first_backpacks: Vec<_> = lines.iter().step_by(3).collect();
    let second_backpacks: Vec<_> = lines[1..].iter().step_by(3).collect();
    let third_backpacks: Vec<_> = lines[2..].iter().step_by(3).collect();

    let groups = first_backpacks.iter().zip(second_backpacks.iter());
    let groups = groups.zip(third_backpacks.iter());

    let items: Vec<char> = groups
        .filter_map(|group| {
            let ((one, two), three) = group;
            one.chars()
                .into_iter()
                .find(|c| two.contains(*c) && three.contains(*c))
        })
        .collect();

    let solution: u32 = items.iter().cloned().map(score).sum();
    println!("Solution: {}", solution);

    Ok(())
}
