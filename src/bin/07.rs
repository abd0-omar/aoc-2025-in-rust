use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

// [0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14]
// [F, F, F, F, F, F, F, F, F, F, F, F, F, F, F]
// [F, F, F, F, F, F, F, T, F, F, F, F, F, F, F]
// [F, F, F, F, F, F, T, F, T, F, F, F, F, F, F]

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        let tree = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

        let n = tree[0].len();
        let mut beams = vec![false; n];

        for line in tree {
            for (idx, symbol) in line.chars().enumerate() {
                match symbol {
                    'S' => beams[idx] = true,
                    '^' => {
                        if beams[idx] {
                            beams[idx.saturating_sub(1)] = true;
                            beams[(idx + 1).min(n - 1)] = true;
                            beams[idx] = false;
                            result += 1;
                        }
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }

        Ok(result)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let tree = reader
            .lines()
            .map(|l| l.unwrap().chars().collect())
            .collect::<Vec<Vec<char>>>();

        let rows = tree.len();
        let cols = tree[0].len();

        let mut memo = vec![vec![None; cols]; rows];

        let start_col = tree[0].iter().position(|&s| s == 'S').unwrap();

        let result = dp(&tree, &mut memo, 0, start_col);

        Ok(result)
    }

    fn dp(tree: &[Vec<char>], memo: &mut Vec<Vec<Option<usize>>>, idx: usize, j: usize) -> usize {
        if idx == tree.len() {
            return 1;
        }

        if let Some(saved_result) = memo[idx][j] {
            return saved_result;
        }

        let symbol = tree[idx][j];
        let width = tree[idx].len();

        let result = if symbol == '^' {
            let take_left = {
                if j > 0 {
                    dp(tree, memo, idx + 1, j - 1)
                } else {
                    0
                }
            };

            let take_right = {
                if j + 1 < width {
                    dp(tree, memo, idx + 1, j + 1)
                } else {
                    0
                }
            };

            take_left + take_right
        } else {
            dp(tree, memo, idx + 1, j)
        };

        memo[idx][j] = Some(result);

        result
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 2 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
