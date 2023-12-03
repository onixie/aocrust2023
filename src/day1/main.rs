use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file = File::open("src/day1/input.txt").unwrap();
    let mut input = String::new();
    let _ = BufReader::new(file).read_to_string(&mut input);

    let normalized_input = normalize_input(&input);
    // println!("{}", input);
    // println!("{}", normalized_input);
    println!("{}", sum_of_calibration_value(&input));
    println!("{}", sum_of_calibration_value(&normalized_input));
}

fn normalize_input(input: &str) -> String {
    let pat = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut input = input;
    let mut result = String::new();
    let mut matched = false;

    while !input.is_empty() {
        for (i, p) in pat.iter().enumerate() {
            if input.starts_with(p) {
                result.push(char::from_digit((i + 1) as u32, 10).unwrap());
                matched = true;
            }
        }
        if !matched {
            result += &input[0..1];
        }
        input = &input[1..];
        matched = false;
    }

    result
}

fn sum_of_calibration_value(input: &str) -> u32 {
    let mut first_digit = 0;
    let mut last_digit: Option<u8> = None;
    let mut sum: u32 = 0;
    for c in input.chars() {
        match c {
            '\n' => {
                if let Some(last_digit) = last_digit {
                    sum += (first_digit * 10 + last_digit) as u32;
                }
                last_digit = None;
            }
            _ if c.is_digit(10) => {
                let d = c as u8 - '0' as u8;
                if last_digit == None {
                    first_digit = d;
                }
                last_digit = Some(d);
            }
            _ => {}
        }
    }
    sum
}
