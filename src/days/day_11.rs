use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};
use itertools::Itertools;
use std::cell::RefCell;

pub const DAY: Day = Day {
    day: 11,
    name: "Cosmic Expansion",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

type Data = Map;

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2::part2(&data, 1_000_000 - 1)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn part1(input: &Data) -> u64 {
    let mut data = input.to_owned();
    // this vector holds the amount of empty space in a column
    // if its 10, the column is empty

    // check which rows are empty
    let (rows, columns) = get_empty(&data);

    // map the rows and colums to vecs of the indices where empty space needs to be inserted
    let (colums_to_insert, rows_to_insert) =
        empty_space_to_indices(&columns, &rows, data.0.len() as u32);

    insert_columns(colums_to_insert, &mut data);

    insert_rows(rows_to_insert, &mut data);

    let mut star_positions: Vec<Star<_>> = vec![];
    for (y, line) in data.0.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            match cell {
                Some(_) => star_positions.push(Star::new((x as u64, y as u64))),
                None => (),
            }
        }
    }

    // for each unique star combination, find the closest star
    let distance: u64 = star_positions
        .iter()
        .combinations(2)
        .unique()
        .map(|vec| {
            let a = vec[0];
            let b = vec[1];
            a.taxicab_distance(*b)
        })
        .sum();

    //eprintln!("{data}");
    distance
}

fn insert_columns(colums_to_insert: Vec<usize>, data: &mut Map) {
    for (already_inserted, index) in colums_to_insert.iter().enumerate() {
        for line in &mut data.0 {
            line.insert(*index + already_inserted, Option::None);
        }
    }
}

fn insert_rows(rows_to_insert: Vec<usize>, data: &mut Map) {
    let empty_row = vec![None; data.0[0].len()];
    for (already_inserted, index) in rows_to_insert.iter().enumerate() {
        data.0.insert(*index + already_inserted, empty_row.clone());
    }
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
                    None => write!(f, "{}", ".".bright_blue())?,
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
    if input.is_empty() {
        Err(Report::msg("No input given"))?;
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Star<N> {
    pub x: N,
    pub y: N,
}
impl<N> Star<N> {
    pub fn new(point: (N, N)) -> Self {
        Self {
            x: point.0,
            y: point.1,
        }
    }
}
impl Star<u64> {
    pub fn taxicab_distance(self, other: Self) -> u64 {
        // cast to i128 to prevent overflow and then back to u64 after using abs()
        (self.x as i128 - other.x as i128).abs() as u64
            + (self.y as i128 - other.y as i128).abs() as u64
    }
}

fn empty_space_to_indices(
    colums: &Vec<u32>,
    rows: &Vec<bool>,
    row_count: u32,
) -> (Vec<usize>, Vec<usize>) {
    let colums_to_insert = colums
        .iter()
        .enumerate()
        .filter_map(|(i, empty_space)| {
            if *empty_space >= row_count {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let rows_to_insert = rows
        .iter()
        .enumerate()
        .filter_map(|(i, bool)| if *bool { Some(i) } else { None })
        .collect::<Vec<_>>();

    (colums_to_insert, rows_to_insert)
}

fn get_empty(data: &Map) -> (Vec<bool>, Vec<u32>) {
    let colums = RefCell::new(vec![0; data.0[0].len()]);
    let mut rows = Vec::with_capacity(data.0.len());
    for (_, line) in data.0.iter().enumerate() {
        let x = line
            .iter()
            .enumerate()
            .map(|(j, x)| {
                let cell_is_empty = x.is_none();
                // if the cell is empty, increase the amount of empty space in the column j
                if cell_is_empty {
                    colums.borrow_mut()[j] += 1;
                }
                cell_is_empty
            })
            // collect so the column computation works
            .collect::<Vec<_>>()
            .iter()
            .all(|x| *x);

        rows.push(x);
    }
    (rows, colums.into_inner())
}

#[cfg(test)]
mod day11_tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn day11_1() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 0)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 374;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day11_2() {
        const EXPAND_BY: u64 = 1;

        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 0)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        {
            let expected = 374;
            let actual = part2(&parsed, EXPAND_BY);
            assert_eq!(expected, actual);
        }
        {
            let expected = 1030;
            let actual = part2(&parsed, 10 - 1);
            assert_eq!(expected, actual);
        }
		{
            let expected = 8410;
            let actual = part2(&parsed, 100 - 1);
            assert_eq!(expected, actual);
        }
    }
}
mod part2 {

    use super::*;


    pub fn part2(input: &Data, expansion_factor: u64) -> u64 {
        let mut data = input.to_owned();
        // this vector holds the amount of empty space in a column
        // if its 10, the column is empty

        // check which rows are empty
        let (rows, columns) = get_empty(&data);

        // convert columns to vec<bool>
        let column_count = data.0[0].len();

        let columns = columns
            .iter()
            .map(|x| x == &(column_count as u32))
            .collect::<Vec<_>>();

        let star_positions = convert_map(rows, columns, &mut data, expansion_factor);

        // for each unique star combination, find the closest star
        let distance: u64 = star_positions
            .iter()
            .combinations(2)
            .unique()
            .map(|vec| {
                let a = vec[0];
                let b = vec[1];
                a.taxicab_distance(*b)
            })
            .sum();

        distance
    }
    #[derive(Debug)]
    enum MapRow {
        Full(Vec<StarCell>),
        Empty,
    }
    #[derive(Debug)]
    enum StarCell {
        Star { x: u64, y: u64 },
        EmptyColumn,
        EmptySpace,
    }

    fn convert_map(
        empty_rows: Vec<bool>,
        empty_colums: Vec<bool>,
        data: &mut Map,
        expansion_factor: u64,
    ) -> Vec<Star<u64>> {
        let mut result = vec![];
        for ((y, row), row_is_empty) in data.0.iter_mut().enumerate().zip(empty_rows) {
            if row_is_empty {
                result.push(MapRow::Empty);
            }
            // else
            let mut temp = vec![];

            for ((x, cell), column_is_empty) in row.iter_mut().enumerate().zip(empty_colums.clone())
            {
                if !column_is_empty && cell.is_some() {
                    temp.push(StarCell::Star {
                        x: x as u64,
                        y: y as u64,
                    });
                } else if column_is_empty {
                    temp.push(StarCell::EmptyColumn)
                } else {
                    temp.push(StarCell::EmptySpace);
                    assert!(cell.is_none());
                }
            }

            result.push(MapRow::Full(temp));
        }
        to_star_positions(result, expansion_factor)
    }

    /// convert the Map, that's not a Vec<MapRow>
    fn to_star_positions(result: Vec<MapRow>, expansion_factor: u64) -> Vec<Star<u64>> {
        let mut stars = vec![];
        // walk the map to convert the star positions
        let mut y_offset = 0;
        for row in result {
            match row {
                MapRow::Empty => {
                    y_offset += 1;
                }
                MapRow::Full(row) => {
                    let mut x_offset = 0;
                    for cell in row {
                        match cell {
                            StarCell::EmptyColumn => {
                                x_offset += 1
                            }
                            StarCell::EmptySpace => (),
                            StarCell::Star { x, y } => {
                                stars.push(Star::new((
                                    x + x_offset * expansion_factor,
                                    y + y_offset * expansion_factor,
                                )))
                            }
                        }
                    }
                }
            }
        }

        stars
    }
}
