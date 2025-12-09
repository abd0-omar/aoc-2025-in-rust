use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> Vec<(i64, i64)> {
        reader
            .lines()
            .map(|line| {
                let parts = line
                    .unwrap()
                    .split(',')
                    .map(|part| part.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                (parts[1], parts[0])
            })
            .collect::<Vec<_>>()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        let cords = parse(reader);

        for i in 0..cords.len() {
            for j in i + 1..cords.len() {
                let area = ((cords[i].0 - cords[j].0) + 1) * ((cords[i].1 - cords[j].1) + 1);
                result = result.max(area);
            }
        }

        Ok(result as usize)
    }

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let cords = parse(reader);

        let mut unique_rows: HashSet<i64> = HashSet::new();
        let mut unique_cols: HashSet<i64> = HashSet::new();

        unique_rows.insert(0);
        unique_cols.insert(0);

        let max_y = cords.iter().map(|(y, _)| *y).max().unwrap() + 1;
        let max_x = cords.iter().map(|(_, x)| *x).max().unwrap() + 1;
        unique_rows.insert(max_y);
        unique_cols.insert(max_x);

        for &(r, c) in &cords {
            unique_rows.insert(r);
            unique_cols.insert(c);
        }

        let sorted_rows: Vec<i64> = {
            let mut v: Vec<_> = unique_rows.into_iter().collect();
            v.sort();
            v
        };
        let sorted_cols: Vec<i64> = {
            let mut v: Vec<_> = unique_cols.into_iter().collect();
            v.sort();
            v
        };

        let row_map: HashMap<i64, usize> = sorted_rows
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();
        let col_map: HashMap<i64, usize> = sorted_cols
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();

        let height = sorted_rows.len();
        let width = sorted_cols.len();
        let mut grid = vec![vec![0u8; width]; height];

        let len = cords.len();
        for i in 0..len {
            let (r1, c1) = cords[i];
            let (r2, c2) = cords[(i + 1) % len];

            if c1 == c2 {
                let c_idx = *col_map.get(&c1).unwrap();
                let r_start = *row_map.get(&r1.min(r2)).unwrap();
                let r_end = *row_map.get(&r1.max(r2)).unwrap();
                for r in r_start..=r_end {
                    grid[r][c_idx] = 1;
                }
            } else if r1 == r2 {
                let r_idx = *row_map.get(&r1).unwrap();
                let c_start = *col_map.get(&c1.min(c2)).unwrap();
                let c_end = *col_map.get(&c1.max(c2)).unwrap();
                for c in c_start..=c_end {
                    grid[r_idx][c] = 1;
                }
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        // flood fill outside
        while let Some((cr, cc)) = queue.pop_front() {
            for (dr, dc) in dirs {
                let nr = cr as isize + dr;
                let nc = cc as isize + dc;

                if nr >= 0 && nr < height as isize && nc >= 0 && nc < width as isize {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if grid[nr][nc] == 0 {
                        grid[nr][nc] = 2;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }

        let mut max_area = 0;

        for i in 0..cords.len() {
            for j in i + 1..cords.len() {
                let (r1, c1) = cords[i];
                let (r2, c2) = cords[j];

                let h_real = (r1 - r2).abs() + 1;
                let w_real = (c1 - c2).abs() + 1;
                let area = h_real * w_real;

                if area <= max_area {
                    continue;
                }

                let r_min_idx = *row_map.get(&r1.min(r2)).unwrap();
                let r_max_idx = *row_map.get(&r1.max(r2)).unwrap();
                let c_min_idx = *col_map.get(&c1.min(c2)).unwrap();
                let c_max_idx = *col_map.get(&c1.max(c2)).unwrap();

                let mut is_valid = true;
                'outer: for r in r_min_idx..=r_max_idx {
                    for c in c_min_idx..=c_max_idx {
                        if grid[r][c] == 2 {
                            is_valid = false;
                            break 'outer;
                        }
                    }
                }

                if is_valid {
                    max_area = area;
                }
            }
        }

        Ok(max_area as usize)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 2 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
