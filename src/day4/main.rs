use anyhow::Result;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> Result<()> {
    let file = File::open("src/day4/input.txt")?;
    let mut table = String::new();
    let _ = BufReader::new(file).read_to_string(&mut table);
    println!("{}", table);

    let mut total_points: usize = 0;
    let mut number_of_matchings = vec![];
    for card in table.lines() {
        let mut card = card.split(":");
        let header = card.next();
        println!("{:?}", header);
        let numbers = card.next().map(|ns| ns.split("|"));
        if let Some((Some(winning_numbers), Some(numbers_you_have))) =
            numbers.map(|mut ns| (ns.next(), ns.next()))
        {
            let winning_numbers = winning_numbers
                .split(" ")
                .filter_map(|n| n.parse::<u32>().ok())
                .collect::<HashSet<_>>();

            let numbers_you_have = numbers_you_have
                .split(" ")
                .filter_map(|n| n.parse::<u32>().ok())
                .collect::<HashSet<_>>();

            println!("{:?} {:?}", winning_numbers, numbers_you_have);

            let point = winning_numbers.intersection(&numbers_you_have).count();
            if point > 0 {
                total_points += 2_usize.pow(point as u32 - 1);
            }
            println!("{}", point);

            number_of_matchings.push(point);
        }
    }

    println!("{}", total_points);

    println!("{:?}", number_of_matchings);

    let rounds = number_of_matchings.len();
    let mut number_of_cards = vec![1_usize; rounds];
    for (i, n) in number_of_matchings.iter().enumerate() {
        let copy = *number_of_cards.iter().nth(i).expect("rounds not matching");
        for c in &mut number_of_cards[i + 1..(i + 1 + n).min(rounds)] {
            *c += copy;
        }
    }
    println!("{:?}", number_of_cards);
    println!("{}", number_of_cards.iter().sum::<usize>());

    Ok(())
}
