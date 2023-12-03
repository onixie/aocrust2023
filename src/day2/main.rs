use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, one_of, space0},
    combinator::{map, recognize},
    error::ParseError,
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

#[cfg(test)]
use nom_test_helpers::prelude::*;

use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let (_, gs) = parse(include_str!("input.txt")).unwrap();
    //println!("{:#?}", gs);

    let possible_gs = gs.iter().filter(|g| possible(g, 12, 13, 14));
    let ids = possible_gs.map(|g| g.id).collect::<Vec<_>>();
    //println!("{:?}", ids);

    let sum_of_ids = ids.iter().sum::<usize>();
    println!("{}", sum_of_ids);

    let min_possibles = gs.iter().map(min_possible).collect::<Vec<_>>();
    //println!("{:#?}", min_possibles);

    let sum_of_powers = min_possibles
        .iter()
        .map(|o| o.red.unwrap() * o.blue.unwrap() * o.green.unwrap())
        .sum::<usize>();
    println!("{}", sum_of_powers);
}

/// Type defs

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    observations: Vec<Observation>,
}

#[derive(Debug, PartialEq)]
struct Observation {
    red: Option<usize>,
    green: Option<usize>,
    blue: Option<usize>,
}

/// Algorithms

fn possible(game: &Game, red: usize, green: usize, blue: usize) -> bool {
    for observed in game.observations.iter() {
        if let Some(observed_red) = observed.red {
            if observed_red > red {
                return false;
            }
        }
        if let Some(observed_blue) = observed.blue {
            if observed_blue > blue {
                return false;
            }
        }
        if let Some(observed_green) = observed.green {
            if observed_green > green {
                return false;
            }
        }
    }
    return true;
}

fn min_possible(game: &Game) -> Observation {
    let reds = game
        .observations
        .iter()
        .map(|o| o.red)
        .collect::<Vec<Option<usize>>>();
    let blues = game
        .observations
        .iter()
        .map(|o| o.blue)
        .collect::<Vec<Option<usize>>>();
    let greens = game
        .observations
        .iter()
        .map(|o| o.green)
        .collect::<Vec<Option<usize>>>();

    Observation {
        red: max(&reds),
        blue: max(&blues),
        green: max(&greens),
    }
}

fn max(count: &[Option<usize>]) -> Option<usize> {
    count.into_iter().flatten().max().copied()
}

#[test]
fn test_max() {
    assert_eq!(max(&vec![Some(1)]), Some(1));
    assert_eq!(max(&vec![None]), None);
    assert_eq!(max(&vec![]), None);
    assert_eq!(max(&vec![Some(1), Some(2), None, Some(0)]), Some(2));
}

/// Input Parsers

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list0(newline, parse_game)(input)
}

#[test]
fn test_parse() {
    let game1 = "Game 1: 10 red, 3 green; 20 blue; 50 green";
    let game2 = "Game 2: 5 green, 20 red, 30 blue; 6 red, 7 green; 90 blue";
    let games = format!("{}\n{}", game1, game2);
    assert_eq!(
        parse(&games).unwrap().1,
        vec![parse_game(&game1).unwrap().1, parse_game(&game2).unwrap().1,]
    );
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(
        tag("Game "),
        terminated(parse_decimal::<usize>, ws(char(':'))),
    )(input)?;

    let (input, observations) = separated_list0(ws(char(';')), parse_observation)(input)?;
    Ok((input, Game { id, observations }))
}

