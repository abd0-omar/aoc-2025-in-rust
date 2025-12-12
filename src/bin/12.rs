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

    fn parse<R: BufRead>(mut reader: R) -> (Vec<Vec<String>>, Vec<Region>) {
        let mut input = String::new();
        reader.read_to_string(&mut input).unwrap();

        let blocks: Vec<&str> = input.split("\n\n").collect();

        let (regions_block, shapes_blocks) = blocks.split_last().unwrap();

        let shapes = shapes_blocks
            .iter()
            .map(|block| block.lines().skip(1).map(|line| line.to_string()).collect())
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

        dbg!(&shapes);
        dbg!(&regions);

        (shapes, regions)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // no need to think about the solution now, let's just parse
        let result = 0;
        parse(reader);
        Ok(result)
    }

    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);
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
