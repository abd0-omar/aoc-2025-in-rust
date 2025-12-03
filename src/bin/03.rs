use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
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

    fn dp(
        bank: &[u8],
        idx: usize,
        amount: usize,
        memo: &mut HashMap<(usize, usize), String>,
    ) -> String {
        if amount == 0 {
            return String::new();
        }

        if idx == bank.len() {
            return String::new();
        }

        if bank.len() - idx < amount {
            return String::new();
        }

        if let Some(ret) = memo.get(&(idx, amount)) {
            return ret.to_string();
        }

        let mut pick = String::new();
        pick.push(bank[idx] as char);
        pick.push_str(&dp(bank, idx + 1, amount - 1, memo));

        let leave = dp(bank, idx + 1, amount, memo);

        if pick > leave {
            memo.insert((idx, amount), pick.clone());
            pick
        } else {
            memo.insert((idx, amount), leave.clone());
            leave
        }
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // dp pick leave
        let mut result = 0;

        for bank in reader.lines() {
            let bank = bank?.into_bytes();

            let mut memo = HashMap::new();
            let dp = dp(&bank, 0, 12, &mut memo);

            result += dp.parse::<usize>().unwrap();
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
