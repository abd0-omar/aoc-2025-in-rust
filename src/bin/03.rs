use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    // #[derive(Eq, PartialEq, PartialOrd)]
    // struct BatteryOrder {
    //     battery: u32,
    //     order: usize,
    // }

    // impl Ord for BatteryOrder {
    //     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    //         other
    //             .battery
    //             .cmp(&self.battery)
    //             .then(other.order.cmp(&self.order))
    //     }
    // }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        for bank in reader.lines() {
            let bank = bank?.into_bytes();
            let n = bank.len();
            let mut max_joltage = 0;

            // brute-force
            for i in 0..n {
                let digit1 = bank[i] - b'0';
                for j in i + 1..n {
                    let digit2 = bank[j] - b'0';

                    let cur_joltage = digit1 * 10 + digit2;
                    max_joltage = max_joltage.max(cur_joltage);
                }
            }

            result += max_joltage as usize;
        }

        Ok(result)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn backtrack(
        bank: &[u8],
        mut cur: Vec<u8>,
        result: &mut Vec<Vec<u8>>,
        mut visited: HashSet<usize>,
        st: usize,
    ) {
        if cur.len() == 12 {
            result.push(cur);
            return;
        }

        for end in st..bank.len() {
            if !visited.insert(end) {
                continue;
            }

            let battery = bank[end] - b'0';

            cur.push(battery);

            backtrack(bank, cur.clone(), result, visited.clone(), end + 1);

            cur.pop();
            visited.remove(&end);
        }
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // backtrack
        let mut result = 0;

        for bank in reader.lines() {
            let bank = bank?.into_bytes();

            let mut max_joltage = 0;
            let mut backtrack_results = Vec::new();

            backtrack(
                &bank,
                Vec::with_capacity(12),
                &mut backtrack_results,
                HashSet::new(),
                0,
            );

            for result in backtrack_results {
                let result = result
                    .into_iter()
                    .map(|x| char::from_digit(x as u32, 10).unwrap())
                    .collect::<String>()
                    .parse::<usize>()?;
                max_joltage = max_joltage.max(result);
            }

            dbg!(&max_joltage);
            result += max_joltage;
        }

        Ok(result as usize)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 2 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
