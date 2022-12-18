use std::io;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SectionRange(u32, u32);

#[cfg(test)]
const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

fn new_section(start: u32, end: u32) -> SectionRange {
    if start <= end {
        SectionRange(start, end)
    } else {
        SectionRange(end, start)
    }
}

#[cfg(test)]
#[test]
fn test_new_section() {
    let sr = new_section(4, 2);
    let SectionRange(start, end) = sr;
    assert_eq!(start, 2);
    assert_eq!(end, 4);
}

fn container_contains(containee: SectionRange, container: SectionRange) -> bool {
    let SectionRange(containee_start, containee_end) = containee;
    let SectionRange(container_start, container_end) = container;
    container_start <= containee_start && container_end >= containee_end
}

#[cfg(test)]
#[test]
fn test_fully_contained() {
    assert_eq!(
        container_contains(new_section(2, 4), new_section(1, 3)),
        false
    );
    assert_eq!(
        container_contains(new_section(2, 4), new_section(2, 4)),
        true
    );
    assert_eq!(
        container_contains(new_section(2, 4), new_section(1, 5)),
        true
    );
    assert_eq!(
        container_contains(new_section(1, 5), new_section(2, 4)),
        false
    );
}

fn one_is_fully_contained(left: SectionRange, right: SectionRange) -> bool {
    container_contains(left, right) || container_contains(right, left)
}

#[cfg(test)]
#[test]
fn test_one_contains() {
    assert_eq!(
        one_is_fully_contained(new_section(1, 5), new_section(2, 4)), true
    );
    assert_eq!(
        one_is_fully_contained(new_section(1, 5), new_section(3, 10)), false
    );
}

fn parse_section(section: &str) -> Option<SectionRange> {
    let split: Vec<&str> = section.splitn(2, "-").collect();
    if let [left, right] = split[..] {
        let start: u32 = left.parse().ok()?;
        let end: u32 = right.parse().ok()?;
        Some(new_section(start, end))
    } else {
        None
    }
}

#[cfg(test)]
#[test]
fn test_parse_section() {
    assert_eq!(parse_section("2-4"), Some(new_section(2, 4)));
    assert_eq!(parse_section("4-2"), Some(new_section(2, 4)));
    assert_eq!(parse_section("2-"), None);
    assert_eq!(parse_section("a-b"), None);
}

fn parse_sections(line: &str) -> Option<(SectionRange, SectionRange)> {
    let split: Vec<&str> =  line.splitn(2, ",").collect();
    if let [left, right] = split[..] {
        let left_sec = parse_section(left)?;
        let right_sec = parse_section(right)?;
        Some((left_sec, right_sec))
    } else {
        None
    }
}

#[cfg(test)]
#[test]
fn test_parse_sections() {
    assert_eq!(
        parse_sections("2-4,5-7"),
        Some((new_section(2, 4), new_section(5, 7)))
    );
    assert_eq!(
        parse_sections("2-"), None
    );
    assert_eq!(
        parse_sections("2,5"), None
    );
    assert_eq!(
        parse_section("2-4,5"), None
    );
}

fn part1_predicate(line: &str) -> Option<bool> {
    let (left_sec, right_sec) = parse_sections(line)?;
    Some(one_is_fully_contained(left_sec, right_sec))
}

fn predicate_count<'a, I, F>(lines: I, pred: F) -> u32
where
    I: IntoIterator<Item = &'a str>,
    F: Fn(&str) -> Option<bool>
{
    let mut count = 0;
    for x in lines {
        if let Some(true) = pred(x) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
#[test]
fn test_part1_example() {
    let lines: Vec<&str> = EXAMPLE.split("\n").collect();
    assert_eq!(predicate_count(lines, part1_predicate), 2);
}

fn overlaps(left: SectionRange, right: SectionRange) -> bool {
    let SectionRange(left_start, left_end) = left;
    let SectionRange(right_start, right_end) = right;
    left_start <= right_end && left_end >= right_start
}

fn part2_predicate(line: &str) -> Option<bool> {
    let (left, right) = parse_sections(line)?;
    Some(overlaps(left, right))
}

#[cfg(test)]
#[test]
fn test_part2_predicate() {
    assert_eq!(
        part2_predicate("2-4,3-5"), Some(true)
    );
    assert_eq!(
        part2_predicate("3-5,2-4"), Some(true)
    );
    assert_eq!(
        part2_predicate("2-4,2-4"), Some(true)
    );
    assert_eq!(
        part2_predicate("1-4,5-9"), Some(false)
    );
    assert_eq!(
        part2_predicate("1-4,5"), None
    );
    assert_eq!(
        part2_predicate("1,3-5"), None
    );
    assert_eq!(
        part2_predicate(""), None
    );
}

#[cfg(test)]
#[test]
fn test_part_2_example() {
    let lines: Vec<&str> = EXAMPLE.split("\n").collect();
    assert_eq!(predicate_count(lines, part2_predicate), 4);
}

fn main()
{
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|line_result| line_result.unwrap_or("".to_string()))
        .filter(|line| !line.is_empty())
        .collect();

    let p1_solution = predicate_count(lines.iter().map(String::as_str), part1_predicate);
    let p2_solution = predicate_count(lines.iter().map(String::as_str), part2_predicate);
    println!("Part 1 solution: {} Part 2 solution: {}", p1_solution, p2_solution);
}
