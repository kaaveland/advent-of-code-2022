use std::io;

fn score(c: char) -> u32 {
    let ordinal = if c.is_uppercase() {
        c as u8 - 'A' as u8 + 27
    } else {
        c as u8 - 'a' as u8 + 1
    };
    ordinal as u32
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|line_result| line_result.unwrap_or("".to_string()))
        .filter(| line | !line.is_empty())
        .collect();

    let first_backpacks: Vec<&String> = lines.iter().step_by(3).collect();
    let second_backpacks: Vec<&String> = lines[1..].iter().step_by(3).collect();
    let third_backpacks: Vec<&String> = lines[2..].iter().step_by(3).collect();

    let groups = first_backpacks.iter().zip(second_backpacks.iter());
    let groups = groups.zip(third_backpacks.iter());

    let items: Vec<char> = groups.map(|group| {
        let ((one, two), three) = group;
        one.chars().into_iter().filter(| c | two.contains(*c) && three.contains(*c)).next()
    }).filter_map(| possibly_char | possibly_char).collect();

    let solution: u32 = items.iter().cloned().map(score).sum();
    println!("Solution: {}", solution);
}
