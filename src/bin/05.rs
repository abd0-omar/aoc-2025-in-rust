use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> Result<(Vec<(usize, usize)>, Vec<usize>)> {
        let mut ranges = Vec::new();
        let mut ingredients = Vec::new();

        let mut lines_iterator = reader.lines().map(|l| l.unwrap());

        while let Some(line) = lines_iterator.next() {
            let (from, to) = match line.split_once("-") {
                Some(r) => r,
                None => break,
            };
            dbg!((from, to));

            ranges.push((from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap()));
        }

        while let Some(line) = lines_iterator.next() {
            let ingredient = line.parse::<usize>().unwrap();

            ingredients.push(ingredient);
        }

        dbg!(&ingredients);

        Ok((ranges, ingredients))
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        // ranges
        // and ranges can overlap
        // I think part2 would include something of merge intervals

        let (mut ranges, ingredients) = parse(reader)?;

        let mut intervals = vec![ranges[0]];

        // any merge intervals starts with sorting
        ranges.sort_unstable();

        // 3-5
        // 10-14
        // 12-18
        // 16-20

        // 3-5
        // 10-18
        // 16-20

        // 3-5
        // 10-20

        for &rang in ranges.iter().skip(1) {
            // merge if they touch or adjacent
            if intervals.last().unwrap().1 + 1 >= rang.0 {
                // there is an overlap
                let (st, end) = intervals.pop().unwrap();
                intervals.push((st, rang.1.max(end)));
                continue;
            }

            intervals.push(rang);
        }

        dbg!(&intervals);

        'outer: for ingredient in ingredients {
            for &inter in ranges.iter() {
                if (inter.0..=inter.1).contains(&ingredient) {
                    result += 1;
                    continue 'outer;
                }
            }
        }

        Ok(result)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample end ===");

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
    // println!("=== Part 2 sample end ===");
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
