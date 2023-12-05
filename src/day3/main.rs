use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("src/day3/input.txt")?;
    let schematic = Schematic::new(&input)?;
    println!("{:#?}", schematic);
    println!("{}", schematic.sum_of_part_numbers());
    println!("{}", schematic.sum_of_gear_ratios());
    Ok(())
}

/// Data types

#[derive(Debug)]
enum Part {
    Number(u32),
    Symbol {
        value: char,
        adjacent_numbers: Vec<u32>,
    },
}

#[derive(Debug)]
struct Schematic {
    input: String,
    parts: HashMap<(usize, usize), Part>,
    height: usize,
    width: usize,
}

/// Algorithms

impl Schematic {
    fn new(input: &str) -> Result<Schematic> {
        let mut schematic = Schematic {
            input: input.to_string(),
            parts: HashMap::new(),
            height: input.lines().count(),
            width: input
                .lines()
                .next()
                .map(|s| s.len())
                .ok_or(anyhow!("fail to read input"))?,
        };

        for y in 0..schematic.height {
            let mut x = 0;
            while x < schematic.width {
                let i = y * (schematic.width + 1) + x;
                x += 1;
                match schematic.input.chars().nth(i) {
                    Some('.') => continue,
                    Some(c) if char::is_ascii_punctuation(&c) => {
                        let loc = (x - 1, y);
                        if !schematic.parts.contains_key(&loc) {
                            schematic.parts.insert(
                                loc,
                                Part::Symbol {
                                    value: c,
                                    adjacent_numbers: vec![],
                                },
                            );
                        }
                    }
                    Some(c) if is_decimal(&c) => {
                        let num: String = schematic
                            .input
                            .chars()
                            .skip(i)
                            .take_while(is_decimal)
                            .collect();
                        let len = num.len();
                        let loc = (x - 1, y);

                        detect_adjacent_symbols(
                            &mut schematic.parts,
                            schematic.width,
                            schematic.height,
                            input,
                            loc,
                            len,
                            num.parse::<u32>()?,
                        );

                        x += len - 1;
                    }
                    _ => continue,
                }
            }
        }

        Ok(schematic)
    }

    fn sum_of_part_numbers(&self) -> u32 {
        self.parts
            .iter()
            .filter_map(|p| match p {
                (_, &Part::Number(value)) => Some(value),
                _ => None,
            })
            .sum()
    }

    fn sum_of_gear_ratios(&self) -> u32 {
        self.parts
            .iter()
            .filter_map(|p| match p {
                (
                    _,
                    &Part::Symbol {
                        value: _,
                        ref adjacent_numbers,
                    },
                ) => {
                    if adjacent_numbers.len() == 2 {
                        Some(adjacent_numbers[0] * adjacent_numbers[1])
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .sum()
    }
}

fn detect_adjacent_symbols(
    parts: &mut HashMap<(usize, usize), Part>,
    width: usize,
    height: usize,
    input: &str,
    loc: (usize, usize),
    len: usize,
    num: u32,
) {
    let (x, y) = loc;

    let y_range = if y == 0 {
        y..=y + 1
    } else if y == height - 1 {
        y - 1..=y
    } else {
        y - 1..=y + 1
    };

    let x_range = if x == 0 { x..=x + len } else { x - 1..=x + len };

    let mut has_adjacent_symbols = false;
    for x in x_range {
        for y in y_range.clone() {
            let i = y * (width + 1) + x;
            if let Some(c) = input.chars().nth(i) {
                if c != '.' && char::is_ascii_punctuation(&c) {
                    has_adjacent_symbols = true;
                    parts
                        .entry((x, y))
                        .and_modify(|p: &mut Part| {
                            if let Part::Symbol {
                                value: _,
                                ref mut adjacent_numbers,
                            } = p
                            {
                                adjacent_numbers.push(num)
                            }
                        })
                        .or_insert(Part::Symbol {
                            value: c,
                            adjacent_numbers: vec![num],
                        });
                }
            }
        }
    }

    if has_adjacent_symbols {
        parts.insert(loc, Part::Number(num));
    }
}

fn is_decimal(c: &char) -> bool {
    char::is_digit(*c, 10)
}
