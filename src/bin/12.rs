use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(Debug)]
    struct Region {
        width: usize,
        height: usize,
        shapes: Vec<usize>,
    }

    fn parse<R: BufRead>(mut reader: R) -> (Vec<Vec<Vec<char>>>, Vec<Region>) {
        let mut input = String::new();
        reader.read_to_string(&mut input).unwrap();

        let blocks: Vec<&str> = input.split("\n\n").collect();

        let (regions_block, shapes_blocks) = blocks.split_last().unwrap();

        let shapes = shapes_blocks
            .iter()
            .map(|block| {
                block
                    .lines()
                    .skip(1)
                    .map(|line| line.chars().collect())
                    .collect()
            })
            .collect();

        let regions = regions_block
            .lines()
            .map(|line| {
                let (dimension, shapes) = line.split_once(": ").unwrap();

                let (width, height) = dimension.split_once('x').unwrap();

                Region {
                    width: width.parse().unwrap(),
                    height: height.parse().unwrap(),
                    shapes: shapes
                        .split_ascii_whitespace()
                        .map(|sh| sh.parse().unwrap())
                        .collect(),
                }
            })
            .collect();

        (shapes, regions)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // no need to think about the solution now, let's just parse
        let mut result = 0;
        let (shapes, regions) = parse(reader);
        for region in regions {
            let mut empty_region = vec![vec!['.'; region.width]; region.height];
            if backtrack(&shapes, &mut empty_region, 0, &region.shapes, 0, 0, 0) {
                result += 1;
            }
        }
        Ok(result)
    }

    fn backtrack(
        shapes: &Vec<Vec<Vec<char>>>,
        empty_region: &mut Vec<Vec<char>>,
        idx: usize,
        region_shapes: &Vec<usize>,
        st_i: usize,
        st_j: usize,
        fill_count: usize,
    ) -> bool {
        if idx == region_shapes.len() {
            return if fill_count == idx { true } else { false };
        }

        for region_shape_idx in idx..region_shapes.len() {
            let region_shape_count = region_shapes[region_shape_idx];

            for _ in 0..region_shape_count {
                let cur_shape = &shapes[region_shape_idx];
                // fill empty region
                if fill_region(empty_region, cur_shape, st_i, st_j) {
                    if st_j == empty_region[0].len() - 1 {
                        backtrack(
                            shapes,
                            empty_region,
                            idx + 1,
                            region_shapes,
                            st_i + 1,
                            0,
                            fill_count + 1,
                        );
                    } else {
                        backtrack(
                            shapes,
                            empty_region,
                            idx + 1,
                            region_shapes,
                            st_i,
                            st_j + 1,
                            fill_count + 1,
                        );
                    }
                    unfill_region(empty_region, cur_shape, st_i, st_j);
                }

                // backtrack without filling
                if st_j == empty_region[0].len() - 1 {
                    backtrack(
                        shapes,
                        empty_region,
                        idx + 1,
                        region_shapes,
                        st_i + 1,
                        0,
                        fill_count,
                    );
                } else {
                    backtrack(
                        shapes,
                        empty_region,
                        idx + 1,
                        region_shapes,
                        st_i,
                        st_j + 1,
                        fill_count,
                    );
                }
            }
        }

        false
    }

    fn fill_region(
        empty_region: &mut Vec<Vec<char>>,
        cur_shape: &Vec<Vec<char>>,
        st_i_empty_region: usize,
        st_j_empty_region: usize,
    ) -> bool {
        if st_j_empty_region + cur_shape[0].len() >= empty_region[0].len()
            || st_i_empty_region + cur_shape.len() >= empty_region.len()
        {
            return false;
        }

        for i in 0..cur_shape.len() {
            for j in 0..cur_shape[0].len() {
                if cur_shape[i][j] == '#' {
                    if empty_region[st_i_empty_region + i][st_j_empty_region + j] == '#' {
                        return false;
                    }
                }
            }
        }

        for i in 0..cur_shape.len() {
            for j in 0..cur_shape[0].len() {
                if cur_shape[i][j] == '#' {
                    empty_region[st_i_empty_region + i][st_j_empty_region + j] = '#';
                }
            }
        }

        true
    }

    fn unfill_region(
        empty_region: &mut Vec<Vec<char>>,
        cur_shape: &Vec<Vec<char>>,
        st_i_empty_region: usize,
        st_j_empty_region: usize,
    ) {
        for i in 0..cur_shape.len() {
            for j in 0..cur_shape[0].len() {
                if cur_shape[i][j] == '#' {
                    empty_region[st_i_empty_region + i][st_j_empty_region + j] = '.';
                }
            }
        }
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    // let result = 0;
    // Ok(result)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    // println!("=== Part 2 sample end ===");
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
