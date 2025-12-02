use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> Vec<(Vec<char>, Vec<char>)> {
        let line = reader.lines().next().unwrap().unwrap();
        let mut ranges = Vec::new();

        for part in line.split(',') {
            let l: Vec<char> = part.chars().collect();

            let dash = l.iter().position(|&x| x == '-').unwrap();
            let (st, end) = l.split_at(dash);
            ranges.push((st.to_vec(), end[1..].to_vec()));
        }

        ranges
    }

    fn is_invalid_part1(id: Vec<char>) -> bool {
        let n = id.len();
        if n % 2 == 1 {
            return false;
        }
        let mid = n / 2;
        for i in 0..mid {
            if id[i] != id[i + mid] {
                return false;
            }
        }
        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        let ranges = parse(reader);
        println!("{:?}", &ranges);
        for (st, end) in ranges {
            println!("{:?}", &st);
            println!("{:?}", &end);
            println!();
            let (st, end) = (
                st.into_iter().collect::<String>().parse::<usize>().unwrap(),
                end.into_iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            );
            for id in st..=end {
                if is_invalid_part1(format!("{}", id).chars().collect()) {
                    result += id;
                }
            }
        }
        Ok(result)
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample test input end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn is_invalid_part2(id: Vec<char>) -> bool {
        let n = id.len();
        let mid = n / 2;

        'outer: for seq_len in 1..=mid {
            // check if id has equal seq_len parts
            let mut st_window = 0;
            let mut end_window = seq_len;
            let mut candidate_id = None;

            while end_window <= n {
                if let Some(cand_id) = candidate_id {
                    if cand_id != &id[st_window..end_window] {
                        continue 'outer;
                    }
                } else {
                    candidate_id = Some(&id[st_window..end_window]);
                }

                st_window = end_window;
                end_window += seq_len;
            }

            // println!("st_window: {:?}", st_window);
            // println!("end_window: {:?}", end_window);
            // println!("candidate_id: {:?}", candidate_id);

            if st_window != n {
                continue;
            }

            return true;
        }

        false
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        let ranges = parse(reader);
        // println!("{:?}", &ranges);
        for (st, end) in ranges {
            // println!("start");
            // println!("{:?}", &st);
            // println!("{:?}", &end);
            // println!();

            let (st, end) = (
                st.into_iter().collect::<String>().parse::<usize>().unwrap(),
                end.into_iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            );

            for id in st..=end {
                if is_invalid_part2(format!("{}", id).chars().collect()) {
                    // println!("found one invalid id!");
                    // println!("invalid: {}", id);
                    result += id;
                }
            }
        }
        Ok(result)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
