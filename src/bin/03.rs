use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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
        for bank in reader.lines() {
            let bank = bank?;

            let mut min_heap = BinaryHeap::new();
            // overkill, but might need it. premature optimization is the root of all evil
            let mut idx_map = HashMap::new();

            for (idx, battery) in bank.chars().enumerate() {
                let battery = battery.to_digit(10).unwrap();

                if min_heap.len() < 2 {
                    min_heap.push(Reverse(battery));
                    // min_heap.push(BatteryOrder {
                    //     battery,
                    //     order: min_heap.len(),
                    // });
                    idx_map.insert(battery, idx);
                    continue;
                }

                if battery > min_heap.peek().unwrap().0 {
                    let Reverse(old_battery) = min_heap.pop().unwrap();
                    min_heap.push(Reverse(battery));

                    idx_map.remove(&old_battery);
                    idx_map.insert(battery, idx);
                }
            }

            dbg!(min_heap);
            dbg!(&idx_map);

            // reverse the map, like the way they do it in stakpak agent redaction secrets, but won't need it as a hashmap again
            let mut idx_battery = idx_map.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
            idx_battery.sort_unstable();
        }
        todo!()
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample test end ===");

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
    // println!("=== Part 2 sample test end ===");
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
