use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;
        let mut current_distance = 50i64;

        for line in reader.lines() {
            let line = line.unwrap().into_bytes();

            // println!("{:?}", &String::from_utf8_lossy(&line));

            let direction = line[0];

            let mut distance = 0i64;
            for digit in line.iter().skip(1) {
                let digit = digit - b'0';
                // dbg!(digit);
                // dbg!(distance);
                distance = (distance * 10 + digit as i64).rem_euclid(100);
            }

            if direction == b'L' {
                current_distance -= distance as i64;
            } else {
                current_distance += distance as i64;
            }

            // dbg!(direction);
            // dbg!(distance);

            current_distance = current_distance.rem_euclid(100);

            // dbg!(current_distance);
            if current_distance == 0 {
                result += 1;
            }
        }

        Ok(result)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
