use std::io::stdin;

#[cfg(test)]
const EXAMPLE: &str = "30373
25512
65332
33549
35390
";

struct Forest {
    height: usize,
    width: usize,
    forest: Vec<u8>,
}

fn parse_forest(forest: &str) -> Forest {
    let mut rows = 0;
    let mut out = Vec::new();

    for line in forest.lines() {
        rows += 1;
        for height in line.as_bytes() {
            out.push(*height - ('0' as u8))
        }
    }

    Forest {
        height: rows,
        width: out.len() / rows,
        forest: out,
    }
}

fn index_forest(row: usize, col: usize, forest: &Forest) -> u8 {
    let index = row * forest.width + col;
    *forest.forest.get(index).expect("Out of bounds")
}

fn decrement_height(row: usize, col: usize, forest: &mut Forest) {
    let index = row * forest.width + col;
    forest.forest[index] -= 1;
}

#[cfg(test)]
#[test]
fn test_parse_example() {
    let forest = parse_forest(EXAMPLE);
    assert_eq!(forest.height, 5);
    assert_eq!(forest.width, 5);
    assert_eq!(index_forest(0, 0, &forest), 3);
    assert_eq!(index_forest(1, 1, &forest), 5);
    assert_eq!(index_forest(4, 2, &forest), 3);
}

fn calculate_visibility_map(forest: &Forest) -> Forest {
    let mut out_forest = Forest {
        height: forest.height,
        width: forest.width,
        forest: Vec::with_capacity(forest.width * forest.height),
    };
    for _ in 0..(forest.height * forest.width) {
        out_forest.forest.push(4 as u8);
    }
    for row in 0..forest.height {
        for col in 0..forest.width {
            let tree_height = index_forest(row, col, forest);

            for above in 0..row {
                if index_forest(above, col, forest) >= tree_height {
                    decrement_height(row, col, &mut out_forest);
                    break;
                }
            }
            for below in (row + 1)..forest.height {
                if index_forest(below, col, forest) >= tree_height {
                    decrement_height(row, col, &mut out_forest);
                    break;
                }
            }
            for left in 0..col {
                if index_forest(row, left, forest) >= tree_height {
                    decrement_height(row, col, &mut out_forest);
                    break;
                }
            }
            for right in (col + 1)..forest.width {
                if index_forest(row, right, forest) >= tree_height {
                    decrement_height(row, col, &mut out_forest);
                    break;
                }
            }
        }
    }
    out_forest
}
#[cfg(test)]
#[test]
fn test_calc_visibility_map() {
    let forest = parse_forest(EXAMPLE);
    let height_map = calculate_visibility_map(&forest);
    let visible = height_map.forest.iter().filter(|tree| *tree > &0).count();
    assert_eq!(visible, 21);
}

fn calculate_scenic_score_map(forest: &Forest) -> Vec<i32> {
    let mut out = Vec::new();

    for row in 1..forest.height - 1 {
        for col in 1..forest.width - 1 {
            let tree_height = index_forest(row, col, forest);
            let mut seen_left = 0;
            let mut seen_right = 0;
            for i in (0..col).rev() {
                seen_left += 1;
                if tree_height <= index_forest(row, i, forest) {
                    break;
                }
            }
            for i in col + 1..forest.width {
                seen_right += 1;
                if tree_height <= index_forest(row, i, forest) {
                    break;
                }
            }
            let mut seen_above = 0;
            let mut seen_below = 0;
            for i in (0..row).rev() {
                seen_above += 1;
                if tree_height <= index_forest(i, col, forest) {
                    break;
                }
            }
            for i in (row + 1)..forest.width {
                seen_below += 1;
                if tree_height <= index_forest(i, col, forest) {
                    break;
                }
            }

            let scenic = seen_left * seen_right * seen_above * seen_below;
            out.push(scenic);
        }
    }
    out
}

#[cfg(test)]
#[test]
fn test_scenic_visibility_map() {
    let forest = parse_forest(EXAMPLE);
    let height_map = calculate_scenic_score_map(&forest);
    let scenic = *height_map.iter().max().unwrap_or(&0);
    assert_eq!(scenic, 8);
}

fn main() {
    let input: Vec<String> = stdin()
        .lines()
        .map(|line| line.expect("IO error"))
        .filter(|line| !line.is_empty())
        .collect();
    let text = input.join("\n");
    let forest = parse_forest(text.as_str());
    print!("Made forest");
    let height_map = calculate_visibility_map(&forest);
    let visible = height_map.forest.iter().filter(|tree| *tree > &0).count();
    println!("Part 1: {}", visible);
    let scenic_map = calculate_scenic_score_map(&forest);
    let scenic = scenic_map.iter().max();
    println!("Part 2: {}", scenic.unwrap_or(&0));
}
