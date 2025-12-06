use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> Result<(Vec<Vec<usize>>, Vec<char>)> {
        let mut total_numbers = Vec::new();
        let mut ops = Vec::new();

        for line in reader.lines().map(|l| l.unwrap()) {
            let mut cur_numbers = Vec::new();
            for number_or_op in line.split_ascii_whitespace() {
                match number_or_op.parse::<usize>() {
                    Ok(number) => {
                        cur_numbers.push(number);
                    }
                    Err(_) => {
                        // wow you can parse to char
                        let op = number_or_op.parse::<char>()?;
                        ops.push(op);
                    }
                };
            }

            if cur_numbers.len() > 0 {
                total_numbers.push(cur_numbers);
            }
        }

        Ok((total_numbers, ops))
    }

    fn parse_part2<R: BufRead>(reader: R) -> Result<(Vec<Vec<char>>, Vec<char>)> {
        let mut total_numbers = Vec::new();
        let mut ops = Vec::new();

        for line in reader.lines().map(|l| l.unwrap()) {
            if line.chars().any(|ch| ch == '+' || ch == '*') {
                for op in line.split_ascii_whitespace() {
                    ops.push(op.parse().unwrap());
                }
            } else {
                total_numbers.push(line.chars().collect());
            }
        }

        Ok((total_numbers, ops))
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        let (numbers, ops) = parse(reader)?;

        dbg!(&numbers);
        dbg!(&ops);

        let mut ops = ops.into_iter();

        let (n, m) = (numbers.len(), numbers[0].len());

        // move by cols not by rows
        for j in 0..m {
            let mut cur_stack = Vec::with_capacity(m);

            for i in 0..n {
                let number = numbers[i][j];
                dbg!(&number);

                cur_stack.push(number);
            }

            let cur_result: usize = match ops.next().unwrap() {
                '*' => cur_stack.iter().fold(1, |acc, x| acc * x),
                '+' => cur_stack.iter().sum(),
                _ => unreachable!(),
            };

            result += cur_result;
        }

        Ok(result)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (numbers, ops) = parse_part2(reader)?;

        let mut ops = ops.into_iter();
        let mut cur_op = ops.next().unwrap();

        // dbg!(&numbers);
        // dbg!(&ops);

        let mut result = 0;

        let (n, m) = (numbers.len(), numbers[0].len());

        let mut big_cur_result = Vec::new();

        // move by cols not by rows
        for j in 0..m {
            let mut cur_stack = Vec::with_capacity(m);

            for i in 0..n {
                let number = numbers[i][j];

                if !number.is_ascii_whitespace() {
                    cur_stack.push(number);
                }

                // dbg!(&number);

                // cur_stack.push(number);
            }

            if cur_stack.is_empty() {
                println!("empty");

                let cur_result: usize = match cur_op {
                    '*' => big_cur_result
                        .iter()
                        .map(|x: &String| x.parse::<usize>().unwrap())
                        .product(),
                    '+' => big_cur_result
                        .iter()
                        .map(|x: &String| x.parse::<usize>().unwrap())
                        .sum(),
                    _ => unreachable!(),
                };

                dbg!(&cur_result);

                big_cur_result.clear();

                cur_op = ops.next().unwrap();

                result += cur_result;

                continue;
            }

            // dbg!(&cur_stack);

            big_cur_result.push(cur_stack.iter().collect::<String>());

            // dbg!(&cur_result);

            // result += cur_result;
        }

        let cur_result: usize = match cur_op {
            '*' => big_cur_result
                .iter()
                .map(|x: &String| x.parse::<usize>().unwrap())
                .product(),
            '+' => big_cur_result
                .iter()
                .map(|x: &String| x.parse::<usize>().unwrap())
                .sum(),
            _ => unreachable!(),
        };

        dbg!(&cur_result);

        result += cur_result;

        Ok(result)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 2 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
