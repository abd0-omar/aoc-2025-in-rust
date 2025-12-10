use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(Debug, Clone, PartialEq, Copy)]
    enum Light {
        On,
        Off,
    }

    fn parse<R: BufRead>(reader: R) -> (Vec<Vec<Light>>, Vec<Vec<Vec<usize>>>, Vec<Vec<usize>>) {
        // one row per machine
        //
        // indicator light
        // button wiring schematics
        // joltage requirements
        //
        // the machine is initially off
        //

        let mut light_indicators = Vec::new();
        let mut buttons = Vec::new();
        let mut joltage = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

            let mut cur_light_indicator = Vec::new();
            for light in parts[0].chars() {
                match light {
                    '#' => cur_light_indicator.push(Light::On),
                    '.' => cur_light_indicator.push(Light::Off),
                    _ => (),
                }
            }

            light_indicators.push(cur_light_indicator);

            let mut cur_buttons = Vec::new();
            let buttons_part = &parts[1..parts.len() - 1];

            for &consecuative_buttons in buttons_part {
                // remove brackets
                let consecuative_buttons = &consecuative_buttons[1..consecuative_buttons.len() - 1];

                let mut cur_consecuative_buttons = Vec::new();
                let button_parts = consecuative_buttons.split(',');
                for button_part in button_parts {
                    cur_consecuative_buttons.push(button_part.parse().unwrap());
                }

                cur_buttons.push(cur_consecuative_buttons);
            }

            buttons.push(cur_buttons);

            let mut cur_joltage = Vec::new();

            for jol in parts
                .last()
                .unwrap()
                .trim_matches(|ch| ch == '{' || ch == '}')
                .split(',')
            {
                cur_joltage.push(jol.parse().unwrap());
            }

            joltage.push(cur_joltage);
        }

        (light_indicators, buttons, joltage)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // ignore joltage requirements for part1
        //
        // fewest total presses to match the indicator light
        //
        // idk if there's a greedy solution
        // but a straightforward solution is backtrack
        //
        let mut result = 0;

        let (light_indicators, buttons, _) = parse(reader);

        for (mut cur_machine_light_indi, cur_machine_buttons) in
            light_indicators.into_iter().zip(buttons)
        {
            let mut cur_result = 999;
            backtrack_part1(
                &mut cur_machine_light_indi,
                &cur_machine_buttons,
                0,
                0,
                &mut cur_result,
            );
            result += cur_result
        }

        Ok(result)
    }

    fn backtrack_part1(
        light_indicators: &mut Vec<Light>,
        buttons: &Vec<Vec<usize>>,
        idx: usize,
        count: usize,
        cur_result: &mut usize,
    ) {
        if light_indicators.iter().all(|&light| light == Light::Off) {
            *cur_result = (*cur_result).min(count);
            return;
        }

        if idx == buttons.len() {
            return;
        }

        // pick
        for &button in buttons[idx].iter() {
            match light_indicators[button] {
                Light::On => light_indicators[button] = Light::Off,
                Light::Off => light_indicators[button] = Light::On,
            }
        }

        backtrack_part1(light_indicators, buttons, idx + 1, count + 1, cur_result);

        for &button in buttons[idx].iter() {
            match light_indicators[button] {
                Light::Off => light_indicators[button] = Light::On,
                Light::On => light_indicators[button] = Light::Off,
            }
        }

        // leave
        backtrack_part1(light_indicators, buttons, idx + 1, count, cur_result);
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 1 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    // TLE
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // ignore light indicators requirements for part2
        let mut result = 0;

        let (_, buttons, joltage) = parse(reader);

        for (mut cur_machine_joltage, cur_machine_buttons) in joltage.into_iter().zip(buttons) {
            let mut cur_result = usize::MAX;
            backtrack_part2(
                &mut cur_machine_joltage,
                &cur_machine_buttons,
                0,
                0,
                &mut cur_result,
            );
            result += cur_result
        }

        Ok(result)
    }

    fn backtrack_part2(
        joltage: &mut Vec<usize>,
        buttons: &Vec<Vec<usize>>,
        idx: usize,
        count: usize,
        cur_result: &mut usize,
    ) {
        if count >= *cur_result {
            return;
        }

        if joltage.iter().all(|&jol| jol == 0) {
            *cur_result = (*cur_result).min(count);
            return;
        }

        if idx == buttons.len() {
            return;
        }

        // atomic, if can decrease all voltage
        let can_press = { buttons[idx].iter().all(|&button| joltage[button] > 0) };

        if can_press {
            for &button in &buttons[idx] {
                joltage[button] -= 1;
            }
            backtrack_part2(joltage, buttons, idx, count + 1, cur_result);
            for &button in &buttons[idx] {
                joltage[button] += 1;
            }
        }

        // leave
        backtrack_part2(joltage, buttons, idx + 1, count, cur_result);
    }

    assert_eq!(33, part2(BufReader::new(TEST.as_bytes()))?);
    println!("=== Part 2 sample end ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