#[test]
fn test_parse_game() {
    assert_done_and_eq!(
        parse_game("Game 1:"),
        Game {
            id: 1,
            observations: vec![]
        }
    );

    assert_done_and_eq!(
        parse_game("Game 1 : "),
        Game {
            id: 1,
            observations: vec![]
        }
    );

    assert_done_and_eq!(
        parse_game("Game 1 : 10 red"),
        Game {
            id: 1,
            observations: vec![Observation {
                red: Some(10),
                blue: None,
                green: None,
            }]
        }
    );

    assert_done_and_eq!(
        parse_game("Game 1 : 10 red ; 20 green ; 30 blue"),
        Game {
            id: 1,
            observations: vec![
                Observation {
                    red: Some(10),
                    blue: None,
                    green: None,
                },
                Observation {
                    red: None,
                    blue: None,
                    green: Some(20),
                },
                Observation {
                    red: None,
                    blue: Some(30),
                    green: None,
                }
            ]
        }
    );

    assert_done_and_eq!(
        parse_game("Game 1 : 10 red ; 20 green, 30 red, 5 blue; 30 blue"),
        Game {
            id: 1,
            observations: vec![
                Observation {
                    red: Some(10),
                    blue: None,
                    green: None,
                },
                Observation {
                    red: Some(30),
                    blue: Some(5),
                    green: Some(20),
                },
                Observation {
                    red: None,
                    blue: Some(30),
                    green: None,
                }
            ]
        }
    );
}

fn parse_observation(input: &str) -> IResult<&str, Observation> {
    let (input, counts) = separated_list1(
        ws(char(',')),
        alt((
            parse_cube::<usize>(ws(tag("red"))),
            parse_cube::<usize>(ws(tag("green"))),
            parse_cube::<usize>(ws(tag("blue"))),
        )),
    )(input)?;

    let mut o = Observation {
        blue: None,
        red: None,
        green: None,
    };

    for count in counts {
        match count {
            (count, "red") => o.red = Some(count),
            (count, "blue") => o.blue = Some(count),
            (count, "green") => o.green = Some(count),
            _ => panic!("impossible count"),
        }
    }

    Ok((input, o))
}

#[test]
fn test_parse_observation() {
    assert_done_and_eq!(
        parse_observation("10 red ,20 green, 30 blue"),
        Observation {
            red: Some(10),
            blue: Some(30),
            green: Some(20)
        }
    );

    assert_done_and_eq!(
        parse_observation("10 red, 20 green"),
        Observation {
            red: Some(10),
            blue: None,
            green: Some(20)
        }
    );

    assert_done_and_eq!(
        parse_observation("20 green"),
        Observation {
            red: None,
            blue: None,
            green: Some(20)
        }
    );

    assert_done_and_eq!(
        parse_observation("20 green,"),
        Observation {
            red: None,
            blue: None,
            green: Some(20)
        }
    );

    assert_done_and_eq!(
        parse_observation("20 green;"),
        Observation {
            red: None,
            blue: None,
            green: Some(20)
        }
    );
}

fn parse_cube<'a, T>(
    color_parser: impl FnMut(&'a str) -> IResult<&'a str, &'a str>,
) -> impl FnMut(&'a str) -> IResult<&'a str, (T, &'a str)>
where
    T: FromStr,
    T::Err: Debug,
{
    pair(parse_decimal::<T>, color_parser)
}

#[test]
fn test_parse_cube() {
    assert_done_and_eq!(parse_cube::<u32>(ws(tag("red")))("10 red"), (10, "red"));
    assert_done_and_eq!(parse_cube::<u32>(ws(tag("red")))("20 red "), (20, "red"));
    assert_done_and_eq!(parse_cube::<u32>(ws(tag("red")))("30 red;"), (30, "red"));
    assert_done_and_eq!(parse_cube::<u32>(ws(tag("red")))("40 red,"), (40, "red"));
}

fn parse_decimal<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
    T::Err: Debug,
{
    map(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.replace("_", "").parse::<T>().unwrap(),
    )(input)
}

#[test]
fn test_parse_decimal() {
    assert_done_and_eq!(parse_decimal::<u32>("0"), 0);
    assert_done_and_eq!(parse_decimal::<u32>("1"), 1);
    assert_done_and_eq!(parse_decimal::<u32>("10"), 10);
    assert_done_and_eq!(parse_decimal::<u32>("1_0"), 10);
    assert_done_and_eq!(parse_decimal::<u32>("1_0_"), 10);
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(space0, inner, space0)
}
