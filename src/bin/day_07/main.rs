use anyhow::Result;
use aoc::io::read_stdin;

fn dir_traversal(inp: &str) -> Vec<i64> {
    let mut stack = vec![0];
    let mut out = vec![];

    for line in inp.lines().filter(|line| !line.is_empty()) {
        let mut parts = line.split_ascii_whitespace();

        if line.starts_with("$ ls") || line.starts_with("dir") || line.starts_with("$ cd /") {
            continue;
        } else if line.starts_with("$ cd ..") {
            out.push(stack.pop().unwrap());
        } else if line.starts_with("$ cd ") {
            stack.push(0);
        } else {
            let num = parts
                .next()
                .and_then(|num| num.parse::<i64>().ok())
                .unwrap_or(0);
            for i in 0..stack.len() {
                stack[i] += num;
            }
        }
    }
    out.extend(stack);
    out
}

fn part_1(inp: &str) -> i64 {
    let folder_sizes = dir_traversal(inp);
    folder_sizes.iter().filter(|&size| *size <= 100000).sum()
}

fn part_2(inp: &str) -> i64 {
    let cap = 70000000;
    let req = 30000000;
    let sizes = dir_traversal(inp);
    let used = *sizes.iter().max().unwrap_or(&0);
    let free = cap - used;
    sizes
        .into_iter()
        .filter(|&size| size + free >= req)
        .min()
        .unwrap_or(0)
}

fn main() -> Result<()> {
    let inp = read_stdin()?;
    let p1 = part_1(inp.as_str());
    println!("{p1}");
    let p2 = part_2(inp.as_str());
    println!("{p2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_part_1() {
        let ans = part_1(EXAMPLE);
        assert_eq!(ans, 95437);
    }

    #[test]
    fn test_part_2() {
        let ans = part_2(EXAMPLE);
        assert_eq!(ans, 24933642);
    }
}
