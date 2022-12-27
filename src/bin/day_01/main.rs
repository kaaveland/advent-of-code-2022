use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::io::{stdin, Read};

fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
    let mut calorie_groups: Vec<Vec<i32>> = Vec::new();
    let mut current = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            if !current.is_empty() {
                calorie_groups.push(current);
            }
            current = Vec::new();
        } else {
            let calories: i32 = line.parse()?;
            current.push(calories);
        }
    }
    if !current.is_empty() {
        calorie_groups.push(current);
    }
    Ok(calorie_groups)
}

fn largest_group(groups: &[Vec<i32>]) -> Result<i32> {
    groups
        .iter()
        .map(|v| v.iter().sum())
        .max()
        .context("Empty groups")
}

fn top_n(groups: &[Vec<i32>], n: usize) -> Result<i32> {
    let top_n: Vec<i32> = groups
        .iter()
        .map(|v| v.iter().sum())
        .sorted()
        .rev()
        .take(n)
        .collect();
    if top_n.len() != 3 {
        Err(Error::msg("Too few groups"))
    } else {
        Ok(top_n.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use crate::{largest_group, top_n};

    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_parse_input() {
        let groups = super::parse_input(EXAMPLE).expect("Unable to parse");
        assert_eq!(groups[0], vec![1000, 2000, 3000]);
        assert_eq!(groups[1], vec![4000]);
    }

    #[test]
    fn test_largest_group() {
        let groups = super::parse_input(EXAMPLE).expect("Unable to parse");
        assert_eq!(largest_group(&groups).expect("Unable to sum"), 24000);
    }

    #[test]
    fn test_top_n() {
        let groups = super::parse_input(EXAMPLE).expect("Unable to parse");
        assert_eq!(top_n(&groups, 3).expect("Too few n"), 45000);
    }
}
fn main() -> Result<()> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;
    let groups = parse_input(buf.as_str())?;
    let part_1 = largest_group(&groups)?;
    let part_2 = top_n(&groups, 3)?;
    println!("Part 1: {}, Part 2: {}", part_1, part_2);
    Ok(())
}
