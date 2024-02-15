use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 11,
    name: "",
    part_1: run_part1,
    part_2: None,
    other: &[("Parse", run_parse)],
};

type Data = Map;

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}
#[derive(Debug, Clone)]
pub struct Map(Vec<Vec<Option<()>>>);

impl From<Vec<Vec<Option<()>>>> for Map {
    fn from(value: Vec<Vec<Option<()>>>) -> Self {
        Self(value)
    }
}

fn parse(input: &str) -> Result<Data> {
    // for each char in each line, if its a ̀`#`, put Some(()) else None
    Ok(input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => None,
                    '#' => Some(()),
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect::<Vec<Vec<Option<()>>>>()
        .into())
}

fn part1(input: &Data) -> u32 {
    let mut data = input.to_owned();
    // check rows if empty
    for line in data 
    0
}

#[cfg(test)]
mod day11_tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 0)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 374;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }
}
