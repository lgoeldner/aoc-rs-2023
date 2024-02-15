use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 11,
    name: "Cosmic Expansion",
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
use colored::Colorize;
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for cell in row {
                match cell {
                    Some(_) => write!(f, "{}", "#".red())?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

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

use std::{cell::RefCell, path::Display, rc::Rc, sync::Mutex};
fn part1(input: &Data) -> u32 {
    let mut data = input.to_owned();
    // this vector holds the amount of empty space in a column
    // if its 10, the column is empty
    let colums = RefCell::new(vec![0; input.0[0].len()]);
    let mut rows = Vec::with_capacity(input.0.len());

    // check rows if empty, expand if it is
    for (i, line) in data.0.iter().enumerate() {
        println!();
        let x = line
            .iter()
            .enumerate()
            .map(|(j, x)| {
                let cell_is_empty = x.is_none();
                // if the cell is empty, increase the amount of empty space in the column j
                if cell_is_empty {
                    colums.borrow_mut()[j] += 1;
                }
                // print!("{i} {j} empty: {cell_is_empty}  ");
                cell_is_empty
            })
            // collect so the column computation works
            .collect::<Vec<_>>()
            .iter()
            .all(|x| *x);

        rows.push(x);
    }

    // map the rows and colums to vecs of the indices where empty space needs to be inserted
    let colums_to_insert = colums
        .into_inner()
        .iter()
        .enumerate()
        .filter_map(|(i, empty_space)| if *empty_space == 10 { Some(i) } else { None })
        .collect::<Vec<_>>();

    let rows_to_insert = rows
        .iter()
        .enumerate()
        .filter_map(|(i, bool)| if *bool { Some(i) } else { None })
        .collect::<Vec<_>>();

    dbg!(&rows_to_insert);
    dbg!(&colums_to_insert);

    let empty_column = vec!['.'; data.0.len()];

    for (already_inserted, index) in colums_to_insert.iter().enumerate() {
        for line in &mut data.0 {
            line.insert(*index + already_inserted, None);
        }
    }

    let empty_row = vec![None; data.0[0].len()];

    for (already_inserted, index) in rows_to_insert.iter().enumerate() {
        data.0.insert(*index + already_inserted, empty_row.clone());
    }
    println!("{data}");
    0
}

#[cfg(test)]
mod day11_tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn day11() {
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
