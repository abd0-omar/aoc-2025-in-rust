use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    // #[derive(Clone, Debug)]
    // struct PaperRoll;

    // impl PaperRoll {
    //     fn parse(value: u8) -> Option<Self> {
    //         match value {
    //             b'.' => None,
    //             b'@' => Some(Self),
    //             _ => unreachable!(),
    //         }
    //     }
    // }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        let graph = reader
            .lines()
            .map(|l| l.unwrap().into_bytes())
            .collect::<Vec<_>>();

        for i in 0..graph.len() {
            'outer: for j in 0..graph[0].len() {
                if graph[i][j] == b'.' {
                    continue;
                }

                if graph[i][j] == b'@' {
                    let mut adjacent_paper_rolls = 0;
                    // traverse neighbors
                    for (ni, nj) in [
                        (i, j + 1),
                        (i + 1, j),
                        (i.wrapping_sub(1), j),
                        (i, j.wrapping_sub(1)),
                        (i + 1, j + 1),
                        (i.wrapping_sub(1), j.wrapping_sub(1)),
                        (i + 1, j.wrapping_sub(1)),
                        (i.wrapping_sub(1), j + 1),
                    ] {
                        // if !(0..graph.len()).contains(&ni) || !(0..graph[0].len()).contains(&nj) {
                        if ni >= graph.len() || nj >= graph[0].len() {
                            continue;
                        }

                        if graph[ni][nj] == b'@' {
                            adjacent_paper_rolls += 1;
                            if adjacent_paper_rolls == 4 {
                                // exceeded the amout of paper rolls that should be around it
                                continue 'outer;
                            }
                        }
                    }
                }
                // if it reaches here, means it's a valid papaer roll
                // dbg!((i, j));
                result += 1;
            }
        }

        Ok(result)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample test end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        let mut graph = reader
            .lines()
            .map(|l| l.unwrap().into_bytes())
            .collect::<Vec<_>>();

        'infinte_power: loop {
            let mut visited = vec![vec![false; graph[0].len()]; graph.len()];

            for i in 0..graph.len() {
                'outer: for j in 0..graph[0].len() {
                    if graph[i][j] == b'.' {
                        continue;
                    }

                    if graph[i][j] == b'@' {
                        let mut adjacent_paper_rolls = 0;
                        // traverse neighbors
                        for (di, dj) in [
                            (0, 1),
                            (1, 0),
                            (-1, 0),
                            (0, -1),
                            (1, 1),
                            (-1, -1),
                            (1, -1),
                            (-1, 1),
                        ] {
                            let ni = i.wrapping_add(di as usize);
                            let nj = j.wrapping_add(dj as usize);

                            // if !(0..graph.len()).contains(&ni) || !(0..graph[0].len()).contains(&nj) {
                            if ni >= graph.len() || nj >= graph[0].len() {
                                continue;
                            }

                            if graph[ni][nj] == b'@' {
                                adjacent_paper_rolls += 1;
                                if adjacent_paper_rolls == 4 {
                                    // exceeded the amout of paper rolls that should be around it
                                    continue 'outer;
                                }
                            }
                        }
                    }
                    // if it reaches here, means it's a valid papaer roll
                    // dbg!((i, j));
                    visited[i][j] = true;
                    result += 1;
                }
            }

            // dbg!(&visited);

            let mut flag = false;

            for i in 0..graph.len() {
                for j in 0..graph[0].len() {
                    if visited[i][j] {
                        graph[i][j] = b'.';
                        flag = true;
                    }
                }
            }

            if !flag {
                break 'infinte_power;
            }
        }

        Ok(result)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 2 sample test end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
