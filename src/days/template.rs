#![allow(unused)]
use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 0,
    name: "Template",
    part_1: run_part1,
    part_2: None,
    other: &[("Parse", run_parse)],
};

type Data = u32;

pub fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(data)))
}

pub fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

pub fn parse(input: &str) -> Result<Data> {
    Ok(0)
}

pub fn part1(data: Data) -> u32 {
    let mut _data = data.to_owned();
    data as u32
}

#[cfg(test)]
mod day01_tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        assert_eq!(data.len(), 0);
    }
}
