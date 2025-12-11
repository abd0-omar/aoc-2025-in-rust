use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

const TEST_PART2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> HashMap<String, Vec<String>> {
        let mut cables = HashMap::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let (input, outputs) = line.split_once(':').unwrap();
            let output_parts = outputs
                .split_ascii_whitespace()
                .map(|output| output.to_string())
                .collect::<Vec<_>>();
            for output_part in output_parts {
                cables
                    .entry(input.to_string())
                    .or_insert(Vec::new())
                    .push(output_part);
            }
        }
        cables
    }

    fn dfs(cables: &HashMap<String, Vec<String>>, cur_input: &str, target: &str) -> usize {
        if cur_input == "out" {
            return 1;
        }

        let mut result = 0;
        for neighbor in &cables[cur_input] {
            result += dfs(cables, neighbor, target);
        }

        result
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // simple dfs
        let mut result = 0;

        let cables = parse(reader);

        result += dfs(&cables, "you", "out");

        Ok(result)
    }

    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn dfs_part2<'a>(
        cables: &'a &HashMap<String, Vec<String>>,
        cur_input: &'a str,
        mut fft: bool,
        mut dac: bool,
        memo: &mut HashMap<(&'a str, bool, bool), usize>,
    ) -> usize {
        if cur_input == "fft" {
            fft = true;
        }

        if cur_input == "dac" {
            dac = true;
        }

        if cur_input == "out" {
            return if dac && fft { 1 } else { 0 };
        }

        if let Some(&cached_result) = memo.get(&(cur_input, fft, dac)) {
            return cached_result;
        }

        let mut result = 0;
        if let Some(neighbors) = cables.get(cur_input) {
            for neighbor in neighbors {
                result += dfs_part2(cables, neighbor, fft, dac, memo);
            }
        }

        memo.insert((cur_input, fft, dac), result);

        result
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // simple dfs
        let mut result = 0;

        let cables = parse(reader);

        let mut memo = HashMap::new();

        result += dfs_part2(&&cables, "svr", false, false, &mut memo);

        Ok(result)
    }

    assert_eq!(2, part2(BufReader::new(TEST_PART2.as_bytes()))?);
    println!("=== Part 2 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
